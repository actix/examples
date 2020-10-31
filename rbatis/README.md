## Actix + Rbatis

How to use

- Make sure you already create table you desire on database, [here the migration example](./person.sql).
- set env var : DB_URL as database service url.
- `cargo test` then `cargo run` or `cargo build`
- Hit at `localhost:2121/person` for the result.

### Reference

- [Rbatis quick example](https://github.com/rbatis/rbatis#quick-example-querywrapper-and-common-usages-see-examplecrud_testrs-for-details)