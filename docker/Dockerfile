# NB: This is not a production-grade Dockerfile.

#################
## build stage ##
#################
FROM rust:1-slim-bookworm AS builder
WORKDIR /code

# Use the repository root as the build context for workspace metadata.
COPY . .

# Compile the Docker example from the workspace.
RUN cargo build --release --package docker_sample

###############
## run stage ##
###############
FROM bitnami/minideb:bookworm
WORKDIR /app

# copy server binary from build stage
COPY --from=builder /code/target/release/docker_sample docker_sample

# set user to non-root unless root is required for your app
USER 1001

# indicate what port the server is running on
EXPOSE 8080

# run server
CMD [ "/app/docker_sample" ]
