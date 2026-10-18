# Middleware: Redirect HTTP To HTTPS

## Alternatives

For a reusable middleware, see [`RedirectHttps`](https://docs.rs/actix-web-lab/latest/actix_web_lab/middleware/struct.RedirectHttps.html) from [`actix-web-lab`](https://crates.io/crates/actix-web-lab).

## This Example

Extends the [Rustls example](../../https-tls/rustls/README.md) by listening on both HTTP port `80` and HTTPS port `443`. HTTP requests are redirected to HTTPS.

## Prerequisites

- Permission to bind ports `80` and `443`.
- A PEM certificate and private key in this example directory. Use the certificate instructions in the Rustls example, with this directory as the working directory.

## Usage

```sh
cd middleware/http-to-https
cargo run
```
