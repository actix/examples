# Redis

This project illustrates how to send multiple cache requests to Redis in bulk, asynchronously. This approach resembles traditional Redis pipelining. [See here for more details about this approach.](https://github.com/benashford/redis-async-rs/issues/19#issuecomment-412208018)

## Prerequisites

- A Redis server listening on `127.0.0.1:6379`.

## Usage

```sh
cd databases/redis
cargo run
```

## Routes

### `POST /stuff`

Send a JSON object containing three strings to the `/stuff` endpoint:

```json
{
  "one": "first entry",
  "two": "second entry",
  "three": "third entry"
}
```

The server stores these values under the keys `my_domain:one`, `my_domain:two`, and `my_domain:three`.

Using [HTTPie]:

```sh
http :8080/stuff one="first entry" two="second entry" three="third entry"
```

Using [curl]:

```sh
curl localhost:8080/stuff -H 'content-type: application/json' -d '{"one":"first entry","two":"second entry","three":"third entry"}'
```

### `DELETE /stuff`

Send a DELETE request to `/stuff` to remove the three entries.

Using [HTTPie]:

```sh
http DELETE :8080/stuff
```

Using [curl]:

```sh
curl -XDELETE 127.0.0.1:8080/stuff
```

## Verify Redis Contents

At any time, verify the contents of Redis using its CLI:

```sh
redis-cli MGET my_domain:one my_domain:two my_domain:three
```

[HTTPie]: https://httpie.io/cli
[curl]: https://curl.se
