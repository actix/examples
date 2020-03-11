use crate::rusoto_s3::S3;
use rusoto_core::{Region};
use rusoto_s3::{DeleteObjectRequest, PutObjectRequest, S3Client};
use std::io::Read;

pub struct Client {
    region: Region,
    s3: S3Client,
    bucket_name: String,
}

impl Client {
    // construct S3 testing client
    pub fn new() -> Client {
      let env_region = std::env::var("AWS_REGION").unwrap();
      let region = match env_region.as_ref() {
        "ap-east-1" => Region::ApEast1,
        "ap-northeast-1" => Region::ApNortheast1,
        "ap-northeast-2" => Region::ApNortheast2,
        "ap-south-1" => Region::ApSouth1,
        "ap-southeast-1" => Region::ApSoutheast1,
        "ap-southeast-2" => Region::ApSoutheast2,
        "ca-central-1" => Region::CaCentral1,
        "eu-central-1" => Region::EuCentral1,
        "eu-north-1" => Region::EuNorth1,
        "eu-west-1" => Region::EuWest1,
        "eu-west-2" => Region::EuWest2,
        "eu-west-3" => Region::EuWest3,
        "me-south-1" => Region::MeSouth1,
        "sa-east-1" => Region::SaEast1,
        "us-east-1" => Region::UsEast1,
        "us-east-2" => Region::UsEast2,
        "us-west-1" => Region::UsWest1,
        "us-west-2" => Region::UsWest2,
        // Default
        _ => Region::ApNortheast2
      };

      Client {
          region: region.to_owned(),
          s3: S3Client::new(region),
          bucket_name: std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
      }
    }

    pub fn url(&self, key: &str) -> String {
        format!(
            "https://{}.s3.{}.amazonaws.com/{}",
            std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
            std::env::var("AWS_REGION").unwrap(),
            key
        )
    }

    pub async fn put_object(&self, localfilepath: &str, key: &str) -> String {
        let mut file = std::fs::File::open(localfilepath).unwrap();
        let mut contents: Vec<u8> = Vec::new();
        file.read_to_end(&mut contents);
        let put_request = PutObjectRequest {
            bucket: self.bucket_name.to_owned(),
            key: key.to_owned(),
            body: Some(contents.into()),
            ..Default::default()
        };
        let _res = self
            .s3
            .put_object(put_request)
            .await
            .expect("Failed to put test object");

        self.url(key)
    }

    pub async fn delete_object(&self, key: String) {
        let delete_object_req = DeleteObjectRequest {
            bucket: self.bucket_name.to_owned(),
            key: key.to_owned(),
            ..Default::default()
        };

        let _res = self
            .s3
            .delete_object(delete_object_req)
            .await
            .expect("Couldn't delete object");
    }
}
