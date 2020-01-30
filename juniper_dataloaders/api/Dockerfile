FROM ekidd/rust-musl-builder AS build
WORKDIR /usr/src/
USER root

# install rustup/cargo
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH /root/.cargo/bin:$PATH

# Add compilation target for later scratch container
ENV RUST_TARGETS="x86_64-unknown-linux-musl"
RUN rustup target install x86_64-unknown-linux-musl

# Creating a placeholder project
RUN USER=root cargo new rust-graphql
WORKDIR /usr/src/rust-graphql

# moving deps info
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Caching deps
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN rm target/x86_64-unknown-linux-musl/release/deps/rust*

# Replacing with actual src
RUN rm src/*.rs
COPY ./src ./src

# Only code changes should need to compile
RUN cargo build --target x86_64-unknown-linux-musl --release

# This creates a TINY container with the executable! Like 4-5mb srsly
FROM scratch
COPY --from=build /usr/src/rust-graphql/target/x86_64-unknown-linux-musl/release/rust-graphql .
USER 1000
CMD ["./rust-graphql"]
