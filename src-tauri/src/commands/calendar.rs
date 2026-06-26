use crate::models::{CreateEventRequest, Event, UpdateEventRequest};
use crate::services::database::DatabaseState;
use crate::services::event_service;
use tauri::State;

#[tauri::command]
pub async fn get_events(state: State<'_, DatabaseState>) -> Result<Vec<Event>, String> {
    event_service::get_all_events(&state)
}

#[tauri::command]
pub async fn get_events_by_range(
    state: State<'_, DatabaseState>,
    start: String,
    end: String,
) -> Result<Vec<Event>, String> {
    event_service::get_events_by_range(&state, &start, &end)
}

#[tauri::command]
pub async fn create_event(
    state: State<'_, DatabaseState>,
    request: CreateEventRequest,
) -> Result<Event, String> {
    event_service::create_new_event(&state, request)
}

#[tauri::command]
pub async fn update_event(
    state: State<'_, DatabaseState>,
    request: UpdateEventRequest,
) -> Result<Event, String> {
    event_service::update_existing_event(&state, request)
}

#[tauri::command]
pub async fn delete_event(
    state: State<'_, DatabaseState>,
    event_id: String,
) -> Result<(), String> {
    event_service::delete_existing_event(&state, &event_id)
}
