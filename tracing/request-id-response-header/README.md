# Request ID in Response Header

This example shows how to set the `RequestId` as a response header.

## Running

You can launch this example with

```bash
cd tracing/request-id-response-header
cargo run
```

An `actix-web` application will be listening on port `8080`. You can fire requests to it with:

```bash
curl -v http://localhost:8080/hello
```

```text
...
< HTTP/1.1 200 OK
< content-length: 12
< x-request-id: 1d5c5448-44d2-4051-ab59-985868875f94
...
```
