# GraphQL Using Juniper And MySQL

GraphQL Implementation in Rust using Actix, Juniper, and MySQL as Database

## Prerequisites

- MySQL server

## Database Configuration

Start from the example directory:

```sh
cd graphql/juniper-advanced
```

Create a database named `graphql_testing` and import the provided `mysql-schema.sql` schema.

Create `.env` file on the root directory of this project and set environment variable named `DATABASE_URL`, the example file has been provided named `.env.example`, you can see the format in there.

```sh
mysql -u root -e 'CREATE DATABASE graphql_testing'
mysql -u root -D graphql_testing < mysql-schema.sql
```

## Usage

```sh
cp .env.example .env
# edit .env and insert your DB credentials
cargo run
```

## GraphQL Playground

GraphQL provides its own documentation. Click the "docs" link in the top right of the GraphiQL UI to see what types of queries and mutations are possible.

```text
http://localhost:8080/graphiql
```
