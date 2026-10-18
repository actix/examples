# Automatic Let's Encrypt TLS Using Rustls

Uses an [ACME client library](https://github.com/x52dev/acme-rfc8555) to obtain a TLS certificate for a domain and start an HTTPS server with Rustls.

## Prerequisites

- A domain that resolves to this server.
- Public access to port `80` for the ACME HTTP challenge and port `443` for HTTPS.
- Permission to bind ports `80` and `443`.

## Usage

```sh
cd https-tls/acme-letsencrypt
```

Set `DOMAIN_NAME` and `CONTACT_EMAIL` in `src/main.rs`. For development, change the ACME directory to `DirectoryUrl::LetsEncryptStaging` as indicated in the code.

```sh
cargo run
```

Open your configured domain over HTTPS after certificate issuance succeeds. The source code describes the certificate renewal and restart process.
