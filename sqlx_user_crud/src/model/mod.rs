mod db_context;
mod user;
mod group;

mod user_dao;
mod group_dao;

pub type DbContext<'c> = db_context::DbContext<'c>;
pub type DbSet<'c, T> = db_context::DbSet<'c, T>;

pub type User = user::User;
pub type Group = group::Group;