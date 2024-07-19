# websocket

Simple echo websocket server.

## Usage

### server

```sh
cd websockets/echo
cargo run --bin websocket-server
# Started http server: 127.0.0.1:8080
```

### web client

- [http://localhost:8080/index.html](http://localhost:8080/index.html)

### rust client

```sh
cd websockets/echo
cargo run --bin websocket-client
```

### python client

- `pip install aiohttp`
- `python websocket-client.py`

if ubuntu :

- `pip3 install aiohttp`
- `python3 websocket-client.py`
