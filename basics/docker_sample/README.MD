# Docker sample

## Build image

```shell
docker build -t docker_sample .
```

## Run built image

```shell
docker run -d -p 8080:8080 docker_sample
# and the server should start instantly
curl http://localhost:8080
```

## Running unit tests

```shell
docker build -t docker_sample:test .
docker run --rm docker_sample:test
```
