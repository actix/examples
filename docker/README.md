# Docker

Build and run an Actix Web server in Docker.

## Prerequisites

- Docker.

## Usage

```sh
cd docker
```

### Build The Image

```sh
docker build -t actix-docker .
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
docker build --target builder -t actix-docker:test .
docker run --rm --workdir /code actix-docker:test cargo test
```
