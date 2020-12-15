use super::{randomize_string, init_db_context};
use sqlx;
use sqlx_user_crud::dao::db_context::{DbSet, DbContext};
use sqlx_user_crud::model::User;
use uuid::Uuid;

#[actix_rt::test]
async fn add_user_returns_1() -> Result<(),sqlx::Error> {
    let db = init_db_context().await;
    let user = User {
        id: Uuid::new_v4().to_string(),
        name: randomize_string("alice"),
        email: randomize_string("alice@email.com"),
        groups: Vec::with_capacity(0),
    };

    let result = db.users.add_user(&user).await;

    assert!(result.is_ok());
    assert_eq!(1, result.unwrap());

    Ok(())
}

#[actix_rt::test]
async fn add_user_returns_err_when_duplicate_username_is_added() -> Result<(),sqlx::Error> {
    let db = init_db_context().await;

    let name = randomize_string("bob");
    let email = randomize_string("bob@emai.com");

    let original = User {
        id: Uuid::new_v4().to_string(),
        name: name.clone(),
        email: email.clone(),
        groups: Vec::with_capacity(0),
    };

    let duplicate = User {
        id: Uuid::new_v4().to_string(),
        name: name.clone(),
        email: email.clone(),
        groups: Vec::with_capacity(0),
    };

    let result = db.users.add_user(&original).await?;
    assert_eq!(1, result);

    let result = db.users.add_user(&duplicate).await;
    assert!(result.is_err());

    Ok(())
}

#[actix_rt::test]
async fn get_user_by_id_returns_error_when_user_does_not_exist() -> () {
    let db = init_db_context().await;

    let id = Uuid::new_v4().to_string();

    let result = db.users.get_user_by_id(&id).await;
    assert!(result.is_err());
}

#[actix_rt::test]
async fn get_user_by_id_returns_user_when_user_exists() -> Result<(),sqlx::Error> {
    let db = init_db_context().await;

    let user  = User {
        id: Uuid::new_v4().to_string(),
        name: randomize_string("charlie"),
        email: randomize_string("charlie@email.com"),
        groups: Vec::with_capacity(0),
    };

    let _ = db.users.add_user(&user).await?;

    let result = db.users.get_user_by_id(&user.id).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(user.name, result.name);
    assert_eq!(user.email, result.email);
    Ok(())
}

#[actix_rt::test]
async fn update_user_returns_zero_when_user_does_not_exist() -> () {
    let db = init_db_context().await;

    let user  = User {
        id: Uuid::new_v4().to_string(),
        name: randomize_string("david"),
        email: randomize_string("david@email.com"),
        groups: Vec::with_capacity(0),
    };

    let result = db.users.update_user(&user).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(0, result);
}