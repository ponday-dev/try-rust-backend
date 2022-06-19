use std::sync::Arc;

use crate::controllers::todo;
use axum::{routing, Extension, Router};
use sqlx::{Pool, Sqlite};

pub struct State {
    pub pool: Pool<Sqlite>,
}

pub fn create_router(state: Arc<State>) -> Router {
    Router::new()
        .route("/todos", routing::get(todo::index).post(todo::create))
        .layer(Extension(state))
}
