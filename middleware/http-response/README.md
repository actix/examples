## Middleware : Return HttpResponse from Middleware

```rs
cd middleware-return-httpresponse
cargo run
# Started http server: 127.0.0.1:8080
```

## What is this?

A Middleware example which returning HttpResponse.

## How to test

### success case
```sh
curl http://127.0.0.1:8080/ -H 'Authorization:ok' | json_pp -json_opt pretty,canonical
 % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100    42  100    42    0     0  42000      0 --:--:-- --:--:-- --:--:-- 42000
{
   "data" : "Hello this is success response!"
}
```

### failed case
```sh
curl http://127.0.0.1:8080/ | json_pp -json_opt pretty,canonical
 % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   102  100   102    0     0    99k      0 --:--:-- --:--:-- --:--:--   99k
{
   "data" : "Hello this is default error message! you need to set Authorization header to get thru this."
}
```