# middleware examples

This example showcases a bunch of interesting and little complex uses of middlewares. See also the [Middleware guide](https://actix.rs/docs/middleware/)..

## Usage

```bash
cd middleware-complex
cargo run
# Started http server: 127.0.0.1:8080
```

Look in `src/main.rs` and comment the different middlewares in/out to see how
they function.

## Middlewares

### hack_my_web::HackMyWeb

A middleware have a two hacker's functions about Request and Response.

First, if received path equals "/hack_secret", show a page of secret path.

Second, send response with extra custom header "hacker-code"
