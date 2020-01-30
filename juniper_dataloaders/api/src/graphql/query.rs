pub struct Query;
use super::schema::Context;
use crate::type_defs::{Cult, Person};
use juniper::FieldResult;

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn person_by_id(context: &Context, id: i32) -> FieldResult<Person> {
        Ok(context.person_data.person_by_id(id).await)
    }

    async fn persons(context: &Context) -> FieldResult<Vec<Person>> {
        Ok(context.person_data.get_all_persons().await)
    }

    async fn cult_by_id(context: &Context, id: i32) -> FieldResult<Cult> {
        Ok(context.cult_data.cult_by_id(id).await)
    }

    async fn cults(context: &Context) -> FieldResult<Vec<Cult>> {
        Ok(context.cult_data.get_all_cults().await)
    }
}
