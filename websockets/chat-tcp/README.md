# WebSocket And TCP Chat

This extends the [Ractor chat example](https://github.com/actix/examples/tree/HEAD/websockets/chat).

Added features:

- Browser WebSocket client
- Shared Ractor actor manages rooms for TCP and WebSocket clients
- Tokio TCP listener and actix-ws WebSocket connections

## Chat Commands

The server accepts TCP and WebSocket connections and supports these messages:

- `/list` - list all available rooms
- `/join name` - join room, if room does not exist, create new one
- `/name name` - set session name (WebSocket clients only)
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

## Protocol

TCP uses JSON messages with a two-byte, big-endian length prefix. The request and response formats
remain compatible with the included TCP client. Encoded messages must fit in 65,535 bytes.

## Shutdown

SIGINT, SIGTERM, and SIGQUIT cancel both listeners on Unix; other platforms use Ctrl-C. WebSocket
sessions close with code 1001. TCP sessions close their connections, and the chat actor stops after
both listeners finish shutdown.
