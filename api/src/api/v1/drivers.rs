use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json, Router,
};
use std::str::FromStr;
use url::Url;
use crate::AppState;
use crate::dto::{ApiResponse, DriverDetail, DriverListItem, PaginatedList, PaginationParams};
use crate::domain::Driver;

const CACHE_STALE_HEADER: &'static str = "X-Cache-Stale";
const ALLOWED_DOWNLOAD_DOMAINS: &[&str] = &[
    "github.com",
    "raw.githubusercontent.com",
];

fn validate_download_url(url_str: &str) -> Result<Url, (StatusCode, Json<ApiResponse<()>>)> {
    let url = Url::from_str(url_str)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(ApiResponse::<()>::error(40001, "invalid download URL"))))?;

    if url.scheme() != "https" {
        return Err((StatusCode::BAD_REQUEST, Json(ApiResponse::<()>::error(40002, "download URL must use HTTPS"))));
    }

    let host = url.host_str().ok_or_else(|| (StatusCode::BAD_REQUEST, Json(ApiResponse::<()>::error(40003, "download URL missing host"))))?;

    if !ALLOWED_DOWNLOAD_DOMAINS.iter().any(|d| host == *d || host.ends_with(&format!(".{}", d))) {
        return Err((StatusCode::BAD_REQUEST, Json(ApiResponse::<()>::error(40004, "download URL domain not allowed"))));
    }

    Ok(url)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/drivers", axum::routing::get(list_drivers))
        .route("/drivers/{id}", axum::routing::get(get_driver))
        .route("/drivers/{id}/download", axum::routing::get(download_driver))
}

async fn list_drivers(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Response, (StatusCode, Json<ApiResponse<()>>)> {
    if let Err(e) = params.validate() {
        return Err((StatusCode::BAD_REQUEST, Json(ApiResponse::<()>::error(400, format!("Invalid pagination: {}", e)))));
    }

    let is_cold = state.cache.is_cold();

    match state.cache.get_drivers() {
        Ok(Some(items)) => {
            let filtered = filter_and_search(&items, &params);

            let total = filtered.len();
            let offset = params.offset();
            let page_items: Vec<DriverListItem> = filtered
                .into_iter()
                .skip(offset)
                .take(params.per_page)
                .filter_map(|v: serde_json::Value| serde_json::from_value::<DriverListItem>(v).ok())
                .map(DriverListItem::from)
                .collect();

            let mut headers = HeaderMap::new();
            if is_cold {
                headers.insert(CACHE_STALE_HEADER, "true".parse().unwrap());
            }
            headers.insert("X-Total-Count", total.to_string().parse().unwrap());
            headers.insert("X-Page", params.page.to_string().parse().unwrap());
            headers.insert("X-Per-Page", params.per_page.to_string().parse().unwrap());

            Ok((headers, Json(ApiResponse::success(PaginatedList::new(
                page_items,
                total,
                params.page,
                params.per_page,
            )))).into_response())
        }
        Ok(None) => {
            let mut headers = HeaderMap::new();
            headers.insert(CACHE_STALE_HEADER, "true".parse().unwrap());

            Ok((headers, Json(ApiResponse::success(PaginatedList::new(
                Vec::<DriverListItem>::new(),
                0,
                params.page,
                params.per_page,
            )))).into_response())
        }
        Err(e) => {
            tracing::warn!("Sled read error for drivers: {}", e);
            Err((StatusCode::BAD_GATEWAY, Json(ApiResponse::<()>::error(502, "Cache unavailable"))))
        }
    }
}

async fn get_driver(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response, (StatusCode, Json<ApiResponse<()>>)> {
    let is_cold = state.cache.is_cold();

    match state.cache.get_drivers() {
        Ok(Some(items)) => {
            match items.iter().find(|item| item.get("id").and_then(|v| v.as_str()) == Some(&id)) {
                Some(v) => {
                    match serde_json::from_value::<Driver>(v.clone()) {
                        Ok(d) => {
                            let mut headers = HeaderMap::new();
                            if is_cold {
                                headers.insert(CACHE_STALE_HEADER, "true".parse().unwrap());
                            }
                            Ok((headers, Json(ApiResponse::success(DriverDetail::from(d)))).into_response())
                        }
                        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error(500, format!("Data error: {}", e))))),
                    }
                }
                None => Err((StatusCode::NOT_FOUND, Json(ApiResponse::<()>::error(40401, "driver not found")))),
            }
        }
        Ok(None) => Err((StatusCode::NOT_FOUND, Json(ApiResponse::<()>::error(40401, "driver not found")))),
        Err(_) => Err((StatusCode::BAD_GATEWAY, Json(ApiResponse::<()>::error(502, "cache unavailable")))),
    }
}

async fn download_driver(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response, (StatusCode, Json<ApiResponse<()>>)> {
    let items = match state.cache.get_drivers() {
        Ok(Some(items)) => items,
        Ok(None) => return Err((StatusCode::NOT_FOUND, Json(ApiResponse::<()>::error(40401, "driver not found")))),
        Err(_) => return Err((StatusCode::BAD_GATEWAY, Json(ApiResponse::<()>::error(502, "cache unavailable")))),
    };

    let driver = match items.iter().find(|item| item.get("id").and_then(|v| v.as_str()) == Some(&id)) {
        Some(v) => v,
        None => return Err((StatusCode::NOT_FOUND, Json(ApiResponse::<()>::error(40401, "driver not found")))),
    };

    // For drivers, file_url might be in homepage or we redirect to documentation
    let redirect_url = driver.get("homepage")
        .and_then(|v| v.as_str())
        .or(driver.get("documentation").and_then(|v| v.as_str()))
        .unwrap_or("https://github.com");

    // Validate URL before redirect (SSRF protection)
    let validated_url = validate_download_url(redirect_url)?;

    let mut headers = HeaderMap::new();
    headers.insert("Location", validated_url.to_string().parse().unwrap());

    Ok((StatusCode::FOUND, headers, ()).into_response())
}

fn filter_and_search(items: &[serde_json::Value], params: &PaginationParams) -> Vec<serde_json::Value> {
    items.iter()
        .filter(|item| {
            if let Some(ref proto) = params.protocol {
                if item.get("protocol").and_then(|v| v.as_str()) != Some(proto.as_str()) {
                    return false;
                }
            }

            if let Some(ref search) = params.search {
                let search_lower = search.to_lowercase();
                let matches = [
                    item.get("name").and_then(|v| v.as_str()),
                    item.get("description").and_then(|v| v.as_str()),
                ].into_iter().flatten().any(|s| s.to_lowercase().contains(&search_lower));
                if !matches {
                    return false;
                }
            }

            true
        })
        .cloned()
        .collect()
}
