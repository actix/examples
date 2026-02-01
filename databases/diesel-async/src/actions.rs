use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::{NoContext, Timestamp, Uuid};

use crate::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;

// /// Run query using Diesel to find item by uid and return it.
pub async fn find_item_by_id(
    conn: &mut AsyncPgConnection,
    uid: Uuid,
) -> Result<Option<models::Item>, DbError> {
    use super::schema::items::dsl::*;

    let item = items
        .filter(id.eq(uid))
        .select(models::Item::as_select())
        // execute the query via the provided async `diesel_async::RunQueryDsl`
        .first::<models::Item>(conn)
        .await
        .optional()?;

    Ok(item)
}

/// Run query using Diesel to insert a new database row and return the result.
pub async fn insert_new_item(
    conn: &mut AsyncPgConnection,
    nm: &str, // prevent collision with `name` column imported inside the function
) -> Result<models::Item, DbError> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::items::dsl::*;

    let new_item = models::Item {
        id: Uuid::new_v7(Timestamp::now(NoContext)),
        name: nm.to_owned(),
    };

    let item = diesel::insert_into(items)
        .values(&new_item)
        .returning(models::Item::as_returning())
        .get_result(conn)
        .await
        .expect("Error inserting person");

    Ok(item)
}
