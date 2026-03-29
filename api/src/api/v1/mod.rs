pub mod templates;
pub mod drivers;

use axum::Router;
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(templates::routes())
        .merge(drivers::routes())
}
