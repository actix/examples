use super::model::{User, Group};

pub mod db_context;
mod group_dao;
mod user_dao;

pub type DbContext<'c> = db_context::DbContext<'c>;
pub type DbSet<'c, T> = db_context::DbSet<'c, T>;