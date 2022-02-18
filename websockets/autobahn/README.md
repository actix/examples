# WebSocket Autobahn Test Server

WebSocket server for the [Autobahn WebSocket protocol testsuite](https://github.com/crossbario/autobahn-testsuite).

## Usage

### Server

```sh
cd websockets/autobahn
cargo run
```

### Running Autobahn Test Suite

Running the autobahn test suite is easiest using the docker image as explained on the [autobahn test suite repo](https://github.com/crossbario/autobahn-testsuite#using-the-testsuite-docker-image).

After starting the server, in the same directory, run the test suite in "fuzzing client" mode:

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

Results are written to the `reports/servers` directory for viewing.
