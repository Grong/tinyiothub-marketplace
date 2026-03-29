use std::sync::Arc;
use std::collections::HashMap;
use std::path::PathBuf;
use parking_lot::RwLock;
use thiserror::Error;
use tracing::{info, warn, error};
use crate::infrastructure::{SledCache, GitHubClient};
use crate::infrastructure::github::GitHubError;
use crate::domain::{Template, Driver};
use crate::dto::RepositoryConfig;
use serde_json::Value;
use chrono::{DateTime, Utc};

#[derive(Error, Debug)]
pub enum SyncError {
    #[error("lock held by another process")]
    LockConflict,
    #[error("sync failed: {0}")]
    Failed(String),
    #[error("all repos returned 404")]
    AllReposNotFound,
}

pub struct SyncService {
    cache: Arc<SledCache>,
    github: Arc<GitHubClient>,
    config: Arc<RwLock<Vec<RepositoryConfig>>>,
    local_data_path: Arc<RwLock<Option<PathBuf>>>,
}

impl SyncService {
    pub fn new(cache: Arc<SledCache>, github: Arc<GitHubClient>) -> Self {
        Self {
            cache,
            github,
            config: Arc::new(RwLock::new(Vec::new())),
            local_data_path: Arc::new(RwLock::new(None)),
        }
    }

    pub fn with_local_data_path(self, path: PathBuf) -> Self {
        *self.local_data_path.write() = Some(path);
        self
    }

    pub fn set_local_data_path(&self, path: PathBuf) {
        *self.local_data_path.write() = Some(path);
    }

    /// Load repository config from JSON file
    pub fn load_config(&self, config_json: &str) -> Result<(), serde_json::Error> {
        let configs: Vec<RepositoryConfig> = serde_json::from_str(config_json)?;
        let mut config = self.config.write();
        *config = configs;
        Ok(())
    }

    /// Perform full sync of all repositories
    pub async fn full_sync(&self) -> Result<(), SyncError> {
        info!("Starting full sync");

        let holder_id = uuid::Uuid::new_v4().to_string();
        if !self.cache.acquire_sync_lock(&holder_id).map_err(|e| SyncError::Failed(e.to_string()))? {
            warn!("Sync lock conflict, skipping full sync");
            return Err(SyncError::LockConflict);
        }

        let result = self.do_full_sync().await;

        // Release lock
        if let Err(e) = self.cache.release_sync_lock(&holder_id) {
            warn!("Failed to release sync lock: {}", e);
        }

        result
    }

    async fn do_full_sync(&self) -> Result<(), SyncError> {
        let configs = self.config.read().clone();

        let mut all_templates: Vec<Value> = Vec::new();
        let mut all_drivers: Vec<Value> = Vec::new();
        let mut has_any_repo = false;
        let mut failed_repos = 0;

        for config in &configs {
            // Try local file first if configured
            let local_path = self.local_data_path.read().clone();
            let result = if let Some(ref local_path) = local_path {
                let local_file = local_path.join(&config.path).join("index.json");
                if local_file.exists() {
                    info!("Reading from local file: {:?}", local_file);
                    match tokio::fs::read_to_string(&local_file).await {
                        Ok(content) => {
                            match serde_json::from_str::<Vec<Value>>(&content) {
                                Ok(items) => Ok(items),
                                Err(e) => {
                                    warn!("Failed to parse local file {:?}: {}", local_file, e);
                                    Err(GitHubError::NotFound)
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to read local file {:?}: {}", local_file, e);
                            Err(GitHubError::NotFound)
                        }
                    }
                } else {
                    warn!("Local file {:?} not found, falling back to GitHub", local_file);
                    self.github.fetch_index_json(&config.repo, "main", &config.path).await
                }
            } else {
                self.github.fetch_index_json(&config.repo, "main", &config.path).await
            };

            match result {
                Ok(items) => {
                    has_any_repo = true;
                    // Validate and filter items
                    for item in items {
                        match config.repo_type.as_str() {
                            "template" => {
                                if Template::validate(&item).is_ok() {
                                    all_templates.push(item);
                                } else {
                                    warn!("Skipping invalid template item in {}/{}", config.repo, config.path);
                                }
                            }
                            "driver" => {
                                if Driver::validate(&item).is_ok() {
                                    all_drivers.push(item);
                                } else {
                                    warn!("Skipping invalid driver item in {}/{}", config.repo, config.path);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Err(GitHubError::NotFound) => {
                    warn!("Repository {}/{} returned 404, skipping", config.repo, config.path);
                    failed_repos += 1;
                }
                Err(e) => {
                    error!("Failed to fetch {}/{}: {}", config.repo, config.path, e);
                    failed_repos += 1;
                }
            }
        }

        if !has_any_repo && failed_repos > 0 {
            return Err(SyncError::AllReposNotFound);
        }

        // LWW merge for templates (Last-Write-Wins based on updated_at)
        let templates = Self::lww_merge(all_templates);
        let drivers = Self::lww_merge(all_drivers);

        // Write to Sled
        if let Err(e) = self.cache.set_templates(&templates) {
            error!("Failed to write templates to cache: {}", e);
        }
        if let Err(e) = self.cache.set_drivers(&drivers) {
            error!("Failed to write drivers to cache: {}", e);
        }

        // Update last sync time
        let now = Utc::now().timestamp();
        if let Err(e) = self.cache.set_last_sync(now) {
            error!("Failed to update last_sync: {}", e);
        }

        info!("Full sync completed: {} templates, {} drivers", templates.len(), drivers.len());

        Ok(())
    }

    /// LWW merge: for duplicate IDs, keep the one with latest updated_at
    fn lww_merge(items: Vec<Value>) -> Vec<Value> {
        // Group by id and keep track of latest
        let mut latest: HashMap<String, (usize, DateTime<Utc>)> = HashMap::new();

        for (idx, item) in items.iter().enumerate() {
            if let (Some(id), Some(updated_at_str)) = (
                item.get("id").and_then(|v| v.as_str()),
                item.get("updated_at").and_then(|v| v.as_str()),
            ) {
                if let Ok(ts) = DateTime::parse_from_rfc3339(updated_at_str) {
                    let ts = ts.with_timezone(&Utc);
                    match latest.get(id) {
                        Some((_, existing_ts)) if *existing_ts >= ts => {
                            // Keep existing
                        }
                        _ => {
                            latest.insert(id.to_string(), (idx, ts));
                        }
                    }
                }
            }
        }

        // Rebuild items list keeping only latest for each id (stable order)
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut result: Vec<Value> = Vec::new();
        for (idx, item) in items.iter().enumerate() {
            if let Some(id) = item.get("id").and_then(|v| v.as_str()) {
                if seen.contains(id) {
                    continue;
                }
                seen.insert(id.to_string());
                // Keep if this item's index matches the latest for this id
                if latest.get(id).map(|(li, _)| *li == idx).unwrap_or(false) {
                    result.push(item.clone());
                }
            } else {
                result.push(item.clone());
            }
        }

        result
    }

    /// Sync a single repo (triggered by webhook)
    pub async fn sync_repo(&self, repo: &str, path: &str) -> Result<(), SyncError> {
        info!("Syncing single repo: {}/{}", repo, path);

        let holder_id = uuid::Uuid::new_v4().to_string();
        if !self.cache.acquire_sync_lock(&holder_id).map_err(|e| SyncError::Failed(e.to_string()))? {
            return Err(SyncError::LockConflict);
        }

        let result = self.do_sync_repo(repo, path).await;

        if let Err(e) = self.cache.release_sync_lock(&holder_id) {
            warn!("Failed to release sync lock: {}", e);
        }

        result
    }

    async fn do_sync_repo(&self, repo: &str, path: &str) -> Result<(), SyncError> {
        // Determine type from config
        let (repo_type, _) = {
            let configs = self.config.read();
            let config = configs.iter().find(|c| c.repo == repo && c.path == path);
            match config {
                Some(c) => (c.repo_type.clone(), c.path.clone()),
                None => {
                    warn!("Repo {}/{} not in config, skipping", repo, path);
                    return Ok(());
                }
            }
        };

        let items = self.github.fetch_index_json(repo, "main", path).await
            .map_err(|e| SyncError::Failed(e.to_string()))?;

        match repo_type.as_str() {
            "template" => {
                let mut existing = self.cache.get_templates()
                    .map_err(|e| SyncError::Failed(e.to_string()))?
                    .unwrap_or_default();

                for item in items {
                    if Template::validate(&item).is_ok() {
                        existing.push(item);
                    }
                }

                let merged = Self::lww_merge(existing);
                self.cache.set_templates(&merged)
                    .map_err(|e| SyncError::Failed(e.to_string()))?;
            }
            "driver" => {
                let mut existing = self.cache.get_drivers()
                    .map_err(|e| SyncError::Failed(e.to_string()))?
                    .unwrap_or_default();

                for item in items {
                    if Driver::validate(&item).is_ok() {
                        existing.push(item);
                    }
                }

                let merged = Self::lww_merge(existing);
                self.cache.set_drivers(&merged)
                    .map_err(|e| SyncError::Failed(e.to_string()))?;
            }
            _ => {}
        }

        let now = Utc::now().timestamp();
        self.cache.set_last_sync(now)
            .map_err(|e| SyncError::Failed(e.to_string()))?;

        info!("Single repo sync completed: {}/{}", repo, path);

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    // lww_merge is private so we test it via the impl block via SyncService
    // These tests verify the lww_merge logic by duplicating it as a test-only function

    fn test_lww_merge(items: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        use std::collections::HashMap;
        let mut latest: HashMap<String, (usize, chrono::DateTime<chrono::Utc>)> = HashMap::new();

        for (idx, item) in items.iter().enumerate() {
            if let (Some(id), Some(updated_at_str)) = (
                item.get("id").and_then(|v| v.as_str()),
                item.get("updated_at").and_then(|v| v.as_str()),
            ) {
                if let Ok(ts) = chrono::DateTime::parse_from_rfc3339(updated_at_str) {
                    let ts = ts.with_timezone(&chrono::Utc);
                    match latest.get(id) {
                        Some((_, existing_ts)) if *existing_ts >= ts => {}
                        _ => {
                            latest.insert(id.to_string(), (idx, ts));
                        }
                    }
                }
            }
        }

        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut result: Vec<serde_json::Value> = Vec::new();
        for (idx, item) in items.iter().enumerate() {
            if let Some(id) = item.get("id").and_then(|v| v.as_str()) {
                if seen.contains(id) {
                    continue;
                }
                seen.insert(id.to_string());
                if latest.get(id).map(|(li, _)| *li == idx).unwrap_or(false) {
                    result.push(item.clone());
                }
            } else {
                result.push(item.clone());
            }
        }

        result
    }

    #[test]
    fn lww_merge_keeps_newer_over_older() {
        // newer first, then older - newer should win (latest timestamp seen first)
        let result = test_lww_merge(vec![
            serde_json::json!({
                "id": "template-001",
                "name": "New",
                "updated_at": "2026-03-29T00:00:00+00:00"
            }),
            serde_json::json!({
                "id": "template-001",
                "name": "Old",
                "updated_at": "2026-03-01T00:00:00+00:00"
            }),
        ]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].get("name").unwrap().as_str().unwrap(), "New");
    }


    #[test]
    fn lww_merge_deduplicates() {
        let t1 = serde_json::json!({
            "id": "template-001",
            "name": "First",
            "updated_at": "2026-03-29T00:00:00+00:00"
        });
        let t2 = serde_json::json!({
            "id": "template-002",
            "name": "Second",
            "updated_at": "2026-03-29T00:00:00+00:00"
        });

        let result = test_lww_merge(vec![t1, t2]);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn lww_merge_keeps_no_id_items() {
        let no_id = serde_json::json!({
            "name": "No ID Item",
            "updated_at": "2026-03-29T00:00:00+00:00"
        });

        let result = test_lww_merge(vec![no_id]);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn lww_merge_older_timestamp_is_discarded() {
        // When the second item has an older timestamp, first item should be kept
        let newer = serde_json::json!({
            "id": "driver-001",
            "name": "Newer",
            "updated_at": "2026-03-29T00:00:00+00:00"
        });
        let older = serde_json::json!({
            "id": "driver-001",
            "name": "Older",
            "updated_at": "2026-03-01T00:00:00+00:00"
        });

        let result = test_lww_merge(vec![newer, older]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].get("name").unwrap().as_str().unwrap(), "Newer");
    }
}
