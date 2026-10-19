# Protocol Buffers

Demonstrates Protocol Buffers request and response bodies with Actix Web.

## Prerequisites

- The Protocol Buffers compiler, `protoc`, for the client commands.
- curl.

## Usage

### Server

```sh
cd protobuf
cargo run
```

### Client

In another terminal, start from the repository root. Encode a `MyObj` message with the schema in `test.proto`, send it to the server, and decode the response:

```sh
cd protobuf
printf '%s\n' 'number: 9' 'name: "USB"' | protoc --encode=MyObj test.proto > request.pb
curl http://localhost:8080/ \
  -H 'Content-Type: application/protobuf' \
  --data-binary @request.pb \
  --output response.pb
protoc --decode=MyObj test.proto < response.pb
```

The response contains the same `number` and `name` fields.
