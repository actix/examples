# Todo

A simple Todo project using a SQLite database.

## Prerequisites

- SQLite 3

## Change Into The Examples Workspace Root Directory

All instructions assume you have changed into the examples workspace root:

```console
$ pwd
.../examples
```

## Set Up The Database

Install the [sqlx](https://github.com/launchbadge/sqlx/tree/HEAD/sqlx-cli) command-line tool with the required features:

```sh
cargo install sqlx-cli --no-default-features --features=rustls,sqlite
```

Then to create and set-up the database run:

```sh
sqlx database create --database-url=sqlite://./todo.db
sqlx migrate run --database-url=sqlite://./todo.db
```

## Run The Application

Start the application with:

```sh
cargo run --bin=todo
```

The app will be viewable in the browser at <http://localhost:8080>.

## Modifying The Example Database

For simplicity, this example uses SQLx's offline mode. If you make any changes to the database schema, this must be turned off or the `sqlx-data.json` file regenerated using the following command:

```sh
cargo sqlx prepare --database-url=sqlite://./todo.db
```
