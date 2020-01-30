use super::schema::Context;
use crate::type_defs::{Cult, NewCult, NewPerson, Person};
use juniper::FieldResult;

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    pub async fn create_person(ctx: &Context, data: NewPerson) -> FieldResult<Person> {
        Ok(ctx.person_data.create_person(data).await)
    }
    pub async fn create_cult(ctx: &Context, data: NewCult) -> FieldResult<Cult> {
        Ok(ctx.cult_data.create_cult(data).await)
    }
}
