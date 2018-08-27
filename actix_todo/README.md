# actix-todo

A port of the [Rocket Todo example](https://github.com/SergioBenitez/Rocket/tree/master/examples/todo) into [actix-web](https://actix.rs/). Except this uses PostgreSQL instead of SQLite.

# Usage

## Prerequisites

* Rust >= 1.26
* PostgreSQL >= 9.5

## Change into the project sub-directory

All instructions assume you have changed into this folder:

```bash
cd examples/actix_todo
```

## Set up the database

Install the [diesel](http://diesel.rs) command-line tool including the `postgres` feature:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

Check the contents of the `.env` file. If your database requires a password, update `DATABASE_URL` to be of the form:

```.env
DATABASE_URL=postgres://username:password@localhost/actix_todo
```

Then to create and set-up the database run:

```bash
diesel database setup
```

## Run the application

To run the application execute:

```bash
cargo run
```

Then to view it in your browser navigate to: [http://localhost:8088/](http://localhost:8088/)
