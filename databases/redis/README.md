# Redis

This project illustrates how to send multiple cache requests to Redis in bulk, asynchronously. This approach resembles traditional Redis pipelining. [See here for more details about this approach.](https://github.com/benashford/redis-async-rs/issues/19#issuecomment-412208018)

## Start Server

```sh
cd databases/redis
cargo run
```

## Endpoints

### `POST /stuff`

To test the demo, POST a json object containing three strings to the `/stuff` endpoint:

```json
{
  "one": "first entry",
  "two": "second entry",
  "three": "third entry"
}
```

These three entries will cache to redis, keyed accordingly.

Using [HTTPie]:

```sh
http :8080/stuff one="first entry" two="second entry" three="third entry"
```

Using [cURL]:

```sh
curl localhost:8080/stuff -H 'content-type: application/json' -d '{"one":"first entry","two":"second entry","three":"third entry"}'
```

### `DELETE /stuff`

To delete these, simply issue a DELETE http request to /stuff endpoint

Using [HTTPie]:

```sh
http DELETE :8080/stuff
```

Using [cURL]:

```sh
curl -XDELETE 127.0.0.1:8080/stuff
```

## Verify Redis Contents

At any time, verify the contents of Redis using its CLI:

```sh
echo "MGET mydomain:one mydomain:two mydomain:three" | redis-cli
```

[httpie]: https://httpie.org
[curl]: https://curl.haxx.se
