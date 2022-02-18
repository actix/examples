use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct NewTask {
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub id: i64,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub async fn all(connection: &SqlitePool) -> Result<Vec<Task>, sqlx::Error> {
        let tasks = sqlx::query_as!(
            Task,
            r#"
            SELECT *
            FROM tasks
            "#
        )
        .fetch_all(connection)
        .await?;

        Ok(tasks)
    }

    pub async fn insert(todo: NewTask, connection: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO tasks (description)
            VALUES ($1)
            "#,
            todo.description,
        )
        .execute(connection)
        .await?;

        Ok(())
    }

    pub async fn toggle_with_id(id: i32, connection: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE tasks
            SET completed = NOT completed
            WHERE id = $1
            "#,
            id
        )
        .execute(connection)
        .await?;

        Ok(())
    }

    pub async fn delete_with_id(id: i32, connection: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM tasks
            WHERE id = $1
            "#,
            id
        )
        .execute(connection)
        .await?;

        Ok(())
    }
}
