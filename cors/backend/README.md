# actix_cors::Cors

## Running Server

```sh
cd cors/backend
cargo run
# starting HTTP server at http://localhost:8080
```

### web client

- [http://localhost:8080/user/info](http://localhost:8080/user/info)
  ```json
  // payload structure
  {
    "username": "username",
    "email": "email",
    "password": "password",
    "confirm_password": "password"
  }
  ```

## Others

- For more related examples of [actix_cors](https://docs.rs/actix-cors/latest/actix_cors/struct.Cors.html)
