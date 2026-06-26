use rusqlite::{Connection, Result};
use std::sync::Mutex;

pub struct DatabaseState {
    pub conn: Mutex<Connection>,
}

const CURRENT_VERSION: i32 = 2;

impl DatabaseState {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        migrate(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}

fn column_exists(conn: &Connection, table: &str, col: &str) -> Result<bool> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table))?;
    let exists = stmt
        .query_map([], |r| r.get::<_, String>(1))?
        .filter_map(|r| r.ok())
        .any(|name| name == col);
    Ok(exists)
}

fn migrate(conn: &Connection) -> Result<()> {
    // v1: 基础表
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS events (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            start_time TEXT NOT NULL,
            end_time TEXT NOT NULL,
            all_day INTEGER NOT NULL DEFAULT 0,
            color TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS todos (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            completed INTEGER NOT NULL DEFAULT 0,
            todo_type TEXT NOT NULL DEFAULT 'global',
            date TEXT,
            priority TEXT,
            sort_order INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS schema_meta (version INTEGER NOT NULL);",
    )?;

    let version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_meta",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    if version < CURRENT_VERSION {
        // v2: 事件提醒字段
        if !column_exists(conn, "events", "remind_minutes")? {
            conn.execute("ALTER TABLE events ADD COLUMN remind_minutes INTEGER", [])?;
        }
        conn.execute(
            "INSERT INTO schema_meta (version) VALUES (?1)",
            rusqlite::params![CURRENT_VERSION],
        )?;
    }

    Ok(())
}
