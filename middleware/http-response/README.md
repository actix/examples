# Middleware: Return An HTTP Response

Demonstrates how middleware can return an HTTP response.

## Usage

```sh
cd middleware/http-response
cargo run
# Server: http://localhost:8080
```

## Client

### Success

```sh
curl --silent --show-error http://localhost:8080/ -H 'Authorization: ok'
```

```json
{
  "data": "Hello this is success response!"
}
```

### Failure

```sh
curl --silent --show-error http://localhost:8080/
```

```json
{
  "data": "Hello this is default error message! you need to set Authorization header to get thru this."
}
```
