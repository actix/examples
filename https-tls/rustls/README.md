# HTTPS Server Using Rustls

## Usage

```sh
cd https-tls/rustls
```

### Certificate

The included certificate is for demonstration. To create a certificate trusted by your browser, install [`mkcert`], install its local CA, and generate a new certificate and private key in this directory:

```sh
mkcert -install
mkcert -key-file key.pem -cert-file cert.pem 127.0.0.1 localhost
```

The example loads the private key with `PrivateKeyDer::from_pem_file`, which detects the PEM key format.

### Server

```sh
cargo run
```

If you prefer reloading you can substitute `cargo watch -x run`. That requires you install the `cargo-watch` crate.

### Browser Client

- curl: `curl -v https://127.0.0.1:8443/index.html --compressed -k`
- Browser: [https://127.0.0.1:8443/index.html](https://127.0.0.1:8443/index.html)

[`mkcert`]: https://github.com/FiloSottile/mkcert
