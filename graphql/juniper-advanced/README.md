# GraphQL using Juniper and MySQL

GraphQL Implementation in Rust using Actix, Juniper, and MySQL as Database

## Prerequisites

- MySQL server

## Database Configuration

Create a new database for this project, and import the existing database schema has been provided named `mysql-schema.sql`.

Create `.env` file on the root directory of this project and set environment variable named `DATABASE_URL`, the example file has been provided named `.env.example`, you can see the format in there.

```sh
cat mysql-schema.sql | mysql -u root -D graphql_testing
```

## Usage

```sh
cd graphql/juniper-advanced
cp .env.example .env
# edit .env and insert your DB credentials
cargo run
```

## GraphQL Playground

GraphQL provides its own documentation. Click the "docs" link in the top right of the GraphiQL UI to see what types of queries and mutations are possible.

```
http://localhost:8080/graphiql
```
