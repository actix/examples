# JSON decode errors

This example demonstrates how to return useful error messages to the client when the server receives a request with invalid JSON, or which cannot be deserialized to the expected model. By configuring an `error_handler` on the route, we can set appropriate response codes and return the string representation of the error.

## Usage

```shell
cd json/json_decode_error
cargo run
# Started HTTP server: 127.0.0.1:8080
```

## Examples

The examples use `curl -i` in order to show the status line with the response code. The response headers have been omitted for brevity, and replaced with an ellipsis `...`.

- A well-formed request

  ```shell
  $ curl -i 127.0.0.1:8080 -H 'Content-Type: application/json' -d '{"name": "Alice"}'
  HTTP/1.1 200 OK
  ...

  Hello Alice!
  ```

- Missing `Content-Type` header

  ```shell
  $ curl -i 127.0.0.1:8080 -d '{"name": "Bob"}'
  HTTP/1.1 415 Unsupported Media Type
  ...

  Content type error
  ```

- Malformed JSON

  ```shell
  $ curl -i 127.0.0.1:8080 -H 'Content-Type: application/json' -d '{"name": "Eve}'
  HTTP/1.1 400 Bad Request
  ...

  Json deserialize error: EOF while parsing a string at line 1 column 14
  ```

- JSON value of wrong type

  ```shell
  $ curl -i 127.0.0.1:8080 -H 'Content-Type: application/json' -d '{"name": 350}'
  HTTP/1.1 422 Unprocessable Entity
  ...

  Json deserialize error: invalid type: integer `350`, expected a string at line 1 column 12
  ```

- Wrong JSON key

  ```shell
  $ curl -i 127.0.0.1:8080 -H 'Content-Type: application/json' -d '{"namn": "John"}'
  HTTP/1.1 422 Unprocessable Entity
  ...

  Json deserialize error: missing field `name` at line 1 column 16
  ```

## More documentation

[`actix_web::web::JsonConfig`](https://docs.rs/actix-web/latest/actix_web/web/struct.JsonConfig.html)
