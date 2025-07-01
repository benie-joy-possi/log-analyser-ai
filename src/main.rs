mod models;
mod routes;
mod services;
use axum::Router;

use crate::routes::analyse::analyze_handler;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/analyze", axum::routing::get(analyze_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
