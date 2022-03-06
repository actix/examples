A simple demo for building a `JSONRPC over HTTP` server using [Actix Web](https://github.com/actix/actix-web).

# Server

```sh
cd json/jsonrpc
cargo run
# Starting server on 127.0.0.1:8080
```

# Client

**curl**

```sh
$ curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "ping", "params": [], "id": 1}' http://127.0.0.1:8080
# {"jsonrpc":"2.0","result":"pong","error":null,"id":1}
```

**python**

```sh
$ python tests\test_client.py
# {'jsonrpc': '2.0', 'result': 'pong', 'error': None, 'id': 1}
```

# Methods

- `ping`: Pong immeditely
- `wait`: Wait `n` seconds, and then pong
- `get`: Get global count
- `inc`: Increment global count

See `tests\test_client.py` to get more information.
