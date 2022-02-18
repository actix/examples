# tls example

## Usage

### Certificate

We put the self-signed certificate in this directory as an example
but your browser would complain that it isn't secure.
So we recommend to use [`mkcert`] to trust it. To use local CA, you should run:

```sh
mkcert -install
```

If you want to generate your own cert/private key file, then run:

```sh
mkcert 127.0.0.1
```

[`mkcert`]: https://github.com/FiloSottile/mkcert

### server

```sh
cd security/openssl
cargo run (or ``cargo watch -x run``)
# Started http server: 127.0.0.1:8443
```

### web client

- curl: ``curl -v https://127.0.0.1:8443/index.html --compressed -k``
- browser: [https://127.0.0.1:8443/index.html](https://127.0.0.1:8443/index.html)
