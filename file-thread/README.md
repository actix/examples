# file-thread

File operations is not so easy task in asynchronous environment. Operating
system does not provide standartized API for it or even did not have ANY solution.
This examples shows how to offload file operations to separate thread which
can be usable in some cases.

## Usage
The examples splitted to two binaries to simplify the code in a simple case
though the `tap` one works with any type of files. You can compare it with
diff to see the changes.

### simple file

```bash
cd examples/file-thread
cargo run --bin main Cargo.toml
```

### pipe
More interesting (and important) example is to connect with other process throw
pipe. You can do this on *nix systems.
```bash
mkfifo pipe
cargo run --bin main pipe&
socat - PIPE:pipe
```

### tap
Sometimes you need to prepare device to work in suitable mode. For example
tap device should be switched to blocking mode. You can try tap example.
```bash
cargo run --bin tap /dev/tap0 &
sudo ifconfig tap0 10.0.1.1 up
```
