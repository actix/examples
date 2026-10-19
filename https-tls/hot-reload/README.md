# HTTPS Server With TLS Cert/Key Hot Reload

## Usage

```sh
cd https-tls/hot-reload
```

### Certificate

The included certificate is for demonstration. To create a certificate trusted by your browser, install [`mkcert`], install its local CA, and generate a new certificate and private key in this directory:

```sh
mkcert -install
mkcert -key-file key.pem -cert-file cert.pem 127.0.0.1 localhost
```

### Server

```sh
RUST_LOG=info,example=debug cargo run
```

Reload the server by modifying the certificate metadata:

```sh
touch cert.pem
```

For a deeper inspection, use a tool like [`inspect-cert-chain`] between refreshes of the cert/key files using [`mkcert`] as shown above:

```console
$ inspect-cert-chain --host=localhost --port=8443
...
Serial Number:
  06:81:db:16:ff:c4:73:69:73:69:ae:d1:0e:3d:d1:5e
...

$ mkcert -key-file key.pem -cert-file cert.pem 127.0.0.1 localhost
...

$ inspect-cert-chain --host=localhost --port=8443
...
Serial Number:
  00:a8:39:e7:aa:2e:73:18:f6:4e:d5:71:1e:c7:21:51:58
...
```

Observing a change in the serial number without restarting the server demonstrates that the setup works.

### Client

- [HTTPie]: `http --verify=no https://localhost:8443/`
- curl: `curl -v --insecure https://127.0.0.1:8443`
- Browser: navigate to <https://127.0.0.1:8443>

[`mkcert`]: https://github.com/FiloSottile/mkcert
[HTTPie]: https://httpie.io/cli
[`inspect-cert-chain`]: https://github.com/robjtede/inspect-cert-chain
