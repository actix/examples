# Middleware: CSRF (Synchronizer Token)

Stateful CSRF protection using [`actix-csrf-middleware`] with [`actix-session`]. The token lives server-side in the session, and the middleware compares it in constant time against the copy the client presents.

Unlike the double submit variant the token is never readable by client scripts, at the cost of requiring a session store. See the [OWASP CSRF Prevention Cheat Sheet][owasp] for the trade-off.

For the stateless variant, see [`csrf-double-submit`](../csrf-double-submit/).

## Usage

```sh
cd middleware/csrf-synchronizer
cargo run
```

Open <http://localhost:8080>, sign in, then send a message.

## Routes

- [GET /](http://localhost:8080/) - renders a form carrying the token in a hidden `csrf_token` field.
- `POST /login` - issues a session id cookie and rotates the token from anonymous to authorized.
- `POST /logout` - tears down the session and the token.
- `POST /message` - protected. Reached only after the token is verified.

Every mutating route is protected, including login and logout themselves.

## Token rotation

Rotate on authentication, or a token minted before sign-in stays valid after it. `rotate_csrf_after_login` replaces the anonymous token with one bound to the new session; `rotate_csrf_after_logout` purges it.

## Cookie names

`actix-session` defaults its cookie to `id`, the same name as the middleware's default `session_id_cookie_name`. Left colliding, every request is classified authorized from the first response onward. This example names the session cookie `session`.

## Try rejecting a request

```sh
curl -i -X POST http://localhost:8080/message -d 'text=hello'
```

Returns `400 Bad Request` with `{"error":"csrf_token_missing"}` because no token was presented.

## Notes

- **Middleware order matters.** `SessionMiddleware` is wrapped after `CsrfMiddleware`, which makes it the outer layer. actix applies `wrap` from the inside out, and the session must be available by the time the CSRF middleware runs.
- `Key::generate()` is called once outside `HttpServer::new`. Generating it inside the closure would give each worker a different key and invalidate sessions between requests. Load a fixed key from configuration in production.
- The secret must be at least 32 bytes and identical across workers and restarts.
- `with_secure(false)` and `cookie_secure(false)` are for plain HTTP in local development. Leave both at their secure defaults behind TLS.
- This example uses `CookieSessionStore` to stay self-contained. Any `actix-session` backend works.

[`actix-csrf-middleware`]: https://crates.io/crates/actix-csrf-middleware
[`actix-session`]: https://crates.io/crates/actix-session
[owasp]: https://cheatsheetseries.owasp.org/cheatsheets/Cross-Site_Request_Forgery_Prevention_Cheat_Sheet.html
