# Cookie Authentication

Demonstrates login and logout with cookie-based identity storage.

## Usage

```sh
cd auth/cookie-auth
cargo run
# Server: http://localhost:8080
```

## Client

In another terminal, log in and save the session cookie with [curl](https://curl.se):

```sh
curl -i -c cookies.txt -X POST http://localhost:8080/login
```

The handler logs in as `user1` and returns a redirect to `/`. The server generates a new signing key at startup, and the session expires after one minute. Use the cookie returned by the current server.

Send the saved cookie to identify the user:

```sh
curl -b cookies.txt http://localhost:8080/
```

```text
Hello user1
```

Log out and update the cookie file:

```sh
curl -i -b cookies.txt -c cookies.txt -X POST http://localhost:8080/logout
```
