# Dockerize Graphql Rust

This project is currently for demonstrating the use of dataloaders in a rust-based graphql server.
This demo uses:

- `actix-web` a high-performance webserver for rust https://www.techempower.com/benchmarks/
- `juniper` a popular rust graphql framework https://github.com/graphql-rust/juniper
- A dockerfile which creates a _from scratch_ minimal sized container
- Cults (more to be added)


There are definitely some improvements to be made in db requests, pagination etc. Let me know if you spot anything you think is top-priority!

## Running locally

simple as `docker-compose up`, crazy!
Then navigate to http://localhost:8000/graphql

## TODO

- [x] DB connection
- [x] Expose graphql
- [ ] Mutations!
  - [x] Create
  - [ ] Update
  - [ ] Delete
- [ ] Context to later use in Dataloaders and Auth
- [x] Dataloaders
- [ ] Auth?
- [ ] DB pool

## Schema

```graphql
type Cult {
  id: Int!
  name: String!
  members: [Person!]!
}

type Mutation {
  createPerson(data: NewPerson!): Person!
  createCult(data: NewCult!): Cult!
}

input NewCult {
  name: String!
}

input NewPerson {
  name: String!
  cult: Int
}

type Person {
  id: Int!
  name: String!
  cult: Cult
}

type Query {
  personById(id: Int!): Person!
  persons: [Person!]!
  cultById(id: Int!): Cult!
  cults: [Cult!]!
}
```

## References

Original Rest API & DB connection is inspired by:
https://turreta.com/2019/09/21/rest-api-with-rust-actix-web-and-postgresql-part-3/

Graphql setup is inspired by:
https://www.freecodecamp.org/news/building-powerful-graphql-servers-with-rust/

Rust containerization initially inspired by:
https://alexbrand.dev/post/how-to-package-rust-applications-into-minimal-docker-containers/
