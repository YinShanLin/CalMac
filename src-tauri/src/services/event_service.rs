use crate::models::{CreateEventRequest, Event, UpdateEventRequest};
use crate::services::database::DatabaseState;
use chrono::Utc;
use rusqlite::params;
use uuid::Uuid;

const EVENT_COLUMNS: &str = "id, title, description, start_time, end_time, all_day, color, remind_minutes, created_at, updated_at";

fn event_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Event> {
    Ok(Event {
        id: row.get(0)?,
        title: row.get(1)?,
        description: row.get(2)?,
        start_time: row.get(3)?,
        end_time: row.get(4)?,
        all_day: row.get::<_, i32>(5)? != 0,
        color: row.get(6)?,
        remind_minutes: row.get(7)?,
        created_at: row.get(8)?,
        updated_at: row.get(9)?,
    })
}

pub fn get_all_events(state: &DatabaseState) -> Result<Vec<Event>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(&format!("SELECT {} FROM events ORDER BY start_time", EVENT_COLUMNS))
        .map_err(|e| e.to_string())?;
    let events = stmt
        .query_map([], event_from_row)
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(events)
}

pub fn get_events_by_range(state: &DatabaseState, start: &str, end: &str) -> Result<Vec<Event>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(&format!(
            "SELECT {} FROM events WHERE start_time >= ?1 AND start_time < ?2 ORDER BY start_time",
            EVENT_COLUMNS
        ))
        .map_err(|e| e.to_string())?;
    let events = stmt
        .query_map(params![start, end], event_from_row)
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(events)
}

pub fn create_new_event(state: &DatabaseState, request: CreateEventRequest) -> Result<Event, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let event = Event {
        id: id.clone(),
        title: request.title,
        description: request.description,
        start_time: request.start_time,
        end_time: request.end_time,
        all_day: request.all_day.unwrap_or(false),
        color: request.color,
        remind_minutes: request.remind_minutes,
        created_at: now.clone(),
        updated_at: now,
    };

    conn.execute(
        "INSERT INTO events (id, title, description, start_time, end_time, all_day, color, remind_minutes, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            event.id,
            event.title,
            event.description,
            event.start_time,
            event.end_time,
            event.all_day as i32,
            event.color,
            event.remind_minutes,
            event.created_at,
            event.updated_at,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(event)
}

pub fn update_existing_event(state: &DatabaseState, request: UpdateEventRequest) -> Result<Event, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();

    let mut event = get_event_by_id(state, &request.id)?;

    if let Some(title) = request.title {
        event.title = title;
    }
    if let Some(description) = request.description {
        event.description = description;
    }
    if let Some(start_time) = request.start_time {
        event.start_time = start_time;
    }
    if let Some(end_time) = request.end_time {
        event.end_time = end_time;
    }
    if let Some(all_day) = request.all_day {
        event.all_day = all_day;
    }
    if let Some(color) = request.color {
        event.color = color;
    }
    if let Some(remind) = request.remind_minutes {
        event.remind_minutes = remind;
    }
    event.updated_at = now;

    conn.execute(
        "UPDATE events SET title = ?1, description = ?2, start_time = ?3, end_time = ?4, all_day = ?5, color = ?6, remind_minutes = ?7, updated_at = ?8 WHERE id = ?9",
        params![
            event.title,
            event.description,
            event.start_time,
            event.end_time,
            event.all_day as i32,
            event.color,
            event.remind_minutes,
            event.updated_at,
            event.id,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(event)
}

pub fn delete_existing_event(state: &DatabaseState, event_id: &str) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM events WHERE id = ?1", params![event_id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_event_by_id(state: &DatabaseState, event_id: &str) -> Result<Event, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    conn.query_row(
        &format!("SELECT {} FROM events WHERE id = ?1", EVENT_COLUMNS),
        params![event_id],
        event_from_row,
    )
    .map_err(|e| e.to_string())
}

pub fn get_remindable_events(state: &DatabaseState, after: &str) -> Result<Vec<Event>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(&format!(
            "SELECT {} FROM events WHERE remind_minutes IS NOT NULL AND remind_minutes > 0 AND start_time >= ?1 ORDER BY start_time",
            EVENT_COLUMNS
        ))
        .map_err(|e| e.to_string())?;
    let events = stmt
        .query_map(params![after], event_from_row)
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(events)
}
