# Multipart + AWS S3

Upload a file in multipart form to AWS S3 using [AWS S3 SDK](https://crates.io/crates/aws-sdk-s3).

## Usage

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

Open <http://localhost:8080> in your browser.

Or, start the upload using [HTTPie]:

```sh
http --form POST :8080/ namespace=foo file@Cargo.toml
http --form POST :8080/ namespace=foo file@Cargo.toml file@README.md

http GET :8080/file/uploads/foo/Cargo.toml
```

Or, using curl:

```sh
curl http://localhost:8080/ -F 'namespace=foo' -F 'file=@Cargo.toml'
curl http://localhost:8080/ -F 'namespace=foo' -F 'file=@Cargo.toml' -F 'file=@README.md'

curl http://localhost:8080/file/uploads/foo/Cargo.toml
```

[HTTPie]: https://httpie.io/cli
[curl]: https://curl.se
