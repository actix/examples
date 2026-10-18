# Authentication Server

Demonstrates email invitations, registration, and cookie-based authentication with Actix Web and PostgreSQL.

## Prerequisites

- PostgreSQL and its client development libraries.
- A SparkPost API key and a verified sender email address.

On Ubuntu, install the build dependencies:

```sh
sudo apt install libclang-dev libpq-dev
```

## Usage

```sh
cd auth/simple-auth-server
cargo install diesel_cli --no-default-features --features postgres
```

Create `.env` in this directory. Set these values for your PostgreSQL server and SparkPost account:

```ini
DATABASE_URL=postgres://localhost/simple_auth
SPARKPOST_API_KEY=your-api-key
SENDING_EMAIL_ADDRESS=sender@example.com
DOMAIN=localhost
```

Create the database and apply the migrations, then run the server:

```sh
diesel setup
cargo run
```

Use an HTTP client to call the API at `http://localhost:8080`. The registration flow also needs a frontend that handles invitation links; see the tutorial below for the complete application.

## Authentication Flow

1. Submit an email address to receive an invitation link.
1. Use the invitation ID to register with the same email address and a password.
1. Log in with that email address and password to receive an authentication cookie.

## Routes

- [POST /api/invitation](http://localhost:8080/api/invitation)
- `POST /api/register/{invitation_id}`
- [GET /api/auth](http://localhost:8080/api/auth)
- [POST /api/auth](http://localhost:8080/api/auth)
- [DELETE /api/auth](http://localhost:8080/api/auth)

## Crates Used

- [actix-web](https://crates.io/crates/actix-web) // Actix Web is a simple, pragmatic and extremely fast web framework for Rust.
- [rust-argon2](https://crates.io/crates/rust-argon2) // crate for hashing passwords using the cryptographically-secure Argon2 hashing algorithm.
- [chrono](https://crates.io/crates/chrono) // Date and time library for Rust.
- [diesel](https://crates.io/crates/diesel) // A safe, extensible ORM and Query Builder for PostgreSQL, SQLite, and MySQL.
- [dotenvor](https://crates.io/crates/dotenvor) // A well-maintained dotenv implementation for Rust.
- [derive_more](https://crates.io/crates/derive_more) // Convenience macros to derive traits easily
- [env_logger](https://crates.io/crates/env_logger) // A logging implementation for log which is configured via an environment variable.
- [once_cell](https://crates.io/crates/once_cell) // Lazy initialization of shared values.
- [r2d2](https://crates.io/crates/r2d2) // A generic connection pool.
- [serde](https://crates.io/crates/serde) // A generic serialization/deserialization framework.
- [serde_json](https://crates.io/crates/serde_json) // A JSON serialization file format.
- [sparklepost](https://crates.io/crates/sparklepost) // Rust bindings for Sparkpost email API.
- [uuid](https://crates.io/crates/uuid) // A library to generate and parse UUIDs.

Read the full tutorial series on [gill.net.in](https://gill.net.in)

- [Auth Web Microservice with Rust using Actix Web v4 - Complete Tutorial](https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/)
