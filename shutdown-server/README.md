# shutdown-server

Demonstrates how to shutdown the web server in a couple of ways:

1. remotely, via http request
    - Created in response to actix/actix-web#1315
1. sending a SIGINT signal to the server (control-c)
    - actix-server natively supports SIGINT

## Usage

### Running The Server

```sh
cd shutdown-server
cargo run --bin shutdown-server

# Starting 8 workers
# Starting "actix-web-service-127.0.0.1:8080" service on 127.0.0.1:8080
```

### Available Routes

- [GET /hello](http://localhost:8080/hello)
    - Regular hello world route
- [POST /stop](http://localhost:8080/stop)
    - Calling this will shutdown the server and exit
