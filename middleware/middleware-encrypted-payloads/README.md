# Encrypted Request + Response Payloads Middleware

Shows an example of a `from_fn` middleware that:

1. extracts a JSON request payload;
1. decrypts a data field;
1. re-encodes as JSON and re-packs the request;
1. calls the wrapped service;
1. unpacks the response and parses a similarly structured JSON object;
1. encrypts the data field;
1. re-packs the response.

All this to say, it provides a (fairly brittle) way to have handlers not need to care about the encryption and decryption steps.

## Usage

Using [HTTPie] throughout for simplicity.

```console
$ http POST :8080/encrypt id:=1234 data=abcde > tmp.json
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 1234,
    "data": "kq6MKdP+I0hoI7YC7CN39yamx67T",
    "nonce": "dW5pcXVlIG5vbmNl"
}

$ cat tmp.json | http -v POST :8080/reverse
HTTP/1.1 200 OK
Content-Type: application/json

{
    "data": "UL4PeOr9Di8xpFEJZgylJ5K8R7vW",
    "nonce": "dW5pcXVlIG5vbmNl"
}
```

The server logs would look something like

```plain
[INFO  middleware_encrypted_payloads] creating encrypted sample request for ID = 1234
[INFO  actix_web::middleware::logger] 127.0.0.1 "POST /encrypt HTTP/1.1" 200 76 "-" "-" 0.000393
...
[INFO  middleware_encrypted_payloads] decrypting request 1234
[INFO  middleware_encrypted_payloads] request 1234 with data: abcde
[INFO  middleware_encrypted_payloads] response 1234 with new data: edcba
[INFO  middleware_encrypted_payloads] encrypting response 1234
[INFO  actix_web::middleware::logger] 127.0.0.1 "POST /reverse HTTP/1.1" 200 66 "-" "-" 0.000425
```

[httpie]: https://httpie.io/cli
