# actix-sqlx-todo

Example Todo application using Actix-web and [SQLx](https://github.com/launchbadge/sqlx) with sqlite

# Usage

## Prerequisites

* Rust
* SQLite

## Change into the project sub-directory

All instructions assume you have changed into this directory:

```bash
$ cd database_interactions/sqlx_todo
```

## Set up the database

* Create new database using `schema.sql`
* Copy `.env.example` into `.env` and adjust `DATABASE_URL` to match your SQLite address, if needed

```sh
cat schema.sql | sqlite3 test.db
cp .env.example .env
```

## Run the application

To run the application execute:

```bash
cargo run
```

By default application will be available on `http://localhost:8080`. If you wish to change address or port you can do it inside the `.env` file
