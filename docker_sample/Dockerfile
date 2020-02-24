FROM rust:1 as builder

COPY . .

RUN cargo build --release

FROM rust:1-slim-stretch

COPY --from=builder /target/release/docker_sample .

RUN ls -la /docker_sample

EXPOSE 5000

ENTRYPOINT ["/docker_sample"]
