# MySQL

This RESTful Actix Web API examples illustrates how to connect to MySQL database using a MySQL client library implemented in Rust; e.g., MySQL database driver.

Below APIs are supported:

- Add Bank
- Add Branch
- Add Teller
- Add Customer
- Get Bank
- Get Branch
- Get Teller
- Get Customer

The RESTful Actix Web API has below listed dependencies:

- [Actix Web](https://github.com/actix/actix-web) web framework for Rust
- [Serde](https://github.com/serde-rs/serde) for serializing and deserializing Rust data structures
- [MySQL Server](https://github.com/mysql/mysql-server) MySQL database server
- [MySQL](https://github.com/blackbeam/rust-mysql-simple) MySQL database driver

## Instructions

### NOTE:

You may need to ensure that you are running the commands with the correct MySQL user/password.

1. Access MySQL Server:

   Log in to the MySQL Server using a user account that has the `CREATE DATABASE` privilege.

1. Create database:

   ```sql
   CREATE DATABASE my_bank;
   ```

1. Create tables in the database:

   Directory "mysql\sql" contains below listed ".sql" files:

   - `bankdetails.sql`
   - `branch_details.sql`
   - `teller_details.sql`
   - `customer_details.sql`

   Copy the contents of each of the ".sql" and execute them separately on MySQL Server. This will create four tables in the database.

1. Create `.env` file:

   ```ini
   SERVER_ADDR=127.0.0.1:8080
   MYSQL_USER=XXX
   MYSQL_PASSWORD=XXX
   MYSQL_HOST=127.0.0.1
   MYSQL_PORT=3306
   MYSQL_DBNAME=my_bank
   ```

   Update "MYSQL_USER" and "MYSQL_PASSWORD" values with the correct MySQL user/password.
   If your password contains dollar sign "$", then remember to escape it eg "123$abc" will need to be changed to "123\\$abc"

1. Run the server:

   ```shell
   cargo run
   ```

1. Using a different terminal send an HTTP GET/POST requests to the running server:

   Directory "mysql/apis" contains below listed API's files:

   - `bank.txt`
   - `branch.txt`
   - `teller.txt`
   - `customer.txt`

   Copy the curl request on each of the ".txt" and execute them on separate terminals. Each ".txt" contains curl request and expected JSON response data.
