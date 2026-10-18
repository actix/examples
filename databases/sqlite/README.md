# SQLite

Demonstrates sequential and parallel database queries with Actix Web and SQLite.

## Prerequisites

- SQLite 3 and its development libraries.

On Ubuntu:

```sh
sudo apt install libsqlite3-dev sqlite3
```

## Usage

### Database Setup

```sh
cd databases/sqlite
bash db/setup_db.sh
```

This creates `weather.db` in `databases/sqlite`.

### Server

```sh
cargo run
```

### Browser Client

- <http://localhost:8080/asyncio_weather>
- <http://localhost:8080/parallel_weather>

### SQLite Client

In another terminal, from `databases/sqlite`:

```sh
sqlite3 weather.db
```

```console
sqlite> .tables
sqlite> select * from nyc_weather;
```
