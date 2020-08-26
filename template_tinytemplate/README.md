# template_tinytemplate

See the documentation for the minimalist template engine [tiny_template](https://docs.rs/tinytemplate/1.1.0/tinytemplate/).

## Usage

### server

```bash
cd examples/template_tinytemplate
cargo run (or ``cargo watch -x run``)
# Started http server: 127.0.0.1:8080
```

### web client

- [http://localhost:8080](http://localhost:8080)
- [http://localhost:8080/non-existing-page](http://localhost:8080/non-existing-page) - 404 page rendered using template
