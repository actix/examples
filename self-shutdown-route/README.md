# self-shutdown-route

> Demonstrates how to shutdown the web server using a route hosted by the server itself using channels.

This technique can be easily modified to support shutting down the server using other kinds of external events.
Created in response to actix/actix-web#1315. 

## Usage

### Running The Server

```bash
cargo run --bin self-shutdown-route

# Starting 8 workers
# Starting "actix-web-service-127.0.0.1:8080" service on 127.0.0.1:8080
```

### Available Routes

- [GET /hello](http://localhost:8080/hello)
  - Regular hello world route
- [POST /stop](http://localhost:8080/stop)
  - Calling this will shutdown the server and exit
