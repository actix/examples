pub trait Table<T> {
    fn create_table(&self) -> Result<u64,sqlx::Error>;
    fn drop_table(&self) -> Result<u64,sqlx::Error>;
    fn get_by_id(&self, id: (u64,&str)) -> Result<u64,sqlx::Error>;
}