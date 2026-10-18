# Redis Cache Middleware

This project demonstrates how to implement Redis cache middleware to handle cache responses synchronously. The application should be able to function properly even if Redis is not running; however, the caching process will be disabled in such cases.

## Setup

Set `REDIS_HOST` to a Redis URL, or use the default `redis://localhost:6379`. TLS is supported with the `rediss://` protocol. Redis is optional; requests still work when it is unavailable.

## Usage

```sh
cd cache/redis
cargo run
```

The HTTP server listens on port `8080`. Set `LISTEN_PORT` to use another port.

## Routes

### `GET /fibonacci/{number}`

To test the demo, send a GET request to `/fibonacci/{number}`, where {number} is a positive integer of type u64.

## Request Directives

- `Cache-Control: no-cache` will return the most up-to-date response while still caching it. This will always yield a `miss` cache status.
- `Cache-Control: no-store` will prevent caching, ensuring you always receive the latest response.

## Verify Redis Contents

When making the first GET request to `/fibonacci/47`, it may take around 8 seconds to respond. If Redis is running and the connection is established, subsequent requests should return the cached result immediately, a `hit` cache status will be returned, but with content type `application/json`.

## Known Issues

- Connecting to a remote Redis server might introduce significant overhead and could lead to prolonged connection times or even failure to reach the remote server.

## Possible Improvements

- Implement asynchronous insertion of cache responses.
- Explore using an in-memory datastore within the application process to reduce reliance on Redis.
