# WebSocket Echo

Simple echo WebSocket server.

## Usage

### Server

```sh
cd websockets/echo
cargo run --bin websocket-server
# Server: http://localhost:8080
```

### Browser Client

- [http://localhost:8080/index.html](http://localhost:8080/index.html)

### Rust Client

```sh
cd websockets/echo
cargo run --bin websocket-client
```

### Python Client

In another terminal, start from the repository root:

```sh
cd websockets/echo
python3 -m venv .venv
. .venv/bin/activate
python3 -m pip install aiohttp
python3 websocket-client.py
```
