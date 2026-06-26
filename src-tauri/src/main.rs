#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod models;
mod services;
mod utils;

use services::database::DatabaseState;
use services::reminder;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            std::fs::create_dir_all(&app_dir).expect("failed to create app data dir");
            let db_path = app_dir.join("calmac.db");
            let db_state = DatabaseState::new(db_path.to_str().unwrap())
                .expect("failed to create database state");
            app.manage(db_state);
            reminder::start_reminder_loop(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::calendar::get_events,
            commands::calendar::get_events_by_range,
            commands::calendar::create_event,
            commands::calendar::update_event,
            commands::calendar::delete_event,
            commands::todo::get_todos,
            commands::todo::create_todo,
            commands::todo::update_todo,
            commands::todo::delete_todo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
