# diesel

Basic integration of [Diesel](https://diesel.rs/) using SQLite for Actix web.

## Usage

### Install SQLite

```sh
# on OpenSUSE
sudo zypper install sqlite3-devel libsqlite3-0 sqlite3

# on Ubuntu
sudo apt-get install libsqlite3-dev sqlite3

# on Fedora
sudo dnf install libsqlite3x-devel sqlite3x

# on macOS (using homebrew)
brew install sqlite3
```

### Initialize SQLite Database

```sh
cd examples/diesel
cargo install diesel_cli --no-default-features --features sqlite

echo "DATABASE_URL=test.db" > .env
diesel migration run
```

There will now be a database file at `./test.db`.

### Running Server

```sh
cd examples/diesel
cargo run (or ``cargo watch -x run``)

# Started http server: 127.0.0.1:8080
```

### Available Routes

#### `POST /user`

Inserts a new user into the SQLite DB.

Provide a JSON payload with a name. Eg:
```json
{ "name": "bill" }
```

On success, a response like the following is returned:
```json
{
    "id": "9e46baba-a001-4bb3-b4cf-4b3e5bab5e97",
    "name": "bill"
}
```

<details>
  <summary>Client Examples</summary>

  Using [HTTPie](https://httpie.org/):
  ```sh
  http POST localhost:8080/user name=bill
  ```

  Using cURL:
  ```sh
  curl -S -X POST --header "Content-Type: application/json" --data '{"name":"bill"}' http://localhost:8080/user
  ```
</details>

#### `GET /user/{user_uid}`

Gets a user from the DB using its UID (returned from the insert request or taken from the DB directly). Returns a 404 when no user exists with that UID.

<details>
  <summary>Client Examples</summary>

  Using [HTTPie](https://httpie.org/):
  ```sh
  http localhost:8080/user/9e46baba-a001-4bb3-b4cf-4b3e5bab5e97
  ```

  Using cURL:
  ```sh
  curl -S http://localhost:8080/user/9e46baba-a001-4bb3-b4cf-4b3e5bab5e97
  ```
</details>

### Explore The SQLite DB

```sh
sqlite3 test.db
```

```
sqlite> .tables
sqlite> SELECT * FROM users;
```


## Using Other Databases

You can find a complete example of Diesel + PostgreSQL at: [https://github.com/TechEmpower/FrameworkBenchmarks/tree/master/frameworks/Rust/actix](https://github.com/TechEmpower/FrameworkBenchmarks/tree/master/frameworks/Rust/actix)
