use juniper::{FieldError, FieldResult, RootNode};
use mysql::{from_row, params, Error as DBError, Row};

use crate::db::Pool;

use super::product::{Product, ProductInput};
use super::user::{User, UserInput};

pub struct Context {
    pub dbpool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all users")]
    fn users(context: &Context) -> FieldResult<Vec<User>> {
        let mut conn = context.dbpool.get().unwrap();
        let users = conn
            .prep_exec("select * from user", ())
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|mut row| {
                        let (id, name, email) = from_row(row);
                        User { id, name, email }
                    })
                    .collect()
            })
            .unwrap();
        Ok(users)
    }

    #[graphql(description = "Get Single user reference by user ID")]
    fn user(context: &Context, id: String) -> FieldResult<User> {
        let mut conn = context.dbpool.get().unwrap();

        let user: Result<Option<Row>, DBError> =
            conn.first_exec("SELECT * FROM user WHERE id=:id", params! {"id" => id});

        if let Err(err) = user {
            return Err(FieldError::new(
                "User Not Found",
                graphql_value!({ "not_found": "user not found" }),
            ));
        }

        let (id, name, email) = from_row(user.unwrap().unwrap());
        Ok(User { id, name, email })
    }

    #[graphql(description = "List of all users")]
    fn products(context: &Context) -> FieldResult<Vec<Product>> {
        let mut conn = context.dbpool.get().unwrap();
        let products = conn
            .prep_exec("select * from product", ())
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|mut row| {
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
            .unwrap();
        Ok(products)
    }

    #[graphql(description = "Get Single user reference by user ID")]
    fn product(context: &Context, id: String) -> FieldResult<Product> {
        let mut conn = context.dbpool.get().unwrap();
        let product: Result<Option<Row>, DBError> =
            conn.first_exec("SELECT * FROM user WHERE id=:id", params! {"id" => id});
        if let Err(err) = product {
            return Err(FieldError::new(
                "Product Not Found",
                graphql_value!({ "not_found": "product not found" }),
            ));
        }

        let (id, user_id, name, price) = from_row(product.unwrap().unwrap());
        Ok(Product {
            id,
            user_id,
            name,
            price,
        })
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_user(context: &Context, user: UserInput) -> FieldResult<User> {
        let mut conn = context.dbpool.get().unwrap();
        let new_id = uuid::Uuid::new_v4().to_simple().to_string();

        let insert: Result<Option<Row>, DBError> = conn.first_exec(
            "INSERT INTO user(id, name, email) VALUES(:id, :name, :email)",
            params! {
                "id" => &new_id,
                "name" => &user.name,
                "email" => &user.email,
            },
        );

        match insert {
            Ok(opt_row) => Ok(User {
                id: new_id,
                name: user.name,
                email: user.email,
            }),
            Err(err) => {
                let msg = match err {
                    DBError::MySqlError(err) => err.message,
                    _ => "internal error".to_owned(),
                };
                Err(FieldError::new(
                    "Failed to create new user",
                    graphql_value!({ "internal_error": msg }),
                ))
            }
        }
    }

    fn create_product(context: &Context, product: ProductInput) -> FieldResult<Product> {
        let mut conn = context.dbpool.get().unwrap();
        let new_id = uuid::Uuid::new_v4().to_simple().to_string();

        let insert: Result<Option<Row>, DBError> = conn.first_exec(
            "INSERT INTO product(id, user_id, name, price) VALUES(:id, :user_id, :name, :price)",
            params! {
                "id" => &new_id,
                "user_id" => &product.user_id,
                "name" => &product.name,
                "price" => &product.price.to_owned(),
            },
        );

        match insert {
            Ok(opt_row) => Ok(Product {
                id: new_id,
                user_id: product.user_id,
                name: product.name,
                price: product.price,
            }),
            Err(err) => {
                let msg = match err {
                    DBError::MySqlError(err) => err.message,
                    _ => "internal error".to_owned(),
                };
                Err(FieldError::new(
                    "Failed to create new product",
                    graphql_value!({ "internal_error": msg }),
                ))
            }
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
