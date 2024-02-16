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
[`mkcert`]: https://github.com/FiloSottile/mkcert

### Running the Example Server

```sh
cd rustls v0.22
cargo run
# Started http server: 127.0.0.1:8080
```
### web client

- curl: `curl -v https://127.0.0.1:8080/hello/rustls --compressed -k`
- browser: [https://127.0.0.1:8080/hello/rustls](https://127.0.0.1:8080/hello/rustls)
