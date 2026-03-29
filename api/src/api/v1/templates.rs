use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json, Router,
};
use crate::AppState;
use crate::dto::{ApiResponse, PaginatedList, PaginationParams, TemplateDetail, TemplateListItem};
use crate::domain::Template;

const CACHE_STALE_HEADER: &'static str = "X-Cache-Stale";

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/templates", axum::routing::get(list_templates))
        .route("/templates/:id", axum::routing::get(get_template))
        .route("/templates/:id/download", axum::routing::get(download_template))
}

async fn list_templates(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Response, (StatusCode, Json<ApiResponse<()>>)> {
    if let Err(e) = params.validate() {
        return Err((StatusCode::BAD_REQUEST, Json(ApiResponse::<()>::error(400, format!("Invalid pagination: {}", e)))));
    }

    // Check if cache is cold
    let is_cold = state.cache.is_cold();

    // Try Sled cache
    match state.cache.get_templates() {
        Ok(Some(items)) => {
            // Apply filters
            let filtered = filter_and_search(&items, &params);

            // Paginate
            let total = filtered.len();
            let offset = params.offset();
            let page_items: Vec<TemplateListItem> = filtered
                .into_iter()
                .skip(offset)
                .take(params.per_page)
                .filter_map(|v: serde_json::Value| serde_json::from_value::<TemplateListItem>(v).ok())
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
            // Cold cache - try GitHub fallback
            let mut headers = HeaderMap::new();
            headers.insert(CACHE_STALE_HEADER, "true".parse().unwrap());

            // Return empty with stale header
            Ok((headers, Json(ApiResponse::success(PaginatedList::new(
                Vec::<TemplateListItem>::new(),
                0,
                params.page,
                params.per_page,
            )))).into_response())
        }
        Err(e) => {
            // Sled error - fallback to GitHub direct
            tracing::warn!("Sled read error, falling back to GitHub: {}", e);
            Err((StatusCode::BAD_GATEWAY, Json(ApiResponse::<()>::error(502, "Cache unavailable"))))
        }
    }
}

async fn get_template(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response, (StatusCode, Json<ApiResponse<()>>)> {
    let is_cold = state.cache.is_cold();

    match state.cache.get_templates() {
        Ok(Some(items)) => {
            match items.iter().find(|item| item.get("id").and_then(|v| v.as_str()) == Some(&id)) {
                Some(v) => {
                    match serde_json::from_value::<Template>(v.clone()) {
                        Ok(t) => {
                            let mut headers = HeaderMap::new();
                            if is_cold {
                                headers.insert(CACHE_STALE_HEADER, "true".parse().unwrap());
                            }
                            Ok((headers, Json(ApiResponse::success(TemplateDetail::from(t)))).into_response())
                        }
                        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error(500, format!("Data error: {}", e))))),
                    }
                }
                None => Err((StatusCode::NOT_FOUND, Json(ApiResponse::<()>::error(40401, "template not found")))),
            }
        }
        Ok(None) => Err((StatusCode::NOT_FOUND, Json(ApiResponse::<()>::error(40401, "template not found")))),
        Err(_) => Err((StatusCode::BAD_GATEWAY, Json(ApiResponse::<()>::error(502, "cache unavailable")))),
    }
}

async fn download_template(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response, (StatusCode, Json<ApiResponse<()>>)> {
    // First get the template to find file_url
    let items = match state.cache.get_templates() {
        Ok(Some(items)) => items,
        Ok(None) => return Err((StatusCode::NOT_FOUND, Json(ApiResponse::<()>::error(40401, "template not found")))),
        Err(_) => return Err((StatusCode::BAD_GATEWAY, Json(ApiResponse::<()>::error(502, "cache unavailable")))),
    };

    let template = match items.iter().find(|item| item.get("id").and_then(|v| v.as_str()) == Some(&id)) {
        Some(v) => v,
        None => return Err((StatusCode::NOT_FOUND, Json(ApiResponse::<()>::error(40401, "template not found")))),
    };

    let file_url = match template.get("file_url").and_then(|v| v.as_str()) {
        Some(url) => url,
        None => return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::<()>::error(50001, "template has no file_url")))),
    };

    // Redirect to GitHub raw content
    let mut headers = HeaderMap::new();
    headers.insert("Location", file_url.parse().unwrap());

    Ok((StatusCode::FOUND, headers, ()).into_response())
}

fn filter_and_search(items: &[serde_json::Value], params: &PaginationParams) -> Vec<serde_json::Value> {
    items.iter()
        .filter(|item| {
            // Category filter
            if let Some(ref cat) = params.category {
                if item.get("category").and_then(|v| v.as_str()) != Some(cat.as_str()) {
                    return false;
                }
            }

            // Protocol filter
            if let Some(ref proto) = params.protocol {
                if item.get("protocol").and_then(|v| v.as_str()) != Some(proto.as_str()) {
                    return false;
                }
            }

            // Search
            if let Some(ref search) = params.search {
                let search_lower = search.to_lowercase();
                let name_match = item.get("name").and_then(|v| v.as_str())
                    .map(|s| s.to_lowercase().contains(&search_lower))
                    .unwrap_or(false);
                let desc_match = item.get("description").and_then(|v| v.as_str())
                    .map(|s| s.to_lowercase().contains(&search_lower))
                    .unwrap_or(false);
                let tags_match = item.get("tags").and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str())
                        .any(|s| s.to_lowercase().contains(&search_lower)))
                    .unwrap_or(false);
                if !name_match && !desc_match && !tags_match {
                    return false;
                }
            }

            true
        })
        .cloned()
        .collect()
}
