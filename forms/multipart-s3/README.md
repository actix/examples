# Multipart + AWS S3

Upload a file in multipart form to AWS S3 using [AWS S3 SDK](https://crates.io/crates/aws-sdk-s3).  
Receive multiple data in multipart form in JSON format and receive it as a struct.

# Usage

```sh
cd forms/multipart-s3
```

1. copy .env.example .env
1. edit .env AWS_ACCESS_KEY_ID=your_key
1. edit .env AWS_SECRET_ACCESS_KEY=your_key
1. edit .env AWS_S3_BUCKET_NAME=your_chosen_region

```sh
cargo run
```

<http://localhost:8080>
