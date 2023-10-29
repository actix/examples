# Middleware: Rate Limiting

This example showcases two middleware that achieve rate limiting for your API endpoints. One uses a simple leaky-bucket implementation and the other delegates to [`actix-governor`].

## Usage

```sh
cd middleware/rate-limit
cargo run
```

Look in `src/rate_limit.rs` to see the leaky-bucket implementation.

## Routes

- [GET /test/simple](http://localhost:8080/test/simple) - uses the hand-written leaky-bucket rate limiting.
- [GET /test/governor](http://localhost:8080/test/governor) - uses [`actix-governor`].

Calling either of these endpoints too frequently will result in a 429 Too Many Requests response.

[`actix-governor`]: https://crates.io/crates/actix-governor
