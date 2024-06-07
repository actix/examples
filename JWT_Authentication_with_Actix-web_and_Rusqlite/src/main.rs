use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtGen {
    username: String,
    exp: i64,
}

const SEC_KEY: &[u8] = b"my_very_secret_key_def_not_known";

async fn register(user: web::Json<User>) -> impl Responder {
    if let Err(err) = db_reg(&user) {
        return HttpResponse::InternalServerError().body(err.to_string());
    }
    HttpResponse::Ok().body("User registered successfully")
}

async fn login(user: web::Json<User>) -> impl Responder {
    if let Err(_) = authenticate_user(&user).await {
        return HttpResponse::Unauthorized().body("Invalid username or password");
    }

    match generate_token(&user.username) {
        Ok(token) => HttpResponse::Ok().body(token),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

async fn protected(req: HttpRequest) -> impl Responder {
    if let Some(token) = req
        .headers()
        .get("jwt")
        .and_then(|value| value.to_str().ok())
    {
        if let Ok(token_data) = decode::<JwtGen>(
            token,
            &DecodingKey::from_secret(SEC_KEY),
            &Validation::new(Algorithm::HS256),
        ) {
            if token_data.claims.exp < Utc::now().timestamp() {
                return HttpResponse::Unauthorized().body("Token expired");
            }

            return HttpResponse::Ok().body("Welcome to the protected route");
        }
    }

    HttpResponse::Unauthorized().body("Missing or invalid JWT token in the 'jwt' header")
}

async fn unprotected() -> impl Responder {
    "Unprotected endpoint (does not require authentication)"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/protected", web::get().to(protected))
            .route("/unprotected", web::get().to(unprotected))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn db_reg(user: &User) -> Result<()> {
    let conn = Connection::open("users.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
                  id INTEGER PRIMARY KEY,
                  username TEXT NOT NULL UNIQUE,
                  password TEXT NOT NULL
                  )",
        [],
    )?;

    conn.execute(
        "INSERT INTO users (username, password) VALUES (?1, ?2)",
        &[&user.username, &user.password],
    )?;

    Ok(())
}

async fn authenticate_user(user: &User) -> Result<(), ()> {
    let conn = Connection::open("users.db").map_err(|_| ())?;
    let mut stmt = conn
        .prepare("SELECT * FROM users WHERE username = ?1")
        .map_err(|_| ())?;
    let mut rows = stmt.query(&[&user.username]).map_err(|_| ())?;

    if let Some(row) = rows.next().map_err(|_| ())? {
        let stored_password: String = row.get(2).map_err(|_| ())?;
        if stored_password != user.password {
            return Err(());
        }
    } else {
        return Err(());
    }
    Ok(())
}

fn generate_token(username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = Utc::now() + Duration::hours(2); // Set expiration time to 2 hours from now

    let claims = JwtGen {
        username: username.to_owned(),
        exp: exp.timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SEC_KEY),
    )?;

    Ok(token)
}