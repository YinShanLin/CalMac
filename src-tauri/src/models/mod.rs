use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TodoType {
    Global,
    Daily,
}

impl TodoType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Global => "global",
            Self::Daily => "daily",
        }
    }

    pub fn from_db(s: &str) -> Result<Self, String> {
        match s {
            "global" => Ok(Self::Global),
            "daily" => Ok(Self::Daily),
            other => Err(format!("invalid todo_type: {}", other)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }

    pub fn from_db(s: &str) -> Result<Self, String> {
        match s {
            "high" => Ok(Self::High),
            "medium" => Ok(Self::Medium),
            "low" => Ok(Self::Low),
            other => Err(format!("invalid priority: {}", other)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub all_day: bool,
    pub color: Option<String>,
    pub remind_minutes: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    pub title: String,
    pub description: Option<String>,
    pub start_time: String,
    pub end_time: String,
    #[serde(default)]
    pub all_day: Option<bool>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub remind_minutes: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEventRequest {
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<Option<String>>,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub end_time: Option<String>,
    #[serde(default)]
    pub all_day: Option<bool>,
    #[serde(default)]
    pub color: Option<Option<String>>,
    #[serde(default)]
    pub remind_minutes: Option<Option<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub todo_type: TodoType,
    pub date: Option<String>,
    pub priority: Option<Priority>,
    pub sort_order: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub todo_type: TodoType,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub priority: Option<Priority>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTodoRequest {
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub completed: Option<bool>,
    #[serde(default)]
    pub priority: Option<Option<Priority>>,
    #[serde(default)]
    pub sort_order: Option<i32>,
}
