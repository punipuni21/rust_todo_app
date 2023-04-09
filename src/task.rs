use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Completed,
    Abandoned,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    task_id: Uuid,
    creator: String,
    title: String,
    description: String,
    due_date: DateTime<FixedOffset>,
    status: TaskStatus,
}
