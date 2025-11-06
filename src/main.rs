mod config;
mod handlers;
mod models;
mod routes;
mod services;

use axum::Router;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize the application router
    let app = Router::new().merge(routes::create_routes());

    // Start the server
    let addr = "[::]:3000".parse().unwrap();
    println!("Server running on http://localhost:3000");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
