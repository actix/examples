use sqlx::{MySqlPool, FromRow};
use super::{User, Group, UserToGroup};
use std::sync::Arc;
use sqlx::mysql::{MySqlRow};

pub struct DbSet<'c, T> where T : FromRow<'c, MySqlRow<'c>> {
    pub pool: Arc<MySqlPool>,
    from_row: fn(&MySqlRow<'c>) -> Result<T,sqlx::Error>,
}

impl<'c, T> DbSet<'c, T> where T : FromRow<'c, MySqlRow<'c>> {
    fn new(pool: Arc<MySqlPool>) -> Self {
        DbSet {
            pool,
            from_row: T::from_row,
        }
    }
}

pub struct DbContext<'c> {
    pub users: Arc<DbSet<'c, User>>,
    pub groups: Arc<DbSet<'c, Group>>,
    pub users_to_groups: Arc<DbSet<'c, UserToGroup>>
}

impl DbContext<'_> {
    pub async fn new(sql_url: &str) -> DbContext<'_> {

        let pool = MySqlPool::new(sql_url).await.unwrap();
        let pool = Arc::new(pool);

        DbContext {
            users: Arc::from(DbSet::new(pool.clone())),
            groups: Arc::from( DbSet::new(pool.clone())),
            users_to_groups: Arc::from(DbSet::new(pool.clone()))
        }
    }
}