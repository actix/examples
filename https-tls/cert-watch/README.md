# HTTPS Server With TLS Cert/Key File Watcher

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

### Running The Example Server

```console
$ cd https-tls/cert-watch
$ cargo run
starting HTTPS server at https://localhost:8443
```

Reload the server by modifying the certificate metadata:

```console
$ touch cert.pem
```

### Client

- [HTTPie]: `http --verify=no :8443`
- cURL: `curl -v --insecure https://127.0.0.1:8443`
- Browser: navigate to <https://127.0.0.1:8443>

[`mkcert`]: https://github.com/FiloSottile/mkcert
[httpie]: https://httpie.io/cli
