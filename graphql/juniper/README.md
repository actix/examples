# GraphQL using Juniper

[Juniper](https://github.com/graphql-rust/juniper) integration for Actix Web. If you want more advanced example, see also the [juniper-advanced example].

[juniper-advanced example]: https://github.com/actix/examples/tree/master/graphql/juniper-advanced

## Usage

### Server

```sh
cd graphql/juniper
cargo run
```

### Web Client

Go to <http://localhost:8080/graphiql> in your browser.

_Query example:_

```graphql
{
  human(id: "1234") {
    name
    appearsIn
    homePlanet
  }
}
```

_Result:_

```json
{
  "data": {
    "human": {
      "name": "Luke",
      "appearsIn": ["NEW_HOPE"],
      "homePlanet": "Mars"
    }
  }
}
```

_Mutation example:_

```graphql
mutation {
  createHuman(newHuman: { name: "Fresh Kid Ice", appearsIn: EMPIRE, homePlanet: "earth" }) {
    id
    name
    appearsIn
    homePlanet
  }
}
```

_Result:_

```json
{
  "data": {
    "createHuman": {
      "id": "1234",
      "name": "Fresh Kid Ice",
      "appearsIn": ["EMPIRE"],
      "homePlanet": "earth"
    }
  }
}
```
