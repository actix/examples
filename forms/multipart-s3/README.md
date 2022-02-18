# multipart+s3
Upload a file in multipart form to aws s3(https://github.com/rusoto/rusoto)   
Receive multiple data in multipart form in JSON format and receive it as a struct.   

# usage
```
cd forms/multipart-s3
```
1. copy .env.example .env
2. edit .env  AWS_ACCESS_KEY_ID=you_key
3. edit .env  AWS_SECRET_ACCESS_KEY=you_key
4. edit .env  AWS_S3_BUCKET_NAME=you_key


# Running Server
```
cd forms/multipart-s3
cargo run (or ``cargo watch -x run``)
```
http://localhost:3000


# using other regions
- https://www.rusoto.org/regions.html
- https://docs.rs/rusoto_core/0.42.0/rusoto_core/enum.Region.html
