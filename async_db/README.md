Getting started using databases with Actix web, asynchronously.

## Usage

### init database sqlite

From the root directory of this project:
```bash
bash db/setup_db.sh
```

This creates a sqlite database, weather.db, in the root.


### server

```bash
# if ubuntu : sudo apt-get install libsqlite3-dev
# if fedora : sudo dnf install libsqlite3x-devel
cargo run (or ``cargo watch -x run``)
# Started http server: 127.0.0.1:8080
```

### web client

[http://127.0.0.1:8080/asyncio_weather](http://127.0.0.1:8080/asyncio_weather)

[http://127.0.0.1:8080/parallel_weather](http://127.0.0.1:8080/parallel_weather)


### sqlite client

```bash
# if ubuntu : sudo apt-get install sqlite3
# if fedora : sudo dnf install sqlite3x
sqlite3 weather.db
sqlite> .tables
sqlite> select * from nyc_weather;
```

