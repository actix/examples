# JSON Validation

Demonstrates JSON validation and three sequential HTTP requests. Each step posts the data to HTTPBin and returns the JSON from its response.

Actix Web features illustrated here include:

1. Extract JSON request data.
1. Validate the data with the `validator` crate.
1. Post a JSON body with the `awc` client.
1. Await each request and return the final response.

## Prerequisites

- Network access to `https://httpbin.org/post`.

## Usage

```sh
cd json/json-validation
cargo run
# Server: http://localhost:8080
```

## Client

Send a POST request with HTTPie:

```sh
http POST :8080/ id=1 name=JohnDoe
```
