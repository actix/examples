# Middleware: Add/Retrieve Request-Local Data

This example showcases a middleware that adds and retrieves request-local data. See also the [middleware guide](https://actix.rs/docs/middleware).

## Usage

```sh
cd middleware/request-extensions
cargo run
```

Look in `src/add_msg.rs` to see how it works.

## Routes

- [GET /on](http://localhost:8080/on) - `200 OK` with "hello from middleware" body and console log showing the request passed through the middleware
- [GET /off](http://localhost:8080/off) - `500 Internal Server Error` with "no message found" body and console log showing the request passed through the middleware
