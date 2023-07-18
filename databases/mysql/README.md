# MySQL

This RESTful Actix Web API illustrates how to use a MySQL database as a data source for various endpoints.

You'll need to have a MySQL (or compatible) server running on your machine to test this example.

## Usage

All the following commands assume that your current working directory is _this_ directory. I.e.:

```console
$ pwd
.../databases/mysql
```

1. Create database and tables:

   The `sql` directory contains the SQL files used for database setup:

   ```sh
   mysql -u root -p < sql/0_create_database.sql
   mysql -u root -p my_bank < sql/1_bank_details.sql
   mysql -u root -p my_bank < sql/2_branch_details.sql
   mysql -u root -p my_bank < sql/3_teller_details.sql
   mysql -u root -p my_bank < sql/4_customer_details.sql
   ```

   For each step you will be prompted for the root user's password. If there's no password set on the root use, just hit enter again.

1. Create a `.env` file in this this directory:

   ```ini
   SERVER_ADDR=127.0.0.1:8080
   MYSQL_USER=root
   MYSQL_PASSWORD=<password>
   MYSQL_HOST=127.0.0.1
   MYSQL_PORT=3306
   MYSQL_DBNAME=my_bank
   ```

   Update "MYSQL_USER" and "MYSQL_PASSWORD" values with the correct MySQL user/password.

1. Run the server:

   ```sh
   cargo run
   ```

1. Using a different terminal send requests to the running server. For example, using [HTTPie]:

   ```sh
   http POST :8080/bank bank_name="Bank ABC" country="Kenya"

   http :8080/bank
   ```

   See [the API documentation pages](./apis/) for more info.

[HTTPie]: https://httpie.io/cli
