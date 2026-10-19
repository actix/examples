# Todo

A simple Todo project using a SQLite database.

## Prerequisites

- SQLite 3

## Usage

Keep this directory and these environment variables for all commands below:

```sh
cd basics/todo
export DATABASE_URL=sqlite://./todo.db
export SQLX_OFFLINE=false
```

### Database Setup

Install the [sqlx](https://github.com/launchbadge/sqlx/tree/HEAD/sqlx-cli) command-line tool with the required features:

```sh
cargo install sqlx-cli --no-default-features --features=rustls,sqlite
```

Create the database and apply the migrations:

```sh
sqlx database create
sqlx migrate run
```

### Server

Start the application with:

```sh
cargo run
```

Open <http://localhost:8080> in your browser.

## Modifying The Example Database

The commands above disable SQLx offline mode so the query macros can check the local database. After a schema change, apply the migrations and regenerate the query metadata:

```sh
sqlx migrate run
cargo sqlx prepare
```

SQLx writes the metadata to `.sqlx`. Set `SQLX_OFFLINE=true` only after this metadata exists and matches the schema.
