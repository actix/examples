# tls example

## Usage

### Certificate

We put the self-signed certificate in this direcotry as an example
but your browser would complain that it isn't secure.
So we recommend to use [`mkcert`] to trust it. To use local CA, you should run:

```bash
mkcert -install
```

If you want to generate your own cert/private key file, then run:

```bash
mkcert 127.0.0.1
```

If your key doesn't work, convert it to rsa:
```bash
openssl rsa -in key.pem -out key-rsa.pem
```

[`mkcert`]: https://github.com/FiloSottile/mkcert

### server

```bash
cd examples/rustls
cargo run (or ``cargo watch -x run``)
# Started http server: 127.0.0.1:8443
```

### web client

- curl: ``curl -v https://127.0.0.1:8443/index.html --compressed -k``
- browser: [https://127.0.0.1:8443/index.html](https://127.0.0.1:8443/index.html)
