mod api;
mod domain;
mod dto;
mod infrastructure;
mod sync;

use std::sync::Arc;
use axum::Router;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::info;
use infrastructure::cache::SledCache;
use infrastructure::github::GitHubClient;
use sync::SyncService;

#[derive(Clone)]
pub struct AppState {
    pub cache: Arc<SledCache>,
    pub github: Arc<GitHubClient>,
    pub sync: Arc<SyncService>,
}

// Helper to build the router with state - returns Router<()> so into_make_service works
fn build_app(state: AppState) -> Router {
    Router::new()
        .merge(api::health::routes())
        .nest("/api/v1", api::v1::routes())
        .nest_service("/static", ServeDir::new("web/dist"))
        .with_state(state)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let github_token = std::env::var("GITHUB_TOKEN")
        .unwrap_or_else(|_| "dummy".to_string());
    let _webhook_secret = std::env::var("GITHUB_WEBHOOK_SECRET")
        .unwrap_or_else(|_| "dummy".to_string());

    let cache = Arc::new(SledCache::new(
        std::env::var("SLED_PATH").unwrap_or_else(|_| "/tmp/marketplace.sled".into()),
    )?);

    let github = Arc::new(GitHubClient::new(github_token));

    let sync_service = Arc::new(SyncService::new(
        Arc::clone(&cache),
        Arc::clone(&github),
    ));

    // Set local data path if configured (for local development without GitHub)
    if let Ok(local_data_path) = std::env::var("LOCAL_DATA_PATH") {
        let sync_service = sync_service.clone();
        sync_service.set_local_data_path(std::path::PathBuf::from(local_data_path));
        info!("Local data path configured");
    }

    // Load repository config
    let config_path = std::env::var("REPOS_CONFIG").unwrap_or_else(|_| "config/repositories.json".into());
    let config_json = std::fs::read_to_string(&config_path)
        .unwrap_or_else(|_| r#"[]"#.to_string());
    if let Err(e) = sync_service.load_config(&config_json) {
        tracing::warn!("Failed to load config from {}: {}", config_path, e);
    }

    // Trigger initial sync on startup (non-blocking)
    let sync_for_startup = Arc::clone(&sync_service);
    tokio::spawn(async move {
        if let Err(e) = sync_for_startup.full_sync().await {
            tracing::warn!("Initial sync failed, will retry via cron: {}", e);
        }
    });

    // Start hourly cron sync
    let sync_for_cron = Arc::clone(&sync_service);
    tokio::spawn(async move {
        use tokio_cron::{Scheduler, Job};
        let mut scheduler = Scheduler::utc();
        let job = Job::new_sync("0 0 * * * *", move || {
            let sync = Arc::clone(&sync_for_cron);
            tokio::spawn(async move {
                if let Err(e) = sync.full_sync().await {
                    tracing::error!("Hourly sync failed: {}", e);
                }
            });
        });
        scheduler.add(job);
        // Scheduler runs automatically after utc()
        futures::future::pending::<()>().await // keep alive forever
    });

    let state = AppState {
        cache,
        github,
        sync: sync_service,
    };

    let app = build_app(state);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3003".into())
        .parse::<u16>()?;

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Marketplace API listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
