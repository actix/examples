# websocket

Websocket server for autobahn suite testing.

## Usage

### server

```bash
cd examples/websocket-autobahn
cargo run --bin websocket-autobahn-server
```

### Running Autobahn Test Suite

Follow the steps from the [autobahn-testsuite repo](https://github.com/crossbario/autobahn-testsuite#using-the-testsuite-docker-image)
to install the necessary prerequisites.

For example:

```bash
virtualenv ~/wstest
source ~/wstest/bin/activate
pip install autobahntestsuite
```

Once prerequisites are installed, run the test suite:

```bash
# In one window, start the server
cd examples/websocket-autobahn
cargo run --bin websocket-autobahn-server

# In another window, start the tests
cd ~
mkdir test
cd test
wstest -m fuzzingclient
```
