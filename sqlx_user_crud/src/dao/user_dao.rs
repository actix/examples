use super::Table;
use super::User;
use sqlx::mysql::MySqlQueryAs;

impl<'c> Table<'c, User> {
    pub async fn drop_table(&self) -> Result<u64, sqlx::Error> {
        sqlx::query("DROP TABLE IF EXISTS users;")
            .execute(&*self.pool)
            .await
    }

    pub async fn create_table(&self) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
            id VARCHAR(48) NOT NULL UNIQUE,
            name VARCHAR(64) NOT NULL UNIQUE,
            email VARCHAR(256) NOT NULL UNIQUE,
            PRIMARY KEY (id)
            )"#,
        )
        .execute(&*self.pool)
        .await
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `id`, `name`, `email`
            FROM `users`
            WHERE `id` = ?"#,
        )
        .bind(user_id)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn add_user(&self, user: &User) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO users (`id`, `name`, `email`)
            VALUES(?, ?, ?)"#,
        )
        .bind(&user.id)
        .bind(&user.name)
        .bind(&user.email)
        .execute(&*self.pool)
        .await
    }

    pub async fn update_user(&self, user: &User) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE users
            SET `name` = ?, `email` = ?
            WHERE `id` = ?
            "#,
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.id)
        .execute(&*self.pool)
        .await
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM users
            WHERE `id` = ?
            "#,
        )
        .bind(user_id)
        .execute(&*self.pool)
        .await
    }
}
