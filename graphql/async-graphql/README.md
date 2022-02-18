## GraphQL using `async-graphql`

> Getting started using [async-graphql](https://github.com/async-graphql/async-graphql) with Actix Web.

## Usage

```sh
cd graphql/graphql-demo
cargo run
```

## Endpoints

```
GET/POST  http://localhost:8080/graphql   GraphQL endpoint
GET       http://localhost:8080/graphiql  GraphQL playground UI
```

## Query Examples

```graphql
{
  humans {
    edges {
      node {
        id
        name
        friends {
          id
          name
        }
        appearsIn
        homePlanet
      }
    }
  }
}
```
