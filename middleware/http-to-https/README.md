## Middleware: Redirect Any HTTP Connection To Use HTTPS Connection

## Alternatives

A pre-built solution is soon to be built-in. For now, see [`RedirectHttps`](https://docs.rs/actix-web-lab/0.20/actix_web_lab/middleware/struct.RedirectHttps.html) from [`actix-web-lab`](https://crates.io/crates/actix-web-lab).

## This Example

This example is the next step after implementing this example : [Setup TLS via rustls](https://github.com/actix/examples/tree/master/https-tls/rustls).

You might have already implemented TLS (using one of the ways mentioned in the example of security section), and have setup your server to listen to port 443 (for HTTPS).

Now, the only problem left to solve is, to listen to **HTTP** connections as well and redirect them to use **HTTPS**

## Usage

```sh
cd middleware/http-to-https
cargo run
```
