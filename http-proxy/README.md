# HTTP Proxy

This is a relatively simple HTTP proxy, forwarding HTTP requests to another HTTP server, including request body, headers, and streaming uploads.

## Usage

The command takes a listen address, listen port, forwarding address, and forwarding port. Start a backend on `127.0.0.1:8080`, then run:

```sh
cd http-proxy
cargo run -- 127.0.0.1 3333 127.0.0.1 8080
```

Send requests to `http://localhost:3333` to forward them with `awc`, or to `http://localhost:3333/using-reqwest` to use the `reqwest` implementation.
