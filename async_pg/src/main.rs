
mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "users")] // singular 'user' is a keyword..
    pub struct User {
        pub email: String,
        pub first_name: String,
        pub last_name: String,
        pub username: String
    }
}


mod errors {
	use actix_web::{HttpResponse, ResponseError};
	use derive_more::{Display, From};
	use deadpool_postgres::PoolError;
	use tokio_postgres::error::Error as PGError;
	use tokio_pg_mapper::Error as PGMError;


	#[derive(Display, From, Debug)]
	pub enum MyError {
		NotFound,
		PGError(PGError),
		PGMError(PGMError),
		PoolError(PoolError)
	}
	impl std::error::Error for MyError {}

	impl ResponseError for MyError {
		fn error_response(&self) -> HttpResponse {
			match *self {
				MyError::NotFound => HttpResponse::NotFound().finish(),
				_ => HttpResponse::InternalServerError().finish()
			}	
		}
	}
}


mod db {
	use crate::{errors::MyError, models::User};
	use deadpool_postgres::Client;
	use tokio_pg_mapper::FromTokioPostgresRow;


	pub async fn add_user(client: &Client, user_info: User) -> Result<User, MyError> {
		let _stmt = include_str!("../sql/add_user.sql");
		let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
		let stmt = client.prepare(&_stmt)
						 .await
						 .unwrap();

			client.query(&stmt, 
					 &[&user_info.email,
					   &user_info.first_name,
					   &user_info.last_name,
                       &user_info.username
                       ])
				  .await?
				  .iter()
				  .map(|row| User::from_row_ref(row).unwrap())
				  .collect::<Vec<User>>()
				  .pop()
				  .ok_or(MyError::NotFound) // more applicable for SELECTs
	}
}


mod handlers {
	use actix_web::{HttpResponse, web, Error};
	use deadpool_postgres::{Client, Pool};
    use crate::{db, errors::MyError, models::User};


	pub async fn add_user(user: web::Json<User>, db_pool: web::Data<Pool>)
						-> Result<HttpResponse, Error> {

		let user_info: User = user.into_inner();

		let client: Client = 
            db_pool.get()
                   .await
                   .map_err(|err| MyError::PoolError(err))?;

		let new_user = db::add_user(&client, user_info).await?;
		
		Ok(HttpResponse::Ok().json(new_user))
	}
}


use actix_web::{App, HttpServer, web};
use deadpool_postgres::{Pool, Manager};
use handlers::add_user;
use tokio_postgres::{Config, NoTls};


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    const SERVER_ADDR: &str = "127.0.0.1:8080";

	let pg_config = "postgres://test_user:testing@127.0.0.1:5432/testing_db"
					.parse::<Config>()
					.unwrap();

	let pool = Pool::new(
        Manager::new(pg_config, NoTls),
		16 // # of connections in pool
    );


    let server = HttpServer::new(move || 
			App::new()
				.data(pool.clone())
				.service(web::resource("/users").route(web::post().to(add_user))) 
		)
        .bind(SERVER_ADDR)?
        .run();
    println!("Server running at http://{}/", SERVER_ADDR);

    server.await
}
