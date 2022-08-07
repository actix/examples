# shutdown-server

Demonstrates how to shutdown the web server in a couple of ways:

1. remotely, via HTTP request
1. sending a SIGINT signal to the server (control-c)
   - Actix Web servers support shutdown signals by default. [See here for more info.](https://actix.rs/docs/server#graceful-shutdown)

## Usage

### Running The Server

```console
$ cd shutdown-server
$ cargo run --bin shutdown-server
[INFO] starting HTTP server at http://localhost:8080
[INFO] Starting 2 workers
[INFO] Actix runtime found; starting in Actix runtime
```

### Available Routes

- [`GET /hello`](http://localhost:8080/hello)
  - Test hello world
- `POST /stop/true`
  - Gracefully shuts down the server and exit
- `POST /stop/false`
  - Forces server shutdown and exits
