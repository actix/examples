# PostgreSQL

Demonstrates PostgreSQL queries and connection pooling with Actix Web.

This example uses:

- `tokio_postgres`
- use of `tokio_pg_mapper` for postgres data mapping
- `deadpool_postgres` for connection pooling
- `dotenvor` + `confik` for configuration

## Usage

```sh
cd databases/postgres
```

Start PostgreSQL before running these commands. Use a PostgreSQL administrator account for database setup. On many Linux distributions, you can prefix the setup commands with `sudo -u postgres`.

1. Create database user

   ```sh
   createuser -P test_user
   ```

   Enter a password of your choice. The following instructions assume you used `testing` as password.

   This step is **optional** and you can also use an existing database user for that. Just make sure to replace `test_user` by the database user of your choice in the following steps and change the `.env` file containing the configuration accordingly.

   An alternative using SQL:

   ```sql
   CREATE USER test_user WITH PASSWORD 'testing';
   ```

2. Create database

   ```sh
   createdb -O test_user testing_db
   ```

   An alternative using SQL:

   ```sql
   CREATE DATABASE testing_db OWNER test_user;
   ```

3. Initialize database

   ```sh
   psql -f sql/schema.sql testing_db
   ```

   This step can be repeated and clears the database as it drops and recreates the schema `testing` which is used within the database.

4. Grant privileges to new user

   Connect to `testing_db` as the administrator with `psql testing_db`, then run:

   ```sql
   GRANT ALL PRIVILEGES ON SCHEMA testing TO test_user;
   GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA testing TO test_user;
   GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA testing TO test_user;
   ```

5. Create `.env` file:

   ```ini
   SERVER_ADDR=127.0.0.1:8080
   PG__USER=test_user
   PG__PASSWORD=testing
   PG__HOST=127.0.0.1
   PG__PORT=5432
   PG__DBNAME=testing_db
   PG__POOL__MAX_SIZE=16
   ```

6. Run the server:

   ```sh
   cargo run
   ```

7. Using a different terminal send an HTTP POST request to the running server:

   ```sh
   http POST :8080/users email=ferris@thecrab.com first_name=ferris last_name=crab username=ferreal
   ```

   **...or using curl...**

   ```sh
   curl -i -d '{"email": "ferris@thecrab.com", "first_name": "ferris", "last_name": "crab", "username": "ferreal"}' -H 'Content-Type: application/json' http://127.0.0.1:8080/users
   ```

   A unique constraint exists for username, so sending this request twice will return an internal server error (HTTP 500).
