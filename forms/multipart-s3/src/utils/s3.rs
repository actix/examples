use std::{env, fs, io::Read as _};

use aws_config::SdkConfig as AwsConfig;
use aws_sdk_s3::{types::ByteStream, Client as S3Client};

#[derive(Debug, Clone)]
pub struct Client {
    s3: S3Client,
    bucket_name: String,
}

impl Client {
    // construct S3 testing client
    pub fn new(config: &AwsConfig) -> Client {
        Client {
            s3: S3Client::new(config),
            bucket_name: env::var("AWS_S3_BUCKET_NAME").unwrap(),
        }
    }

    pub fn url(&self, key: &str) -> String {
        format!(
            "https://{}.s3.{}.amazonaws.com/{key}",
            env::var("AWS_S3_BUCKET_NAME").unwrap(),
            env::var("AWS_REGION").unwrap(),
        )
    }

    pub async fn put_object(&self, local_path: &str, key: &str) -> String {
        let mut file = fs::File::open(local_path).unwrap();

        let mut contents =
            Vec::with_capacity(file.metadata().map(|md| md.len()).unwrap_or(1024) as usize);
        file.read_to_end(&mut contents).unwrap();

        let _res = self
            .s3
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(ByteStream::from(contents))
            .send()
            .await
            .expect("Failed to put test object");

        self.url(key)
    }

    pub async fn delete_object(&self, key: &str) {
        self.s3
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .expect("Couldn't delete object");
    }
}
