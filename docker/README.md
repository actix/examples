# Docker

Build and run an Actix Web server in Docker.

## Prerequisites

- Docker.

## Usage

From the repository root, change to the example directory:

```sh
cd docker
```

The build uses only the files in this directory. `--pull` checks for the latest stable Rust image before building.

### Build The Image

```sh
docker build --pull -t actix-docker .
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
docker build --pull --target builder -t actix-docker:test .
docker run --rm --workdir /code actix-docker:test cargo test
```
