# Redis Sessions

Session storage using Redis.

## Prerequisites

- A Redis server listening on `127.0.0.1:6379`.

## Usage

```sh
cd auth/redis-session
cargo run
# Server: http://localhost:8080
```

## Routes

- [GET /](http://localhost:8080/)
- [POST /do_something](http://localhost:8080/do_something)
- [POST /login](http://localhost:8080/login)
- [POST /logout](http://localhost:8080/logout)
