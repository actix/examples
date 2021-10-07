Getting started using [Async-graphql](https://github.com/async-graphql/async-graphql) with Actix web.

## Run

```bash
cd graphql/graphql-demo
cargo run --bin async-graphql-demo
```

## Endpoints

    GET http://127.0.0.1:8000/      GraphQL Playground UI
    POST http://127.0.0.1:8000/     For GraphQL query

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