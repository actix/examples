use super::DbSet;
use super::UserToGroup;
use super::Group;
use super::User;

impl<'c> DbSet<'c, UserToGroup> {

    pub async fn create_table(&self) -> Result<u64,sqlx::Error> {
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS `users_to_groups`
            (
                `user_id` VARCHAR(48) NOT NULL REFERENCES `users`(`id`),
                `group_id` INT NOT NULL REFERENCES `groups`(`id`)
            )
        "#).execute(&*self.pool).await
    }

    pub async fn drop_table(&self) -> Result<u64,sqlx::Error> {
        sqlx::query("DROP TABLE IF EXISTS users_to_groups")
            .execute(&*self.pool).await
    }

    pub async fn add_user_groups(&self, user_id: &String, groups: &Vec<Group>) -> Result<u64,sqlx::Error>
    {
        let insert_statement = self.build_insert_statement(groups.len());
        let mut query = sqlx::query(&insert_statement);

        for group in groups {
            query = query.bind(user_id).bind(group.id)
        }

        query.execute(&*self.pool).await
    }

    fn build_insert_statement(&self, rows: usize) -> String {
        let mut insert = String::from(r#"
            INSERT INTO `users_to_groups` (`user_id`, `group_id`)
            VALUES (?,?)
        "#);

        match rows {
            1|0 => insert,
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
}