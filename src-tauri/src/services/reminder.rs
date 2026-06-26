use std::collections::HashSet;
use std::sync::Mutex;
use std::time::Duration;

use chrono::{DateTime, Utc};
use tauri::{AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;
use tokio::time::sleep;

use crate::services::database::DatabaseState;
use crate::services::event_service;

pub fn start_reminder_loop(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let notified: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
        loop {
            sleep(Duration::from_secs(60)).await;
            check_and_notify(&app, &notified);
        }
    });
}

fn check_and_notify(app: &AppHandle, notified: &Mutex<HashSet<String>>) {
    let state = match app.try_state::<DatabaseState>() {
        Some(s) => s,
        None => return,
    };

    let now = Utc::now();
    let now_str = now.to_rfc3339();
    let events = match event_service::get_remindable_events(&state, &now_str) {
        Ok(e) => e,
        Err(_) => return,
    };

    let mut to_remove: Vec<String> = Vec::new();
    {
        let mut set = notified.lock().unwrap();
        for event in &events {
            let start = match DateTime::parse_from_rfc3339(&event.start_time) {
                Ok(t) => t.with_timezone(&Utc),
                Err(_) => continue,
            };
            if now > start + chrono::Duration::hours(24) {
                to_remove.push(event.id.clone());
            }
        }
        for id in &to_remove {
            set.remove(id);
        }
    }

    for event in events {
        let remind_minutes = match event.remind_minutes {
            Some(m) if m > 0 => m,
            _ => continue,
        };
        let start = match DateTime::parse_from_rfc3339(&event.start_time) {
            Ok(t) => t.with_timezone(&Utc),
            Err(_) => continue,
        };
        let remind_at = start - chrono::Duration::minutes(remind_minutes as i64);
        if now >= remind_at && now <= start {
            let mut set = notified.lock().unwrap();
            if set.insert(event.id.clone()) {
                drop(set);
                let body = event.description.clone().unwrap_or_else(|| {
                    format!("即将开始：{}", event.start_time)
                });
                let _ = app
                    .notification()
                    .builder()
                    .title(format!("日程提醒：{}", event.title))
                    .body(body)
                    .show();
            }
        }
    }
}
