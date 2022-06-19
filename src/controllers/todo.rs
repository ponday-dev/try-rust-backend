use std::sync::Arc;

use crate::{app::State, models::todo::*, repositories::todo::*};
use axum::{extract, http::StatusCode, response, Extension};

pub async fn index(Extension(state): Extension<Arc<State>>) -> impl response::IntoResponse {
    let result = index_todo(&state.pool).await;

    match result {
        Ok(todos) => response::Json(IndexTodoResponse {
            todos: Some(todos),
            error: None,
        }),
        Err(err) => {
            let database_error = err.as_database_error();
            let message = database_error.unwrap().to_string();

            response::Json(IndexTodoResponse {
                todos: None,
                error: Some(message),
            })
        }
    }
}

pub async fn create(
    Extension(state): Extension<Arc<State>>,
    extract::Json(payload): extract::Json<CreateTodoRequest>,
) -> impl response::IntoResponse {
    let data = CreateTodo {
        title: &payload.title.as_str(),
        body: &payload.body.as_str(),
    };

    let result = create_todo(&state.pool, &data).await;

    match result {
        Ok(todo) => (
            StatusCode::CREATED,
            response::Json(CreateTodoResponse {
                id: Some(todo.id),
                error: None,
            }),
        ),
        Err(err) => {
            let database_error = err.as_database_error();
            let message = database_error.unwrap().to_string();
            (
                StatusCode::BAD_REQUEST,
                response::Json(CreateTodoResponse {
                    id: None,
                    error: Some(message),
                }),
            )
        }
    }
}
