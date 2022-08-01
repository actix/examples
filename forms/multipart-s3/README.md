# Multipart + AWS S3

Upload a file in multipart form to AWS S3 using [AWS S3 SDK](https://crates.io/crates/aws-sdk-s3).

# Usage

```sh
cd forms/multipart-s3
```

1. copy `.env.example` to `.env`
1. edit `.env` key `AWS_REGION` = your_bucket_region
1. edit `.env` key `AWS_ACCESS_KEY_ID` = your_key_id
1. edit `.env` key `AWS_SECRET_ACCESS_KEY` = your_key_secret
1. edit `.env` key `AWS_S3_BUCKET_NAME` = your_bucket_name

The AWS SDK automatically reads these environment variables to configure the S3 client.

```sh
cargo run
```

Go to <http://localhost:8080> in you browser.

Or, start the upload using [HTTPie]:

```sh
http --form POST :8080/ file@Cargo.toml
http --form POST :8080/ file@Cargo.toml file@README.md meta='{"namespace":"foo"}'

http GET :8080/file/<key_from_upload>
```

Or, using cURL:

```sh
curl -X POST http://localhost:8080/ -F 'file=@Cargo.toml'
curl -X POST http://localhost:8080/ -F 'file=@Cargo.toml' -F 'file=@README.md' -F 'meta={"namespace":"foo"}'

curl http://localhost:8080/file/<key_from_upload>
```

[httpie]: https://httpie.org
[curl]: https://curl.haxx.se
