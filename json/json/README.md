# JSON

Demonstrates JSON extraction and manual deserialization with `serde_json` in Actix Web.

## Usage

### Server

```sh
cd json/json
cargo run
# Server: http://localhost:8080
```

### Browser Client

Use an HTTP client such as [Postman](https://www.postman.com/).

- POST / (JSON extractor):

  - method : `POST`
  - url : `http://127.0.0.1:8080/`
  - header : `Content-Type` = `application/json`
  - body (raw) : `{"name": "Test user", "number": 100}`

- POST /manual (manual deserialization):

  - method : `POST`
  - url : `http://127.0.0.1:8080/manual`
  - header : `Content-Type` = `application/json`
  - body (raw) : `{"name": "Test user", "number": 100}`

- POST /extractor (JSON extractor with the global payload limit):

  - method : `POST`
  - url : `http://127.0.0.1:8080/extractor`
  - header : `Content-Type` = `application/json`
  - body (raw) : `{"name": "Test user", "number": 100}`

- POST /extractor2 uses the same JSON body with a route-specific payload limit of 1024 bytes. The global limit is 4096 bytes.

### Python Client

In another terminal, start from the repository root:

```sh
cd json/json
python3 -m venv .venv
. .venv/bin/activate
python3 -m pip install aiohttp
python3 client.py
```
