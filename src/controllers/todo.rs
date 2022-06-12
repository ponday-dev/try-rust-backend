use crate::models::todo::*;
use axum::{extract, response};

pub async fn index() -> impl response::IntoResponse {
    response::Json(vec![Todo {
        id: 1,
        title: "title".to_string(),
        body: "body".to_string(),
    }])
}

pub async fn create(
    extract::Json(new_todo): extract::Json<CreateTodoRequest>,
) -> response::Json<CreateTodoResponse> {
    response::Json(CreateTodoResponse { id: 1 })
}
