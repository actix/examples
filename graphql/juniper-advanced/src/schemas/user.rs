use juniper::{GraphQLInputObject, graphql_object};
use mysql::{Row, from_row, params, prelude::*};

use crate::schemas::{product::Product, root::Context};

/// User
#[derive(Default, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl User {
    pub(crate) fn from_row(row: Row) -> Self {
        let (id, name, email) = from_row(row);
        User { id, name, email }
    }
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
        let mut conn = context.db_pool.get().unwrap();

        conn.exec(
            "SELECT * FROM product WHERE user_id = :user_id",
            params! { "user_id" => &self.id },
        )
        .unwrap()
        .into_iter()
        .map(Product::from_row)
        .collect()
    }
}
