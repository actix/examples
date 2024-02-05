# Docker sample

## Build image

```shell
docker build -t actix-docker .
```

## Run built image

```shell
docker run -d -p 8080:8080 actix-docker
# and the server should start instantly
curl http://localhost:8080
```

## Running unit tests

```shell
docker build -t actix-docker:test .
docker run --rm actix-docker:test
```
