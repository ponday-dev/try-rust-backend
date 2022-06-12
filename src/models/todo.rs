use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: String,
}

pub struct CreateTodo<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Serialize)]
pub struct CreateTodoResponse {
    pub id: i32,
}
