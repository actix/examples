#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod models;
mod schema;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize)]
struct MyUser {
    name: String,
}

fn query(
    nm: String,
    pool: web::Data<Pool>,
) -> Result<models::User, diesel::result::Error> {
    use self::schema::users::dsl::*;

    let uuid = format!("{}", uuid::Uuid::new_v4());
    let new_user = models::NewUser {
        id: &uuid,
        name: nm.as_str(),
    };
    let conn: &PgConnection = &pool.get().unwrap();

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    let mut items = users.filter(id.eq(&uuid)).load::<models::User>(conn)?;
    Ok(items.pop().unwrap())
}

async fn insert(
    item: web::Json<MyUser>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || query(item.into_inner().name, pool))
        .await
        .map_err(|_| HttpResponse::InternalServerError())?;

    Ok(HttpResponse::Ok().json(user))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/")
                    .data(
                        web::JsonConfig::default()
                    )
                    .route(web::post().to(insert)),
            )
    })
    .bind("0.0.0.0:3000")?
    .start()
    .await
}
