# JSON-RPC

A simple demo for building a `JSONRPC over HTTP` server using [Actix Web](https://github.com/actix/actix-web).

## Usage

```sh
cd json/jsonrpc
cargo run
# Server: http://localhost:8080
```

## Client

### curl

```console
$ curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "ping", "params": [], "id": 1}' http://127.0.0.1:8080
# {"jsonrpc":"2.0","result":"pong","error":null,"id":1}
```

### Python Client

In another terminal, start from the repository root:

```sh
cd json/jsonrpc
python3 -m venv .venv
. .venv/bin/activate
python3 -m pip install requests
python3 tests/test_client.py
```

## Methods

- `ping`: Return `pong` immediately
- `wait`: Wait `n` seconds, and then pong
- `get`: Get global count
- `inc`: Increment global count

See `tests/test_client.py` for more examples.
