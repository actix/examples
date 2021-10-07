# middleware examples

This example showcases a middleware that adds and retreives request-local data. See also the [Middleware guide](https://actix.rs/docs/middleware/).

## Usage

```bash
cd basics/middleware-ext-mut
cargo run
# Started http server: 127.0.0.1:8080
```

Look in `src/add_msg.rs` to see how it works.

## Routes

- [GET /on](http://localhost:8080/on) - `200 OK` with "hello from middleware" body and console log showing the request passed through the middleware
- [GET /off](http://localhost:8080/off) - `500 Internal Server Error` with "no message found" body and console log showing the request passed through the middleware
