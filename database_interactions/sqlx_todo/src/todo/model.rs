use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row, SqlitePool};

// this struct will use to receive user input
#[derive(Serialize, Deserialize)]
pub struct TodoRequest {
    pub description: String,
    pub done: bool,
}

// this struct will be used to represent database record
#[derive(Serialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub done: bool,
}

// implementation of Actix Responder for Todo struct so we can return Todo from action handler
impl Responder for Todo {
    type Error = Error;
    type Future = HttpResponse;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        // create response and set content type
        HttpResponse::Ok().json(&self)
    }
}

// Implementation for Todo struct, functions for read/write/update and delete todo from database
impl Todo {
    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Todo>> {
        let todos = sqlx::query!(
            r#"
            SELECT id, description, done
            FROM todos
            ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|rec| Todo {
            id: rec.id,
            description: rec.description,
            done: rec.done,
        })
        .collect();

        Ok(todos)
    }

    pub async fn find_by_id(id: i32, pool: &SqlitePool) -> Result<Option<Todo>> {
        let rec = sqlx::query!(
            r#"
            SELECT id, description, done
            FROM todos
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*pool)
        .await?;

        Ok(rec.map(|rec| Todo {
            id: rec.id,
            description: rec.description,
            done: rec.done,
        }))
    }

    pub async fn create(todo: TodoRequest, pool: &SqlitePool) -> Result<Todo> {
        let mut tx = pool.begin().await?;

        sqlx::query!(
            r#"
            INSERT INTO todos (description, done)
            VALUES ($1, $2)
            "#,
            todo.description,
            todo.done,
        )
        .execute(&mut tx)
        .await?;

        // TODO: this can be replaced with RETURNING with sqlite v3.35+ and/or sqlx v0.5+
        let row_id: i32 = sqlx::query("SELECT last_insert_rowid()")
            .map(|row: SqliteRow| row.get(0))
            .fetch_one(&mut tx)
            .await?;

        let rec = sqlx::query!(
            r#"
            SELECT id, description, done
            FROM todos
            WHERE id = $1
            "#,
            row_id,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(Todo {
            id: rec.id,
            description: rec.description,
            done: rec.done,
        })
    }

    pub async fn update(
        id: i32,
        todo: TodoRequest,
        pool: &SqlitePool,
    ) -> Result<Option<Todo>> {
        let mut tx = pool.begin().await.unwrap();

        let n = sqlx::query!(
            r#"
            UPDATE todos
            SET description = $1, done = $2
            WHERE id = $3
            "#,
            todo.description,
            todo.done,
            id,
        )
        .execute(&mut tx)
        .await?;

        if n == 0 {
            return Ok(None);
        }

        // TODO: this can be replaced with RETURNING with sqlite v3.35+ and/or sqlx v0.5+
        let todo = sqlx::query!(
            r#"
            SELECT id, description, done
            FROM todos
            WHERE id = $1
            "#,
            id,
        )
        .fetch_one(&mut tx)
        .await
        .map(|rec| Todo {
            id: rec.id,
            description: rec.description,
            done: rec.done,
        })?;

        tx.commit().await.unwrap();
        Ok(Some(todo))
    }

    pub async fn delete(id: i32, pool: &SqlitePool) -> Result<u64> {
        let mut tx = pool.begin().await?;

        let n_deleted = sqlx::query!(
            r#"
            DELETE FROM todos
            WHERE id = $1
            "#,
            id,
        )
        .execute(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(n_deleted)
    }
}
