# Routing Guards

Shows how to set up custom routing guards.

- Routing different API versions using a header instead of path.

## Usage

### Server

```sh
cd guards
cargo run --bin=guards
```

### Routes

#### `GET /api/hello`

Requires the `Accept-Version` header to be present and set to `1` or `2`.

Using [HTTPie]:

```sh
http :8080/api/hello Accept-Version:1
```

Using [curl]:

```sh
curl 'localhost:8080/api/hello' -H 'accept-version: 1'
```

[HTTPie]: https://httpie.io/cli
[curl]: https://curl.se
