# udp-echo

## Usage

### server

```bash
cd examples/udp-echo
cargo run
# Started http server: 127.0.0.1:12345
```

### socat client
Copy port provided in server output and run following command to communicate
with the udp server:
```bash
socat - UDP4:localhost:12345
```
