use super::Group;
use super::JoinTable;
use super::User;
use sqlx::mysql::MySqlQueryAs;

impl<'c> JoinTable<'c, User, Group> {
    pub async fn create_table(&self) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS `users_to_groups`
            (
                `user_id` VARCHAR(48) NOT NULL,
                `group_id` BIGINT UNSIGNED NOT NULL,
                FOREIGN KEY (`user_id`) REFERENCES `users`(`id`),
                FOREIGN KEY (`group_id`) REFERENCES `groups`(`id`)
            )
        "#,
        )
        .execute(&*self.pool)
        .await
    }

    pub async fn drop_table(&self) -> Result<u64, sqlx::Error> {
        sqlx::query("DROP TABLE IF EXISTS users_to_groups")
            .execute(&*self.pool)
            .await
    }

    pub async fn add_user_groups(
        &self,
        user_id: &String,
        groups: &Vec<Group>,
    ) -> Result<u64, sqlx::Error> {
        if 0 == groups.len() {
            Ok(0)
        } else {
            let insert_statement = build_insert_statement(groups.len());
            let mut query = sqlx::query(&insert_statement);

            for group in groups {
                query = query.bind(user_id).bind(group.id)
            }

            query.execute(&*self.pool).await
        }
    }

    pub async fn get_groups_by_user_id(
        &self,
        user_id: &String,
    ) -> Result<Vec<Group>, sqlx::Error> {
        sqlx::query_as(
            r#"
            select * from `groups` as `a`
            where `a`.`id` in (
                select `b`.`group_id` from `users_to_groups` as `b`
                where `b`.`user_id` = ?
            )
        "#,
        )
        .bind(user_id)
        .fetch_all(&*self.pool)
        .await
    }

    pub async fn delete_by_user_id(&self, user_id: &String) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE
            FROM `users_to_groups`
            WHERE `user_id` = ?
        "#,
        )
        .bind(user_id)
        .execute(&*self.pool)
        .await
    }

    pub async fn delete_by_group_id(&self, group_id: u64) -> Result<u64, sqlx::Error> {
        sqlx::query(
            r#"
            DELETE
            FROM `users_to_groups`
            WHERE `group_id` = ?
        "#,
        )
        .bind(group_id)
        .execute(&*self.pool)
        .await
    }

    pub async fn update_user_groups(&self, user: &User) -> Result<u64, sqlx::Error> {
        if 0 == user.groups.len() {
            self.delete_by_user_id(&user.id).await
        } else {
            let deleted = self.delete_by_user_id(&user.id).await?;
            let added = self.add_user_groups(&user.id, &user.groups).await?;
            Ok(added + deleted)
        }
    }
}

static DEFAULT_INSERT: &'static str = r#"
    INSERT INTO `users_to_groups` (`user_id`, `group_id`)
    VALUES (?,?)
"#;

fn build_insert_statement(rows: usize) -> String {
    let mut insert = String::from(DEFAULT_INSERT);

    match rows {
        1 | 0 => insert,
        _ => {
            let mut i = 1;
            while i < rows {
                insert.push_str(", (?,?)");
                i += 1;
            }
            insert
        }
    }
}

#[cfg(test)]
mod test {
    use super::{build_insert_statement, DEFAULT_INSERT};

    #[test]
    fn build_insert_statement_returns_default_string_when_input_is_zero_or_one() {
        let results = vec![build_insert_statement(0), build_insert_statement(1)];

        assert_eq!(results[0], results[1]);
        assert_eq!(results[0], DEFAULT_INSERT);
    }

    #[test]
    fn build_insert_statement_returns_n_parameters_when_input_is_n() {
        let result = build_insert_statement(3);

        assert_eq!(
            format!("{0}{1}{2}", DEFAULT_INSERT, ", (?,?)", ", (?,?)"),
            result
        );
    }
}
