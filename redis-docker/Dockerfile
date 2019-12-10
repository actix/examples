# select build image
FROM rust:latest as build

# create a new empty shell project
RUN USER=root cargo new --bin actix_redis
WORKDIR /actix_redis

# copy over your manifests
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/actix_redis*
RUN cargo build --release

# our final base
FROM rust:latest

# copy the build artifact from the build stage
COPY --from=build /actix_redis/target/release/actix_redis .

# set the startup command to run your binary
CMD ["./actix_redis"]

# open port
EXPOSE 3000