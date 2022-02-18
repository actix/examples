# Handlebars

This is an example of how to use Actix Web with the [Handlebars](https://crates.io/crates/handlebars) templating language, which is currently the most popular crate that achieves this.

## Usage

```sh
cd templating/handlebars
cargo run
```

After starting the server, you may visit the following pages:

- http://localhost:8080
- http://localhost:8080/Emma/documents
- http://localhost:8080/Bob/passwords
- http://localhost:8080/some-non-existing-page - 404 error rendered using template
