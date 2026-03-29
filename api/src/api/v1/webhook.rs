use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json, Router,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use crate::AppState;
use crate::dto::ApiResponse;

type HmacSha256 = Hmac<Sha256>;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/webhook/github", axum::routing::post(webhook_handler))
}

// Middleware chain: hmac_verify → event_filter → idempotency_check → lock_acquire → sync_handler

async fn webhook_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: String,
) -> Result<Response, (StatusCode, Json<ApiResponse<()>>)> {
    // Step 1: HMAC verification
    let webhook_secret = std::env::var("GITHUB_WEBHOOK_SECRET").unwrap_or_default();

    if let Some(signature_header) = headers.get("x-hub-signature-256") {
        let signature = signature_header.to_str().unwrap_or("");
        if !verify_hmac(&webhook_secret, &body, signature) {
            tracing::warn!("HMAC verification failed");
            return Err((StatusCode::UNAUTHORIZED, Json(ApiResponse::<()>::error(401, "Unauthorized"))));
        }
    } else {
        tracing::warn!("No HMAC signature provided");
        return Err((StatusCode::UNAUTHORIZED, Json(ApiResponse::<()>::error(401, "Unauthorized"))));
    }

    // Step 2: Event type filter (X-GitHub-Event header)
    let event_type = headers.get("x-github-event")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if event_type != "push" {
        // Non-push events return 200 but don't process
        tracing::info!("Ignoring non-push event: {}", event_type);
        return Ok((StatusCode::OK, Json(ApiResponse::success(serde_json::json!({"ignored": true})))) .into_response());
    }

    // Step 3: Idempotency check (X-GitHub-Delivery header)
    if let Some(delivery_id) = headers.get("x-github-delivery")
        .and_then(|v| v.to_str().ok())
    {
        match state.cache.check_idempotency(delivery_id) {
            Ok(true) => {
                tracing::info!("Duplicate delivery: {}", delivery_id);
                return Ok((StatusCode::OK, Json(ApiResponse::success(serde_json::json!({"duplicate": true})))) .into_response());
            }
            Ok(false) => {}
            Err(e) => {
                tracing::warn!("Idempotency check failed: {}", e);
            }
        }
    }

    // Step 4: Lock acquire
    let holder_id = uuid::Uuid::new_v4().to_string();
    match state.cache.acquire_sync_lock(&holder_id) {
        Ok(true) => {}
        Ok(false) => {
            tracing::warn!("Sync lock conflict");
            return Err((StatusCode::CONFLICT, Json(ApiResponse::<()>::error(409, "Sync in progress"))));
        }
        Err(e) => {
            tracing::error!("Failed to acquire sync lock: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error(500, "Internal error"))));
        }
    }

    // Step 5: Sync handler (actual sync)
    let result = state.sync.full_sync().await;

    // Release lock
    if let Err(e) = state.cache.release_sync_lock(&holder_id) {
        tracing::warn!("Failed to release sync lock: {}", e);
    }

    match result {
        Ok(()) => {
            tracing::info!("Webhook sync completed successfully");
            Ok((StatusCode::OK, Json(ApiResponse::success(serde_json::json!({"synced": true})))) .into_response())
        }
        Err(e) => {
            tracing::error!("Webhook sync failed: {}", e);
            Ok((StatusCode::OK, Json(ApiResponse::success(serde_json::json!({"synced": false, "error": e.to_string()})))) .into_response())
        }
    }
}

fn verify_hmac(secret: &str, body: &str, signature_header: &str) -> bool {
    // GitHub sends: sha256=<hmac>
    let signature = signature_header
        .strip_prefix("sha256=")
        .unwrap_or(signature_header);

    let mut mac = match HmacSha256::new_from_slice(secret.as_bytes()) {
        Ok(m) => m,
        Err(_) => return false,
    };
    mac.update(body.as_bytes());

    match hex::decode(signature) {
        Ok(expected) => mac.verify_slice(&expected).is_ok(),
        Err(_) => false,
    }
}
