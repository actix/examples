# WebSocket Autobahn Test Server

WebSocket server for the [Autobahn WebSocket protocol testsuite](https://github.com/crossbario/autobahn-testsuite).

## Usage

### Server

```sh
cd websockets/autobahn
cargo run
```

### Running autobahn test suite

Running the autobahn test suite is easiest using the docker image as explained on the [autobahn test suite repo](https://github.com/crossbario/autobahn-testsuite#using-the-testsuite-docker-image).

After starting the server, in the same directory, run the test suite in "fuzzing client" mode.

#### Docker

```sh
docker run -it --rm \
    -v "${PWD}/config:/config" \
    -v "${PWD}/reports:/reports" \
    --network host \
    --name autobahn \
    crossbario/autobahn-testsuite \
    wstest \
    --spec /config/fuzzingclient.json \
    --mode fuzzingclient
```

#### Podman

```sh
podman run -it --rm \
    -v "${PWD}/config":/config \
    -v "${PWD}/reports":/reports \
    --network host \
    --name autobahn \
    crossbario/autobahn-testsuite \
    wstest \
    --spec /config/fuzzingclient-podman.json \
    --mode fuzzingclient
```

If you run it with `selinux` enabled, then

```sh
podman run -it --rm \
    -v "${PWD}/config":/config:z \
    -v "${PWD}/reports":/reports:z \
    --network host \
    --name autobahn \
    crossbario/autobahn-testsuite \
    wstest \
    --spec /config/fuzzingclient-podman.json \
    --mode fuzzingclient
```
