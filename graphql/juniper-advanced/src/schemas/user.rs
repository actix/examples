use juniper::{graphql_object, GraphQLInputObject};
use mysql::{from_row, params};

use crate::schemas::product::Product;
use crate::schemas::root::Context;

/// User
#[derive(Default, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "User Input")]
pub struct UserInput {
    pub name: String,
    pub email: String,
}

#[graphql_object(Context = Context)]
impl User {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn email(&self) -> &str {
        &self.email
    }

    fn products(&self, context: &Context) -> Vec<Product> {
        let mut conn = context.dbpool.get().unwrap();

        conn.prep_exec(
            "select * from product where user_id=:user_id",
            params! {
                "user_id" => &self.id
            },
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (id, user_id, name, price) = from_row(row);
                    Product {
                        id,
                        user_id,
                        name,
                        price,
                    }
                })
                .collect()
        })
        .unwrap()
    }
}
