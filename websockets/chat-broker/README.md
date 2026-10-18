# WebSocket Chat With A Broker

An alternative implementation of the [WebSocket chat example](../chat/README.md).

Differences:

- Chat Server Actor is a System Service that runs in the same thread as the HttpServer/WS Listener.
- The [actix-broker](https://github.com/Chris-Ricketts/actix-broker) crate is used to facilitate the sending of some messages between the Chat Session and Server Actors where the session does not require a response.
- The Client is not required to send Ping messages. The Chat Server Actor auto-clears dead sessions.

Possible Improvements:

- Could the Chat Server Actor be simultaneously a System Service (accessible to the Chat Session via the System Registry) and also run in a separate thread?

## Usage

```sh
cd websockets/chat-broker
cargo run
```

## Chat Commands

The server accepts WebSocket connections and supports these messages:

- `/list` - list all available rooms
- `/join name` - join room, if room does not exist, create new one
- `/name name` - set session name
- `some message` - just string, send message to all peers in same room

## Browser Client

Open <http://localhost:8080> in your browser.
