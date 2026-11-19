# Middleware: CSRF (Double Submit Cookie)

Stateless CSRF protection using [`actix-csrf-middleware`]. The token is stored in a cookie and mirrored back in a form field, and the middleware rejects any mutating request whose two copies do not agree.

The token is an HMAC over a session (or pre-session) identifier rather than a bare random value, which binds it to one browser. Tokens are compared in constant time. See the [OWASP CSRF Prevention Cheat Sheet][owasp] for the pattern and its trade-offs.

No session store is required. For the stateful variant, see [`csrf-synchronizer`](../csrf-synchronizer/).

## Usage

```sh
cd middleware/csrf-double-submit
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

Rotate on authentication, or a token minted before sign-in stays valid after it. `rotate_csrf_after_login` expires the anonymous token and issues one bound to the new session; `rotate_csrf_after_logout` reverses it.

Anonymous requests carry a `pre-session` cookie and an anonymous token (`CSRF-ANON`), which protect sign-in and registration.

The application owns the session id cookie; the middleware only reads it. Write it with the same `Domain` the middleware is configured with, or the two scopes leave duplicate cookies the middleware cannot reconcile.

## Try rejecting a request

```sh
curl -i -X POST http://localhost:8080/message -d 'text=hello'
```

Returns `400 Bad Request` with `{"error":"csrf_token_missing"}` because no token was presented. Every rejection is a typed error rendered as JSON, recoverable through `ErrorHandlers` if you want a different shape.

## Notes

- The secret must be at least 32 bytes and identical across workers and restarts. Load it from configuration rather than hard-coding it.
- `with_secure(false)` is for plain HTTP in local development. Leave the default `true` behind TLS.
- Single-page apps can send the token in the `X-CSRF-Token` header instead of a form field, which skips body buffering entirely.

[`actix-csrf-middleware`]: https://crates.io/crates/actix-csrf-middleware
[owasp]: https://cheatsheetseries.owasp.org/cheatsheets/Cross-Site_Request_Forgery_Prevention_Cheat_Sheet.html
