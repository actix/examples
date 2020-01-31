# async_pg example

## This example illustrates

- `tokio_postgres`
- use of `tokio_pg_mapper` for postgres data mapping
- `deadpool_postgres` for connection pooling
- `dotenv` + `config` for configuration

## Instructions

1. Create database user

    ```shell
    createuser -P test_user
    ```

    Enter a password of your choice. The following instructions assume you
    used `testing` as password.

    This step is **optional** and you can also use an existing database user
    for that. Just make sure to replace `test_user` by the database user
    of your choice in the following steps and change the `.env` file
    containing the configuration accordingly.

2. Create database

    ```shell
    createdb -O test_user testing_db
    ```

3. Initialize database

    ```shell
    psql -f sql/schema.sql testing_db
    ```

    This step can be repeated and clears the database as it drops and
    recreates the schema `testing` which is used within the database.

4. Create `.env` file:

    ```ini
    SERVER_ADDR=127.0.0.1:8080
    PG.USER=test_user
    PG.PASSWORD=testing
    PG.HOST=127.0.0.1
    PG.PORT=5432
    PG.DBNAME=testing_db
    PG.POOL.MAX_SIZE=16
    ```

5. Run the server:

    ```shell
    cargo run
    ```

6. Using a different terminal send an HTTP POST request to the running server:

    ```shell
    echo '{"email": "ferris@thecrab.com", "first_name": "ferris", "last_name": "crab", "username": "ferreal"}' | http -f --json --print h POST http://127.0.0.1:8080/users
    ```

    **...or using curl...**

    ```shell
    curl -d '{"email": "ferris@thecrab.com", "first_name": "ferris", "last_name": "crab", "username": "ferreal"}' -H 'Content-Type: application/json' http://127.0.0.1:8080/users
    ```

    A unique constraint exists for username, so sending this request twice
    will return an internal server error (HTTP 500).
