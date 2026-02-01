# HTTPS Server (using Rustls)

## Usage

### Certificate

We put the self-signed certificate in this directory as an example but your browser would complain that it isn't secure. So we recommend to use [`mkcert`] to trust it. To use local CA, you should run:

```sh
mkcert -install
```

If you want to generate your own cert/private key file, then run:

```sh
mkcert -key-file key.pem -cert-file cert.pem 127.0.0.1 localhost
```

For `rsa` keys use `rsa_private_keys` function instead `pkcs8_private_keys`

```rs
let mut keys = pkcs8_private_keys(key_file).unwrap(); // pkcs8
let mut keys = rsa_private_keys(key_file).unwrap(); // rsa
```

[`mkcert`]: https://github.com/FiloSottile/mkcert

### Running the Example Server

```sh
cd https-tls/rustls
cargo run # (or ``cargo watch -x run``)
# Started http server: 127.0.0.1:8443
```

If you prefer reloading you can substitute `cargo watch -x run`. That requires you install the `cargo-watch` crate.

### web client

- curl: `curl -v https://127.0.0.1:8443/index.html --compressed -k`
- browser: [https://127.0.0.1:8443/index.html](https://127.0.0.1:8443/index.html)
