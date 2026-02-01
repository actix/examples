use juniper::GraphQLInputObject;
use mysql::{Error as DBError, Row, from_row, params, prelude::*};

use crate::schemas::{root::Context, user::User};

/// Product
#[derive(Default, Debug)]
pub struct Product {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub price: f64,
}

impl Product {
    pub(crate) fn from_row(row: Row) -> Self {
        let (id, user_id, name, price) = from_row(row);

        Self {
            id,
            user_id,
            name,
            price,
        }
    }
}

#[juniper::graphql_object(Context = Context)]
impl Product {
    fn id(&self) -> &str {
        &self.id
    }
    fn user_id(&self) -> &str {
        &self.user_id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn price(&self) -> f64 {
        self.price
    }

    fn user(&self, context: &Context) -> Option<User> {
        let mut conn = context.db_pool.get().unwrap();
        let user: Result<Option<Row>, DBError> = conn.exec_first(
            "SELECT * FROM user WHERE id=:id",
            params! {"id" => &self.user_id},
        );
        if let Err(_err) = user {
            None
        } else {
            let (id, name, email) = from_row(user.unwrap().unwrap());
            Some(User { id, name, email })
        }
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Product Input")]
pub struct ProductInput {
    pub user_id: String,
    pub name: String,
    pub price: f64,
}
