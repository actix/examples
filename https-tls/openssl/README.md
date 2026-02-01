# HTTPS Server using OpenSSL

## Usage

### Generating Trusted Certificate

We put self-signed certificate in this directory as an example but your browser will complain that connections to the server aren't secure. We recommend to use [`mkcert`] to trust it. To use a local CA, you should run:

```sh
mkcert -install
```

If you want to generate your own private key/certificate pair, then run:

```sh
mkcert -key-file key.pem -cert-file cert.pem 127.0.0.1 localhost
```

A new `key.pem` and `cert.pem` will be saved to the current directory. You will then need to modify `main.rs` where indicated.

### Running Server

```console
$ cd https-tls/openssl
$ cargo run # (or `cargo watch -x run`)
starting HTTPS server at 127.0.0.1:8443
```

### Using Client

- curl: `curl -vk https://127.0.0.1:8443`
- curl (forced HTTP/1.1): `curl -vk --http1.1 https://127.0.0.1:8443`
- browser: <https://127.0.0.1:8443>

## Self-Signed Encrypted Private Key Command

```sh
openssl req -x509 -newkey rsa:4096 -keyout key-pass.pem -out cert-pass.pem -sha256 -days 365
```

[`mkcert`]: https://github.com/FiloSottile/mkcert
