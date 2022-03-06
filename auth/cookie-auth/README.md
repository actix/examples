# cookie-auth

```sh
cd auth/cookie-auth
cargo run
# Starting http server: 127.0.0.1:8080
```

Testing with cookie auth with [curl](https://curl.haxx.se).

Login:

        curl -v -b "auth-example=user1" -X POST  http://localhost:8080/login
        < HTTP/1.1 302 Found
        < set-cookie: auth-example=GRm2Vku0UpFbJ3CNTKbndzIYHVGi8wc8eoXm/Axtf2BO; HttpOnly; Path=/
        < location: /

Uses a POST request with a Useridentity `user1`. A cookie is set and a redirect to home `/` follows.

Get:

Now with the cookie `auth-example` sent in a GET request, the `user1` is recognized.

        curl -v -b "auth-example=GRm2Vku0UpFbJ3CNTKbndzIYHVGi8wc8eoXm/Axtf2BO" http://localhost:8080/
        * Connected to localhost (127.0.0.1) port 8080 (#0)
        > GET / HTTP/1.1
        > Host: localhost:8080
        > Cookie: auth-example=GRm2Vku0UpFbJ3CNTKbndzIYHVGi8wc8eoXm/Axtf2BO
        >
        < HTTP/1.1 200 OK
        <
        Hello user1
