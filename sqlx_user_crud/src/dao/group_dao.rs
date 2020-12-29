use super::Group;
use super::Table;
use sqlx::mysql::MySqlQueryAs;

impl<'c> Table<'c, Group> {
    pub async fn create_table(&self) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS `groups`
            (
                `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
                `name` VARCHAR(64) NOT NULL UNIQUE,
                PRIMARY KEY(id)
            )
        "#,
        )
        .execute(&*self.pool)
        .await
    }

    pub async fn drop_table(&self) -> Result<u64, sqlx::Error> {
        sqlx::query("DROP TABLE IF EXISTS `groups`")
            .execute(&*self.pool)
            .await
    }

    pub async fn get_group_by_id(&self, id: u64) -> Result<Group, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `id`, `name`
            FROM `groups`
            WHERE `id` = ?
        "#,
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn get_group_by_name(&self, name: &str) -> Result<Group, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `id`, `name`
            FROM `groups`
            WHERE `name` = ?
        "#,
        )
        .bind(name)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn add_group(&self, name: &str) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO `groups` (`name`)
            VALUES (?)
        "#,
        )
        .bind(name)
        .execute(&*self.pool)
        .await
    }

    pub async fn update_group(
        &self,
        current: &str,
        update: &str,
    ) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE `groups`
            SET `name` = ?
            WHERE `name` = ?
        "#,
        )
        .bind(update)
        .bind(current)
        .execute(&*self.pool)
        .await
    }

    pub async fn delete_group(&self, name: &str) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM `groups`
            WHERE `name` = ?
        "#,
        )
        .bind(name)
        .execute(&*self.pool)
        .await
    }
}
