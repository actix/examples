# guards

Shows how to set up custom routing guards.

- Routing different API versions using a header instead of path.

## Usage

### Running The Server

```sh
cd guards
cargo run --bin=guards
```

### Available Routes

#### `GET /api/hello`

Requires the `Accept-Version` header to be present and set to `1` or `2`.

Using [HTTPie]:

```sh
http :8080/api/hello Accept-Version:1
```

Using [cURL]:

```sh
curl 'localhost:8080/api/hello' -H 'accept-version: 1'
```

[httpie]: https://httpie.org
[curl]: https://curl.haxx.se
