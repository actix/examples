# Docker

Build and run an Actix Web server in Docker.

## Prerequisites

- Docker.

## Usage

Run the build commands from the repository root. The build uses the shared workspace metadata and lockfile. `--pull` checks for the latest stable Rust image before building.

### Build The Image

```sh
docker build --pull --file docker/Dockerfile -t actix-docker .
```

### Server

```sh
docker run --rm -p 8080:8080 actix-docker
```

In another terminal, send a request:

```sh
curl http://localhost:8080/
```

## Tests

```sh
docker build --pull --file docker/Dockerfile --target builder -t actix-docker:test .
docker run --rm --workdir /code actix-docker:test cargo test --package docker_sample
```
