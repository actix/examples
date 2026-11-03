# Kernel TLS

Demonstrates Linux kernel TLS with a Rustls handshake and an Actix HTTP service.

## Prerequisites

Run on Linux with `CONFIG_TLS` enabled. Load the `tls` module if needed.

## Usage

```sh
cd https-tls/kernel-tls-handoff
cargo run
```

In another terminal:

```sh
curl --insecure --http1.1 --tlsv1.2 --tls-max 1.2 https://localhost:8443
```

The expected response is:

```text
Hello from Actix over kernel TLS!
```

The certificate is self-signed and is for local testing only. This example uses TLS 1.2 and AES-128-GCM because [ktls 6.0.2 does not provide TLS 1.3 key updates](https://docs.rs/ktls/6.0.2/ktls/fn.config_ktls_server.html).

If kTLS setup fails, the connection fails; there is no fallback to userspace TLS. A successful setup is logged.

If enabling the TLS ULP returns "No such file or directory", check that the running kernel provides TLS support. Containers use their host's kernel.

This example does not use `sendfile` or demonstrate NIC hardware offload.
