use sqlx::postgres::{PgPool, PgPoolOptions};

use crate::model::{NewTask, Task};

pub async fn init_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect(database_url)
        .await
}

pub async fn get_all_tasks(pool: &PgPool) -> Result<Vec<Task>, &'static str> {
    Task::all(pool).await.map_err(|_| "Error retrieving tasks")
}

pub async fn create_task(todo: String, pool: &PgPool) -> Result<(), &'static str> {
    let new_task = NewTask { description: todo };
    Task::insert(new_task, pool)
        .await
        .map(|_| ())
        .map_err(|_| "Error inserting task")
}

pub async fn toggle_task(id: i32, pool: &PgPool) -> Result<(), &'static str> {
    Task::toggle_with_id(id, pool)
        .await
        .map(|_| ())
        .map_err(|_| "Error toggling task completion")
}

pub async fn delete_task(id: i32, pool: &PgPool) -> Result<(), &'static str> {
    Task::delete_with_id(id, pool)
        .await
        .map(|_| ())
        .map_err(|_| "Error deleting task")
}
