# Websocket chat example

This is a multi-threaded chat server example.

Fancy shiny features:

- Browser-based WebSocket client served from static html+js
- Chat server runs in separate thread
- Tcp listener runs in separate thread
- Application state is shared with the websocket server and a resource at `/count/`
- Uses actors for improved readability of code in the server.rs implementation

## Server

1. Chat server listens for incoming tcp connections. Server can access several types of message:

- `/list` - list all available rooms
- `/join name` - join room, if room does not exist, create new one
- `/name name` - set session name
- `some message` - just string, send message to all peers in same room
- client has to respond to heartbeat `Ping` messages, if server does not receive a heartbeat 'Pong' message for 10 seconds connection gets dropped

2. [http://localhost:8080/count/](http://localhost:8080/count/) is a non-websocket endpoint and will affect and display state.

To start server use the following

```sh
cd websockets/chat
cargo run --bin websocket-chat-server
```

## WebSocket Browser Client

- Open in browser: <http://localhost:8080/>.
- Use two tabs to set up a proper conversation.

## Python Client using aiohttp

- Client connects to server. Reads input from stdin and sends to server.
- Create a venv environment `python3 -m venv venv`.
- Launch venv environment `source ./venv/bin/activate`.
- Fetch the needed python libraries `pip3 install -r requirements.txt`.
- Then start client as `./client.py`.
