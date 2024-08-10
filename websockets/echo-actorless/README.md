# Echo WebSocket (actor-less)

Simple echo websocket server using [`actix-ws`].

## Usage

### Server

```sh
cd websockets/echo-actorless
cargo run --bin websocket-server
# starting HTTP server at http://localhost:8080
```

### Browser Client

Go to <http://localhost:8080> in a browser.

### rust client

```sh
cd websockets/echo-actorless
cargo run --bin websocket-client
```

### CLI Client

```sh
# using `websocat` (https://github.com/vi/websocat)
websocat -v --ping-interval=2 ws://127.0.0.1:8080/ws
```

[`actix-ws`]: https://crates.io/crates/actix-ws
