use chrono::{DateTime, FixedOffset, Utc};
use mysql_async::{Pool, prelude::Queryable};
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

use chrono::{DateTime, Utc, FixedOffset};
use mysql_async::{Pool, prelude::Queryable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

// TaskStatus and Task structs were defined here

impl Task {
    pub async fn add_task(&self, database_url: &str) -> Result<(), mysql_async::Error> {
        let pool = Pool::new(database_url);
        let mut conn = pool.get_conn().await?;

        let task_id = self.task_id.to_string();
        let due_date = self.due_date.map(|dt| dt.to_rfc3339());
        let status = format!("{:?}", self.status);

        conn.exec_drop(
            "INSERT INTO tasks (task_id, creator, title, description, due_date, status) VALUES (:task_id, :creator, :title, :description, :due_date, :status)",
            params! {
                "task_id" => task_id,
                "creator" => &self.creator,
                "title" => &self.title,
                "description" => &self.description,
                "due_date" => due_date,
                "status" => status
            },
        ).await?;

        conn.disconnect().await?;
        Ok(())
    }
}
