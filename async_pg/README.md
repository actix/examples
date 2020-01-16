This example illustrates:
  - tokio_postgres
  - use of tokio_pg_mapper for postgres data mapping
  - deadpool_postgres for connection pooling


# Instructions 
1. Set up the testing database by running /sql/create_db.sh
2. `cargo run`
3. from the command line (linux), POST a user to the endpoint:
	echo '{"email": "ferris@thecrab.com", "first_name": "ferris", "last_name": "crab", "username": "ferreal"}' | http -f --json --print h POST http://127.0.0.1:8080/users
	- a unique constraint exists for username, so running this twice will return a 500
