# Websocket chat broker example

This is a different implementation of the
[websocket chat example](https://github.com/actix/examples/tree/master/websocket-chat)

Differences:

* Chat Server Actor is a System Service that runs in the same thread as the HttpServer/WS Listener.
* The [actix-broker](https://github.com/Chris-Ricketts/actix-broker) crate is used to facilitate the sending of some messages between the Chat Session and Server Actors where the session does not require a response.
* The Client is not required to send Ping messages. The Chat Server Actor auto-clears dead sessions.

Possible Improvements:

* Could the Chat Server Actor be simultaneously a System Service (accessible to the Chat Session via the System Registry) and also run in a seperate thread?

## Server

Chat server listens for incoming tcp connections. Server can access several types of message:

* `/list` - list all available rooms
* `/join name` - join room, if room does not exist, create new one
* `/name name` - set session name
* `some message` - just string, send message to all peers in same room

To start server use command: `cargo run`

## WebSocket Browser Client

Open url: [http://localhost:8080/](http://localhost:8080/)
