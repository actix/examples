## Status Code

The [StatusCode](https://docs.rs/actix-web/latest/actix_web/http/struct.StatusCode.html) trait contains the methods to test for the status codes.
There are mainly two ways to test for the returning status code:

1. Test for the exact status code (defined [here](https://docs.rs/actix-web/latest/actix_web/http/struct.StatusCode.html#impl-StatusCode-1))
2. Test for the status code classes `informational`, `success`, `redirect`, `client_error` and `server_error` (defined [here](https://docs.rs/actix-web/latest/actix_web/http/struct.StatusCode.html#method.is_success))

You can find the list of status codes definitions and their constants [here](https://docs.rs/http/0.2.9/src/http/status.rs.html#323-515).

RFC 7231 [docs](https://datatracker.ietf.org/doc/html/rfc7231#section-6.2).

## server

```sh
cd test/status-codes
cargo test
```
