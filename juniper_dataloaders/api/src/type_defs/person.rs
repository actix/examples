use super::cults::Cult;
use crate::graphql::schema::Context;
use juniper;
use juniper::FieldResult;

#[derive(Debug, Clone)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub cult: Option<i32>,
}

#[derive(juniper::GraphQLInputObject, Debug, Clone)]
#[graphql(name = "NewPerson", description = "A creating a person!")]
pub struct NewPerson {
    pub name: String,
    pub cult: Option<i32>,
}

#[juniper::graphql_object(Context = Context)]
impl Person {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub async fn cult(&self, ctx: &Context) -> FieldResult<Option<Cult>> {
        match self.cult {
            Some(cult_id) => Ok(Some(ctx.cult_data.cult_by_id(cult_id).await)),
            None => Ok(None), /* ## If I had wanted it to error instead
                              ```rust
                              None => Err(FieldError::new(
                                "No cult",
                                graphql_value!({ "internal_error": "No cult found" }),
                              )),
                              ```
                              */
        }
    }
}
