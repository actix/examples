use super::{randomize_string, init_db_context};
use sqlx;
use sqlx_user_crud::dao::db_context::{DbSet, DbContext};
use sqlx_user_crud::model::{Group, User};
use uuid::Uuid;

#[actix_rt::test]
async fn add_user_groups_returns_1_when_user_is_associated_with_group() -> Result<(),sqlx::Error> {
    let db = init_db_context().await;

    let user = User {
        id: Uuid::new_v4().to_string(),
        name: randomize_string("alice"),
        email: randomize_string("alice@email.com"),
        groups: Vec::with_capacity(0),
    };

    let group = randomize_string("user");

    let _ = db.users.add_user(&user).await?;
    let _ = db.groups.add_group(&group).await?;

    let group = db.groups.get_group_by_name(&group).await?;
    let groups = vec![group];

    let result = db.users_to_groups.add_user_groups(&user.id
                                                    , &groups).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(1, result);
    Ok(())
}

#[actix_rt::test]
async fn add_user_groups_returns_3_when_user_is_associated_with_3_groups() -> Result<(),sqlx::Error> {
    let db = init_db_context().await;

    let user = User {
        id: Uuid::new_v4().to_string(),
        name: randomize_string("bob"),
        email: randomize_string("bob@email.com"),
        groups: Vec::with_capacity(0),
    };

    let group_names = vec![randomize_string("engineer")
        , randomize_string("architect")
        , randomize_string("tester")];

    let _ = db.users.add_user(&user).await?;
    for group_name in group_names.iter() {
        let _ = db.groups.add_group(group_name).await?;
    }

    let mut groups = Vec::with_capacity(3);
    for group_name in group_names.iter() {
        let group = db.groups.get_group_by_name(group_name).await?;
        groups.push(group);
    }

    let result = db.users_to_groups.add_user_groups(&user.id
                                                    , &groups).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(3, result);
    Ok(())
}

#[actix_rt::test]
async fn add_user_groups_returns_err_when_user_does_not_exist() -> Result<(),sqlx::Error> {
    let db = init_db_context().await;

    let user = User {
        id: Uuid::new_v4().to_string(),
        name: randomize_string("charlie"),
        email: randomize_string("charlie@email.com"),
        groups: Vec::with_capacity(0),
    };
    let groups = vec![Group {
        id: 0,
        name: String::from("non-existent"),
    }];

    let _ = db.users.add_user(&user).await?;


    let result = db.users_to_groups.add_user_groups(&user.id
                                                    , &groups).await;
    assert!(result.is_err());
    Ok(())
}