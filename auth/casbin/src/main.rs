use std::io;

use actix_web::{App, HttpResponse, HttpServer, middleware, web};
use casbin::{CoreApi, DefaultModel, Enforcer, FileAdapter, RbacApi};

/// simple handle
async fn success(enforcer: web::Data<Enforcer>) -> HttpResponse {
    assert_eq!(
        vec!["data2_admin"],
        enforcer.get_roles_for_user("alice", None)
    );

    HttpResponse::Ok().body("Success: alice is data2_admin.")
}

async fn fail(enforcer: web::Data<Enforcer>) -> HttpResponse {
    assert_eq!(
        vec!["data1_admin"],
        enforcer.get_roles_for_user("alice", None)
    );

    HttpResponse::Ok().body("Fail: alice is not data1_admin.") // In fact, it can't be displayed.
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let model = DefaultModel::from_file("rbac/rbac_model.conf")
        .await
        .unwrap();
    let adapter = FileAdapter::new("rbac/rbac_policy.csv");

    let enforcer = Enforcer::new(model, adapter).await.unwrap();
    let enforcer = web::Data::new(enforcer);

    // move is necessary to give closure below ownership of data
    HttpServer::new(move || {
        App::new()
            .app_data(enforcer.clone()) // <- create app with shared data
            // register simple handler, handle all methods
            .service(web::resource("/success").to(success))
            .service(web::resource("/fail").to(fail))
            // enable logger
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
