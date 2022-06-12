use axum::{routing, Router};
use std::net::SocketAddr;

use try_rust_backend::controllers::todo;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/todos", routing::get(todo::index).post(todo::create));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
