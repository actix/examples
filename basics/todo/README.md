# Todo

A simple Todo project using a SQLite database.

## Prerequisites

- SQLite 3

## Change Into This Project Sub Directory

All instructions assume you have changed into this folder:

```sh
cd basics/todo
```

## Set Up The Database

Install the [sqlx](https://github.com/launchbadge/sqlx/tree/HEAD/sqlx-cli) command-line tool with the required features:

```sh
cargo install sqlx-cli --no-default-features --features=rustls,sqlite
```

Then to create and set-up the database run:

```sh
sqlx database create
sqlx migrate run
```

## Run The Application

Start the application with:

```sh
cargo run
```

The app will be viewable in the browser at <http://localhost:8080>.

## Modifying The Example Database

For simplicity, this example uses SQLx's offline mode. If you make any changes to the database schema, this must be turned off or the `sqlx-data.json` file regenerated using the following command:

```sh
DATABASE_URL="sqlite://$(pwd)/todo.db" cargo sqlx prepare
```
