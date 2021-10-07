# Docker sample

## Build image

```shell
docker build -t docker_sample .
```

## Run built image

```shell
docker run -d -p 5000:5000 docker_sample
# and the server should start instantly
curl http://localhost:5000
```

## Running unit tests

```shell
docker build -t docker_sample:test --target base .
docker run --rm docker_sample:test
```
