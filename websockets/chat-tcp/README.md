# WebSocket And TCP Chat

This is extension of the [actix chat example](https://github.com/actix/examples/tree/HEAD/websockets/chat)

Added features:

- Browser WebSocket client
- Chat server runs in separate thread
- TCP listener runs in separate thread

## Chat Commands

The server accepts TCP and WebSocket connections and supports these messages:

- `/list` - list all available rooms
- `/join name` - join room, if room does not exist, create new one
- `/name name` - set session name
- `some message` - just string, send message to all peers in same room
- client has to send heartbeat `Ping` messages, if server does not receive a heartbeat message for 10 seconds connection gets dropped

## Usage

```sh
cd websockets/chat-tcp
cargo run --bin websocket-tcp-server
```

If the current directory is not correct, the server will look for `index.html` in the wrong place.

## Client

Client connects to server. Reads input from stdin and sends to server.

In another terminal, start the TCP client from the repository root:

```sh
cd websockets/chat-tcp
cargo run --bin websocket-tcp-client
```

## Browser Client

Open url: <http://localhost:8080>
