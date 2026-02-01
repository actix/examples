# Automatic Let's Encrypt TLS/HTTPS using OpenSSL

We use [`an ACME client library`](https://github.com/x52dev/acme-rfc8555) to auto-generate TLS/HTTPS certificates for a given domain and then start our real web server with the obtained certificate.

Process is explained in code.
