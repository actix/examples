# actix-sqlx-todo

Example Todo application using Actix-web and [SQLx](https://github.com/launchbadge/sqlx) with posgresql

# Usage

## Prerequisites

* Rust
* PostgreSQL

## Change into the project sub-directory

All instructions assume you have changed into this folder:

```bash
cd examples/sqlx_todo
```

## Set up the database

* Create new database using `schema.sql`
* Copy `.env-example` into `.env` and adjust DATABASE_URL to match your PostgreSQL address, username and password 

## Run the application

To run the application execute:

```bash
cargo run
```

By default application will be available on `http://localhost:5000`. If you wish to change address or port you can do it insde `.env` file.