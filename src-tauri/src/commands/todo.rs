use crate::models::{CreateTodoRequest, Todo, UpdateTodoRequest};
use crate::services::database::DatabaseState;
use crate::services::todo_service;
use tauri::State;

#[tauri::command]
pub async fn get_todos(state: State<'_, DatabaseState>) -> Result<Vec<Todo>, String> {
    todo_service::get_all_todos(&state)
}

#[tauri::command]
pub async fn create_todo(
    state: State<'_, DatabaseState>,
    request: CreateTodoRequest,
) -> Result<Todo, String> {
    todo_service::create_new_todo(&state, request)
}

#[tauri::command]
pub async fn update_todo(
    state: State<'_, DatabaseState>,
    request: UpdateTodoRequest,
) -> Result<Todo, String> {
    todo_service::update_existing_todo(&state, request)
}

#[tauri::command]
pub async fn delete_todo(
    state: State<'_, DatabaseState>,
    todo_id: String,
) -> Result<(), String> {
    todo_service::delete_existing_todo(&state, &todo_id)
}
