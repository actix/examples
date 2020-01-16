# async_pg example

## This example illustrates

- `tokio_postgres`
- use of `tokio_pg_mapper` for postgres data mapping
- `deadpool_postgres` for connection pooling
- `dotenv` + `config` for configuration

## Instructions

1. Set up the testing database by running the included script:

    ```shell
    ./sql/create_db.sh
    ```

2. Create `.env` file:

    ```ini
    SERVER_ADDR=127.0.0.1:8080
    PG.USER=test_user
    PG.PASSWD=testing
    PG.HOST=127.0.0.1
    PG.PORT=5432
    PG.DBNAME=testing_db
    PG.POOL.MAX_SIZE=16
    ```

3. Run the server:

    ```shell
    cargo run
    ```

4. Using a different terminal send a HTTP POST request to the running server:

    ```shell
    echo '{"email": "ferris@thecrab.com", "first_name": "ferris", "last_name": "crab", "username": "ferreal"}' | http -f --json --print h POST http://127.0.0.1:8080/users
    ```

    A unique constraint exists for username, so sending this request twice
    will return an internal server error (HTTP 500).
