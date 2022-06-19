use std::{net::SocketAddr, sync::Arc};

use try_rust_backend::{app, db};

#[tokio::main]
async fn main() {
    let pool = db::connect().await;
    let state = Arc::new(app::State { pool });
    let router = app::create_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
