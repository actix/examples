# Fluent Templates

This is an example of how to integrate [Fluent Templates](https://crates.io/crates/fluent-templates) with Actix Web using the Handlebars templating engine.

## Directory Structure

- `src`: Rust source code for web server and endpoint configuration
- `templates`: Handlebars templates (no text content is stored in the templates)
- `locales`: Fluent files containing translations for English (en) and French (fr)

## Usage

```sh
cd templating/fluent
cargo run
```

After starting the server, you may visit the following pages:

- http://localhost:8080
- http://localhost:8080/Alice/documents
- http://localhost:8080/Bob/passwords
- http://localhost:8080/some-non-existing-page - 404 error rendered using template

This example selects a language from the `Accept-Language` header. Browsers usually set this header from the user's language settings. To select a translation, set `Accept-Language` to `en` or `fr`. Languages without translation files fall back to English.

```sh
# using HTTPie
http :8080/Alice/documents Accept-Language:fr

# using curl
curl http://localhost:8080/Alice/documents -H 'accept-language: fr'
```
