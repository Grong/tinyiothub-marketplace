pub mod templates;
pub mod drivers;
pub mod webhook;

use axum::Router;
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(templates::routes())
        .merge(drivers::routes())
        .merge(webhook::routes())
}
