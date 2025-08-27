pub mod error;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    AppState,
    todo::error::Error,
    todo_proto::{
        self, CreateTodoRequest, DeleteTodoRequest, FetchTodoByIdRequest, FetchTodosByRangeRequest,
        UpdateTodoRequest,
    },
};

#[inline]
pub fn create_todo_router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/todo/fetch-todo-by-id/{id}", get(fetch_todo_by_id))
        .route("/api/v1/todo/fetch-todo-by-range", get(fetch_todo_by_range))
        .route("/api/v1/todo/create-todo", post(create_todo))
        .route("/api/v1/todo/update-todo/{id}", put(update_todo))
        .route("/api/v1/todo/delete-todo/{id}", delete(delete_todo))
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy, Default, Hash)]
pub struct OffsetLimit {
    offset: usize,
    limit: usize,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Todo {
    id: String,
    title: String,
    description: String,
    is_done: bool,
    updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct CreateTodo {
    title: String,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct UpdateTodo {
    title: Option<String>,
    description: Option<String>,
    is_done: Option<bool>,
}

impl From<todo_proto::TodoItem> for Todo {
    fn from(todo: todo_proto::TodoItem) -> Self {
        fn from_timestamp_to_iso8601(ts: &prost_types::Timestamp) -> Option<String> {
            use chrono::DateTime;

            DateTime::from_timestamp(ts.seconds, ts.nanos as u32)
                .map(|dt| DateTime::to_rfc3339(&dt))
        }

        let updated_at = todo
            .updated_at
            .and_then(|date| from_timestamp_to_iso8601(&date));

        Self {
            id: todo.id,
            title: todo.title,
            description: todo.description,
            is_done: todo.is_done,
            updated_at: updated_at.unwrap_or_default(),
        }
    }
}

async fn fetch_todo_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, Error> {
    let mut service = state.todo_service.lock().await;

    let request = FetchTodoByIdRequest { id };
    let response = service.fetch_todo_by_id(request).await?.into_inner();

    Ok((
        StatusCode::OK,
        Json(json!({
            "msg": response.msg,
            "todo": response.todo.map(Todo::from)
        })),
    ))
}

async fn fetch_todo_by_range(
    State(state): State<AppState>,
    Query(OffsetLimit { offset, limit }): Query<OffsetLimit>,
) -> Result<impl IntoResponse, Error> {
    let mut service = state.todo_service.lock().await;

    let request = FetchTodosByRangeRequest {
        offset: offset as u32,
        limit: limit as u32,
    };
    let response = service.fetch_todos_by_range(request).await?.into_inner();

    Ok((
        StatusCode::OK,
        Json(json!({
            "count": response.count,
            "todos": response.todos.into_iter().map(Todo::from).collect::<Vec<_>>()
        })),
    ))
}

async fn create_todo(
    State(state): State<AppState>,
    Json(CreateTodo { title, description }): Json<CreateTodo>,
) -> Result<impl IntoResponse, Error> {
    let mut service = state.todo_service.lock().await;

    let request = CreateTodoRequest { title, description };
    let response = service.create_todo(request).await?.into_inner();

    Ok((
        StatusCode::OK,
        Json(json!({
            "msg": response.msg,
            "todo": response.todo.map(Todo::from)
        })),
    ))
}

async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(UpdateTodo {
        title,
        description,
        is_done,
    }): Json<UpdateTodo>,
) -> Result<impl IntoResponse, Error> {
    let mut service = state.todo_service.lock().await;

    if title.is_none() && description.is_none() && is_done.is_none() {
        return Err(Error::InvalidArgument);
    }

    let request = UpdateTodoRequest {
        id,
        title,
        description,
        is_done,
    };
    let response = service.update_todo(request).await?.into_inner();

    Ok((
        StatusCode::OK,
        Json(json!({
            "msg": response.msg,
            "todo": response.todo.map(Todo::from)
        })),
    ))
}

async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, Error> {
    let mut service = state.todo_service.lock().await;

    let request = DeleteTodoRequest { id };
    let response = service.delete_todo(request).await?.into_inner();

    Ok((
        StatusCode::OK,
        Json(json!({
            "msg": response.msg,
        })),
    ))
}
