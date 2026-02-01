use std::env;

use aws_config::SdkConfig as AwsConfig;
use aws_sdk_s3::{Client as S3Client, primitives::ByteStream};
use futures_util::{StreamExt as _, stream};
use tokio::{fs, io::AsyncReadExt as _};

use crate::{TempFile, UploadedFile};

/// S3 client wrapper to expose semantic upload operations.
#[derive(Debug, Clone)]
pub struct Client {
    s3: S3Client,
    bucket_name: String,
}

impl Client {
    /// Construct S3 client wrapper.
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

    pub async fn fetch_file(&self, key: &str) -> Option<(u64, ByteStream)> {
        let object = self
            .s3
            .get_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .ok()?;

        Some((
            object
                .content_length()
                .and_then(|len| len.try_into().ok())
                .expect("file has invalid size"),
            object.body,
        ))
    }

    pub async fn upload_files(
        &self,
        temp_files: Vec<TempFile>,
        key_prefix: &str,
    ) -> actix_web::Result<Vec<UploadedFile>> {
        let uploaded_files = stream::iter(temp_files)
            .map(|file| self.upload_and_remove(file, key_prefix))
            // upload files concurrently, up to 2 at a time
            .buffer_unordered(2)
            .collect()
            .await;

        Ok(uploaded_files)
    }

    async fn upload_and_remove(&self, file: TempFile, key_prefix: &str) -> UploadedFile {
        let uploaded_file = self.upload(&file, key_prefix).await;
        tokio::fs::remove_file(file.file.path()).await.unwrap();
        uploaded_file
    }

    async fn upload(&self, file: &TempFile, key_prefix: &str) -> UploadedFile {
        let filename = file.file_name.as_deref().expect("TODO");
        let key = format!("{key_prefix}{filename}");
        let s3_url = self
            .put_object_from_file(file.file.path().to_str().unwrap(), &key)
            .await;
        UploadedFile::new(filename, key, s3_url)
    }

    async fn put_object_from_file(&self, local_path: &str, key: &str) -> String {
        let mut file = fs::File::open(local_path).await.unwrap();

        let size_estimate = file
            .metadata()
            .await
            .map(|md| md.len())
            .unwrap_or(1024)
            .try_into()
            .expect("file too big");

        let mut contents = Vec::with_capacity(size_estimate);
        file.read_to_end(&mut contents).await.unwrap();

        let _res = self
            .s3
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(ByteStream::from(contents))
            .send()
            .await
            .expect("Failed to put object");

        self.url(key)
    }

    /// Attempts to deletes object from S3. Returns true if successful.
    pub async fn delete_file(&self, key: &str) -> bool {
        self.s3
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .is_ok()
    }
}
