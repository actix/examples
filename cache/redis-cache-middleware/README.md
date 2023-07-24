# Redis Cache Middleware

This project demonstrates how to implement Redis cache middleware to handle cache responses synchronously.
The application should be able to function properly even if Redis is not running; however, the caching process will be disabled in such cases.


## Setting up
Configure the environment variable `REDIS_HOST`, or if not set, the default host `redis://localhost:4379` will be used. TLS is supported using the `rediss://` protocol.
Run the server using `cargo run`.

## Endpoints

### `GET /fibonacci/{number}`

To test the demo, send a GET request to `/fibonacci/{number}`, where {number} is a positive integer of type u64.

## Request Directives

- `Cache-Control: no-cache` will return the most up-to-date response while still caching it. This will always yield a `miss` cache status.
- `Cache-Control: no-store` will prevent caching, ensuring you always receive the latest response.

## Verify Redis Contents

When making the first GET request to `/fibonacci/47`, it may take around 8 seconds to respond.
If Redis is running and the connection is established, subsequent requests should return the cached result immediately, a `hit` cache status will be returned, but with content type `application/json`.

## Known issues

- Connecting to a remote Redis server might introduce significant overhead and could lead to prolonged connection times or even failure to reach the remote server.

## Further implementations

- Implement asynchronous insertion of cache responses.
- Explore using an in-memory datastore within the application process to reduce reliance on Redis.
