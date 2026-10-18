# HTTPS Client

The goal of this example is to show you how to use the `awc` for secure HTTPS communication using Rustls.

It uses best practices for efficient client set up and demonstrates how to increase the default payload limit.

This example downloads a 10 MB image from Wikipedia when a client requests the server route.

## Usage

```sh
cd https-tls/awc-https
cargo run
```

Open <http://localhost:8080> in your browser.
