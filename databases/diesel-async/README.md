# diesel

Basic integration of [Diesel-async](https://github.com/weiznich/diesel_async) using PostgreSQL for Actix Web.

## Usage

### Install PostgreSQL

```sh
# on any OS
docker run -d --restart unless-stopped --name postgresql -e POSTGRES_USER=test-user -e POSTGRES_PASSWORD=password -p 5432:5432 -v postgres_data:/var/lib/postgresql/data postgres:alpine
```

make sure it has successfully started up and is running

```sh
# on any OS
docker ps --format "table {{.ID}}\t{{.Names}}\t{{.Status}}\t{{.Ports}}"
```

### Initialize PostgreSQL Database

```sh
cd databases/diesel-async
cargo install diesel_cli --no-default-features --features postgres

echo DATABASE_URL=postgres://test-user:password@localhost:5432/actix_example_diesel_async > .env
diesel setup
diesel migration run
```

The database will now be created in your PostgreSQL instance.

```sh
docker exec -i postgresql psql -U test-user -l
```

### Running Server

```sh
cd databases/diesel-async
cargo run

# Started http server: 127.0.0.1:8080
```

### Available Routes

#### `POST /items`

Inserts a new item into the PostgreSQL DB.

Provide a JSON payload with a name. Eg:

```json
{ "name": "thingamajig" }
```

On success, a response like the following is returned:

```json
{
  "id": "01948982-67d0-7a55-b4b1-8b8b962d8c6b",
  "name": "thingamajig"
}
```

<details>
  <summary>Client Examples</summary>

Using [HTTPie] / [xh]:

```sh
http POST :8080/items name=thingamajig
```

Using cURL:

```sh
curl --show-error -X POST --header "Content-Type: application/json" --data '{"name":"thingamajig"}' http://localhost:8080/items
```

</details>

#### `GET /items/{item_uid}`

Gets an item from the DB using its UID (returned from the insert request or taken from the DB directly). Returns a 404 when no item exists with that UID.

<details>
  <summary>Client Examples</summary>

Using [HTTPie] / [xh]:

```sh
http localhost:8080/items/9e46baba-a001-4bb3-b4cf-4b3e5bab5e97
```

Using cURL:

```sh
curl --show-error http://localhost:8080/items/9e46baba-a001-4bb3-b4cf-4b3e5bab5e97
```

</details>

### Explore The PostgreSQL DB

```sh
docker exec -i postgresql psql -U test-user -d actix_example_diesel_async -c "SELECT * from public.items"
```

[xh]: https://httpie.io/cli
[httpie]: https://httpie.io/cli
