# HTTPS Server With TLS Cert/Key Hot Reload

## Usage

All documentation assumes your terminal is in this directly (`cd https-tls/hot-reload`).

### Certificate

We put the self-signed certificate in this directory as an example but your browser would complain that it isn't secure. So we recommend to use [`mkcert`] to trust it. To use local CA, you should run:

```shell
$ mkcert -install
```

If you want to generate your own cert/private key file, then run:

```shell
$ mkcert -key-file key.pem -cert-file cert.pem 127.0.0.1 localhost
```

### Running The Example Server

```shell
$ RUST_LOG=info,example=debug cargo run
Starting HTTPS server at https://localhost:8443
```

Reload the server by modifying the certificate metadata:

```shell
$ touch cert.pem
```

For a deeper inspection, use a tool like [`inspect-cert-chain`] between refreshes of the cert/key files using [`mkcert`] as shown above:

```shell
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

- [HTTPie]: `http --verify=no :8443`
- cURL: `curl -v --insecure https://127.0.0.1:8443`
- Browser: navigate to <https://127.0.0.1:8443>

[`mkcert`]: https://github.com/FiloSottile/mkcert
[httpie]: https://httpie.io/cli
[`inspect-cert-chain`]: https://github.com/robjtede/inspect-cert-chain
