# WebSocket Chat (actor-less)

> Multi-room WebSocket chat server using [`actix-ws`].

## Usage

### Server

```sh
cd websockets/echo-actorless
cargo run
# starting HTTP server at http://localhost:8080
```

### Browser Client

Go to <http://localhost:8080/> in a browser.

### CLI Client

```sh
# using `websocat` (https://github.com/vi/websocat)
websocat -v --ping-interval=2 ws://127.0.0.1:8080/ws
```

## Chat Commands

Once connected, the following slash commands can be sent:

- `/list`: list all available rooms
- `/join name`: join room, if room does not exist, create new one
- `/name name`: set session name

Sending a plain string will broadcast that message to all peers in same room.

[`actix-ws`]: https://crates.io/crates/actix-ws
