# Websocket chat example

This is extension of the [actix chat example](https://github.com/actix/examples/tree/HEAD/websockets/chat)

Added features:

- Browser WebSocket client
- Chat server runs in separate thread
- Tcp listener runs in separate thread
- Application state is shared with the websocket server and a resource at `/count/`

## Server

1. Chat server listens for incoming tcp connections. Server can access several types of message:

- `/list` - list all available rooms
- `/join name` - join room, if room does not exist, create new one
- `/name name` - set session name
- `some message` - just string, send message to all peers in same room
- client has to send heartbeat `Ping` messages, if server does not receive a heartbeat message for 10 seconds connection gets dropped

2. [http://localhost:8080/count/](http://localhost:8080/count/) is a non-websocket endpoint and will affect and display state.

To start server use command: `cargo run --bin websocket-chat-server`

## Client

Client connects to server. Reads input from stdin and sends to server.

To run client use command: `cargo run --bin websocket-chat-client`

## WebSocket Browser Client

Open url: [http://localhost:8080/](http://localhost:8080/)
