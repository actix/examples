use mysql::{from_row, params, Error as DBError, Row};

use crate::schemas::root::Context;
use crate::schemas::user::User;

/// Product
#[derive(Default, Debug)]
pub struct Product {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub price: f64,
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
        let mut conn = context.dbpool.get().unwrap();
        let user: Result<Option<Row>, DBError> = conn.first_exec(
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
