# HTTPS Server With TLS Cert/Key File Watcher

## Usage

```sh
cd https-tls/cert-watch
```

### Certificate

The included certificate is for demonstration. To create a certificate trusted by your browser, install [`mkcert`], install its local CA, and generate a new certificate and private key in this directory:

```sh
mkcert -install
mkcert -key-file key.pem -cert-file cert.pem 127.0.0.1 localhost
```

### Server

```sh
cargo run
```

Reload the server by modifying the certificate metadata:

```sh
touch cert.pem
```

### Client

- [HTTPie]: `http --verify=no https://localhost:8443/`
- curl: `curl -v --insecure https://127.0.0.1:8443`
- Browser: navigate to <https://127.0.0.1:8443>

[`mkcert`]: https://github.com/FiloSottile/mkcert
[HTTPie]: https://httpie.io/cli
