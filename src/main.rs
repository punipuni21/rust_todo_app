use chrono::{DateTime, Utc, FixedOffset};
use dotenv::dotenv;
use lazy_static::lazy_static;
use mysql_async::{Pool, prelude::Queryable};
use serde::{Serialize, Deserialize};
use std::env;
use uuid::Uuid;

lazy_static! {
  static ref DATABASE_URL: String = {
      dotenv().ok();
      let user_name = env::var("USER_NAME").unwrap();
      let password = env::var("PASSWORD").unwrap();
      let db_name = env::var("DB_NAME").unwrap();
      format!("mysql://{}:{}@localhost:3306/{}", user_name, password, db_name)
  };
}

#[tokio::main]
async fn main() {
  let task = Task {
    task_id: Uuid::new_v4(),
    creator: String::from("Alice"),
    title: String::from("Test task"),
    description: Some(String::from("This is a test task.")),
    due_date: Some(DateTime::parse_from_rfc3339("2023-04-30T00:00:00+09:00").unwrap()),
    status: TaskStatus::NotStarted,
  };
  
  if let Err(e) = task.add_task(&DATABASE_URL).await {
      eprintln!("Error: {}", e);
  } else {
      println!("Task added successfully.");
  }
}
  

      
