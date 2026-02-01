# MongoDB

Simple example of MongoDB usage with Actix Web. For more information on the MongoDB Rust driver, visit the [documentation](https://docs.rs/mongodb/3) and [source code](https://github.com/mongodb/mongo-rust-driver).

## Usage

### Install MongoDB

Visit the [MongoDB Download Center](https://www.mongodb.com/try) for instructions on how to use MongoDB Atlas or set up MongoDB locally.

### Set an environment variable

The example code creates a client with the URI set by the `MONGODB_URI` environment variable. The default URI for a standalone `mongod` running on localhost is "mongodb://localhost:27017". For more information on MongoDB URIs, visit the [connection string](https://docs.mongodb.com/manual/reference/connection-string/) entry in the MongoDB manual.

### Run the example

To execute the example code, run `cargo run` in the `databases/mongodb` directory.
