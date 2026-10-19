# Unix Domain Sockets

Serves HTTP over a Unix domain socket. This example requires a Unix operating system.

## Usage

```sh
cd unix-socket
cargo run
```

## Client

In another terminal, send a request with curl:

```sh
curl --unix-socket /tmp/actix-uds.socket http://localhost/
```

```text
Hello world!
```

The server uses Actix Web's default worker count. See [`HttpServer::workers`](https://docs.rs/actix-web/4/actix_web/struct.HttpServer.html#method.workers).

`bind_uds` removes an existing socket file before binding, so a socket left by a previous run does not require manual removal.
