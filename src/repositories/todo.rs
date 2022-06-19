use sqlx::{Pool, Sqlite};

use crate::models::todo::{CreateTodo, Todo};

pub async fn index_todo(pool: &Pool<Sqlite>) -> anyhow::Result<Vec<Todo>, sqlx::Error> {
    sqlx::query_as!(Todo, "SELECT id, title, body FROM todos")
        .fetch_all(pool)
        .await
}

pub async fn create_todo(
    pool: &Pool<Sqlite>,
    create_todo: &CreateTodo<'_>,
) -> anyhow::Result<Todo, sqlx::Error> {
    sqlx::query_as!(
        Todo,
        "INSERT INTO todos (title, body) VALUES (?, ?) RETURNING id, title, body",
        create_todo.title,
        create_todo.body,
    )
    .fetch_one(pool)
    .await
}
