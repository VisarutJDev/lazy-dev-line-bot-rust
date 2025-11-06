use axum::{routing::post, Router};

mod health;

pub fn create_routes() -> Router {
    Router::new().merge(health::create_routes()).route(
        "/webhook",
        post(crate::handlers::webhook_handler::handle_webhook),
    )
}
