# SQLX User CRUD

## Purpose

This application demonstrates the how to implement a common design for CRUDs in, potentially, a system of microservices.
The design pattern is akin to MVC (model, view, controller) minus the view.
This type of application is commonly developed in languages like Java or C#, so the objective in its development has been
to demonstrate a way to simply implement something familiar to C#'s `DbContext` that contains a `DbSet<T>` of each entity
to be stored in the database.

## Data Design

The database layout is simple, and illustrates a common relationship. The the database there are 3 tables: (1) `Users` (2)
`Groups` and (3) `UsersToGroups`. `Groups` and `Users` have a many-to-many relationship. This many-to-many relationship
also exists in the *Rust* application where a `User` has a `Vec<Group>`.

## Application Design

The application itself has been designed with a modular approach. It is common for *Rust* projects (specifically for the
purpose of demonstration) to consolidate code into only a handful of files. Dividing this project into multiple modules, 
and many small files was intentional, as it makes the overall architecture clear.

The architecture is that of a trivial crud. There are two key layers: (1) the Controller and (2) the DAO (Data Access 
Object). The controller layer organizes the interaction between the DAO and the incoming/outgoing HTTP. Based on the 
various DAO responses, specific HTTP responses are provided.

## Test Coverage

This application uses an integration testing style to provided test coverage for all methods. Note, not all method paths
are fully tested. All expected paths of behavior are tested. In a more comprehensive system, there would be reason to test
all permutations of behavior. These test serve as an example for what is sufficient test coverage for an initial 
application.

## Setup

First, have MySQL installed and running.

Next, run the `schema.sql` script. On Linux, this can be done in the terminal:
```shell
sudo mysql -u root < schema.sql
```

When `schema.sql` has executed successfully, run the tests:
```shell
cargo test
```

After the tests have completed and all pass, startup the application:
```shell
cargo run
```