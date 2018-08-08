# tls example

## Usage

### server

```bash
cd examples/rustls
cargo run (or ``cargo watch -x run``)
# Started http server: 127.0.0.1:8443
```

### web client

- curl: ``curl -v https://127.0.0.1:8443/index.html --compressed -k``
- browser: [https://127.0.0.1:8443/index.html](https://127.0.0.1:8443/index.html)
