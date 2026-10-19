# WebSocket Chat

This is a multi-threaded chat server example.

Fancy shiny features:

- Browser-based WebSocket client served from static HTML and JavaScript
- Chat server runs in separate thread
- Application state is shared with the WebSocket server and a resource at `/count`
- Uses actors for improved readability of code in the server.rs implementation

## Chat Commands

The chat server accepts WebSocket connections and supports these messages:

- `/list` - list all available rooms
- `/join name` - join room, if room does not exist, create new one
- `/name name` - set session name
- `some message` - just string, send message to all peers in same room
- client has to respond to heartbeat `Ping` messages, if server does not receive a heartbeat 'Pong' message for 10 seconds connection gets dropped

[http://localhost:8080/count](http://localhost:8080/count) displays the visitor count shared with the chat server.

## Usage

```sh
cd websockets/chat
cargo run --bin websocket-chat-server
```

## Browser Client

- Open in browser: <http://localhost:8080/>.
- Use two tabs to set up a proper conversation.

## Python Client

The client reads input from standard input and sends it to the server. In another terminal, start from the repository root:

```sh
cd websockets/chat
python3 -m venv .venv
. .venv/bin/activate
python3 -m pip install -r requirements.txt
python3 client.py
```
