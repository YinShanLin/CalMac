use crate::models::{CreateTodoRequest, Priority, Todo, TodoType, UpdateTodoRequest};
use crate::services::database::DatabaseState;
use chrono::Utc;
use rusqlite::params;
use uuid::Uuid;

const TODO_COLUMNS: &str = "id, title, completed, todo_type, date, priority, sort_order, created_at, updated_at";

fn todo_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Todo> {
    let todo_type_str: String = row.get(3)?;
    let priority_str: Option<String> = row.get(5)?;
    Ok(Todo {
        id: row.get(0)?,
        title: row.get(1)?,
        completed: row.get::<_, i32>(2)? != 0,
        todo_type: TodoType::from_db(&todo_type_str).unwrap_or(TodoType::Global),
        date: row.get(4)?,
        priority: priority_str.and_then(|s| Priority::from_db(&s).ok()),
        sort_order: row.get(6)?,
        created_at: row.get(7)?,
        updated_at: row.get(8)?,
    })
}

pub fn get_all_todos(state: &DatabaseState) -> Result<Vec<Todo>, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(&format!("SELECT {} FROM todos ORDER BY sort_order, created_at", TODO_COLUMNS))
        .map_err(|e| e.to_string())?;
    let todos = stmt
        .query_map([], todo_from_row)
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(todos)
}

fn next_sort_order(conn: &rusqlite::Connection, todo_type: TodoType, date: &Option<String>) -> i32 {
    let max: i32 = match todo_type {
        TodoType::Global => conn
            .query_row(
                "SELECT COALESCE(MAX(sort_order), -1) FROM todos WHERE todo_type = 'global'",
                [],
                |r| r.get(0),
            )
            .unwrap_or(-1),
        TodoType::Daily => conn
            .query_row(
                "SELECT COALESCE(MAX(sort_order), -1) FROM todos WHERE todo_type = 'daily' AND date = ?1",
                params![date],
                |r| r.get(0),
            )
            .unwrap_or(-1),
    };
    max + 1
}

pub fn create_new_todo(state: &DatabaseState, request: CreateTodoRequest) -> Result<Todo, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let sort_order = next_sort_order(&conn, request.todo_type, &request.date);

    let todo = Todo {
        id: id.clone(),
        title: request.title,
        completed: false,
        todo_type: request.todo_type,
        date: request.date,
        priority: request.priority,
        sort_order,
        created_at: now.clone(),
        updated_at: now,
    };

    conn.execute(
        "INSERT INTO todos (id, title, completed, todo_type, date, priority, sort_order, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            todo.id,
            todo.title,
            todo.completed as i32,
            todo.todo_type.as_str(),
            todo.date,
            todo.priority.map(|p| p.as_str().to_string()),
            todo.sort_order,
            todo.created_at,
            todo.updated_at,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(todo)
}

pub fn update_existing_todo(state: &DatabaseState, request: UpdateTodoRequest) -> Result<Todo, String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();

    let mut todo = conn
        .query_row(
            &format!("SELECT {} FROM todos WHERE id = ?1", TODO_COLUMNS),
            params![request.id],
            todo_from_row,
        )
        .map_err(|e| e.to_string())?;

    if let Some(title) = request.title {
        todo.title = title;
    }
    if let Some(completed) = request.completed {
        todo.completed = completed;
    }
    if let Some(priority) = request.priority {
        todo.priority = priority;
    }
    if let Some(sort_order) = request.sort_order {
        todo.sort_order = sort_order;
    }
    todo.updated_at = now;

    conn.execute(
        "UPDATE todos SET title = ?1, completed = ?2, priority = ?3, sort_order = ?4, updated_at = ?5 WHERE id = ?6",
        params![
            todo.title,
            todo.completed as i32,
            todo.priority.map(|p| p.as_str().to_string()),
            todo.sort_order,
            todo.updated_at,
            todo.id,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(todo)
}

pub fn delete_existing_todo(state: &DatabaseState, todo_id: &str) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM todos WHERE id = ?1", params![todo_id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
