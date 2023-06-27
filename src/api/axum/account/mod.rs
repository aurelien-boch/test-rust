use axum::Router;
use axum::routing::{post};

mod create;

pub fn router() -> Router {
    Router::new()
        .route("/", post(create::handler))
}
