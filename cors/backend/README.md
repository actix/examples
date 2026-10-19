# CORS Backend

Demonstrates cross-origin requests from the frontend on `http://localhost:8081`.

## Usage

```sh
cd cors/backend
cargo run
# Server: http://localhost:8080
```

## Routes

### `POST /user/info`

Send a JSON object with the following fields:

```json
{
  "username": "username",
  "email": "email",
  "password": "password",
  "confirm_password": "password"
}
```

```sh
curl http://localhost:8080/user/info \
  -H 'Content-Type: application/json' \
  -d '{"username":"user","email":"user@example.com","password":"password","confirm_password":"password"}'
```

## See Also

- For more related examples of [actix_cors](https://docs.rs/actix-cors/latest/actix_cors/struct.Cors.html)
