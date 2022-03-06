## HTTP naïve proxy example

This is a relatively simple HTTP proxy, forwarding HTTP requests to another HTTP server, including request body, headers, and streaming uploads.

To start:

```shell
cargo run -- <listen addr> <listen port> <forward addr> <forward port>
# example:
cargo run -- 127.0.0.1 3333 127.0.0.1 8080
```
