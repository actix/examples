# Handlebars

This is an example of how to use Actix Web with the [Handlebars templating language](https://crates.io/crates/handlebars), which is currently the most popular crate that achieves this. After starting the server with `cargo run`, you may visit the following pages:

```bash
cd template_engines/handlebars
cargo run
```

- http://localhost:8080
- http://localhost:8080/Emma/documents
- http://localhost:8080/Bob/passwords
- http://localhost:8080/some-non-existing-page - 404 error rendered using template
