# websocket

Websocket server for autobahn suite testing.

## Usage

### server

```bash
cd websockets/autobahn
cargo run --bin websocket-autobahn-server
```

### Running Autobahn Test Suite

Running the autobahn test suite is easiest using the docker image
as explained on the [autobahn-testsuite repo](https://github.com/crossbario/autobahn-testsuite#using-the-testsuite-docker-image).

First, start a server (see above). Then, run the test suite in fuzzingclient mode:

```bash
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
