use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use uuid;

use models;
use schema;

pub fn create_user(
    conn: &SqliteConnection, name_: String,
) -> Result<models::User, Error> {
    use self::schema::users::dsl::*;

    let uuid = format!("{}", uuid::Uuid::new_v4());
    let new_user = models::NewUser {
        id: &uuid,
        name: &name_,
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    let mut items = users.filter(id.eq(&uuid)).load::<models::User>(conn)?;

    Ok(items.pop().unwrap())
}
