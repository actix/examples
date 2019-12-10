# select build image
FROM rust:latest as build

# create a new empty shell project
RUN USER=root cargo new --bin diesel-example
WORKDIR /diesel-example

# copy over your manifests
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN cargo build --release

# our final base
FROM rust:latest

# copy the build artifact from the build stage
COPY --from=build /diesel-example/target/release/diesel-example .

# install diesel-example cli
RUN cargo install diesel_cli --no-default-features --features postgres

# copy migrations
COPY ./migrations ./migrations

# set the startup command to run your binary
CMD diesel migration run && ./diesel-example

# open port
EXPOSE 3000
