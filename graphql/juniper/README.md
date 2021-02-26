# Juniper

[Juniper](https://github.com/graphql-rust/juniper) integration for Actix web.
If you want more advanced example, see also the [juniper-advanced example].

[juniper-advanced example]: https://github.com/actix/examples/tree/master/juniper-advanced

## Usage

### server

```bash
cd examples/juniper
cargo run (or ``cargo watch -x run``)
# Started http server: 127.0.0.1:8080
```

### web client

[http://127.0.0.1:8080/graphiql](http://127.0.0.1:8080/graphiql)

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
      "appearsIn": [
        "NEW_HOPE"
      ],
      "homePlanet": "Mars"
    }
  }
}
```

_Mutation example:_

```graphql
mutation {
  createHuman(newHuman: {name: "Fresh Kid Ice", appearsIn: EMPIRE, homePlanet: "earth"}) {
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
      "appearsIn": [
        "EMPIRE"
      ],
      "homePlanet": "earth"
    }
  }
}
```
