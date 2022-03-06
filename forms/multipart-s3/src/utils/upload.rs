use crate::utils::s3::Client;
use actix_multipart::{Field, Multipart};
use actix_web::{web, web::Bytes, Error};
use futures_util::StreamExt as _;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::io::Write;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UploadFile {
    pub filename: String,
    pub key: String,
    pub url: String,
}

impl From<Tmpfile> for UploadFile {
    fn from(tmp_file: Tmpfile) -> Self {
        UploadFile {
            filename: tmp_file.name,
            key: tmp_file.s3_key,
            url: tmp_file.s3_url,
        }
    }
}

/*
1. savefile
2. s3 upload -> upload_data
3. deletefile
*/
#[derive(Debug, Clone)]
pub struct Tmpfile {
    pub name: String,
    pub tmp_path: String,
    pub s3_key: String,
    pub s3_url: String,
}
impl Tmpfile {
    fn new(filename: &str) -> Tmpfile {
        Tmpfile {
            name: filename.to_string(),
            tmp_path: format!("./tmp/{}", filename),
            s3_key: "".to_string(),
            s3_url: "".to_string(),
        }
    }

    async fn s3_upload_and_tmp_remove(&mut self, s3_upload_key: String) {
        self.s3_upload(s3_upload_key).await;
        self.tmp_remove();
    }

    async fn s3_upload(&mut self, s3_upload_key: String) {
        let key = format!("{}{}", &s3_upload_key, &self.name);
        self.s3_key = key.clone();
        let url: String = Client::new().put_object(&self.tmp_path, &key.clone()).await;
        self.s3_url = url;
    }

    fn tmp_remove(&self) {
        std::fs::remove_file(&self.tmp_path).unwrap();
    }
}

pub async fn split_payload(payload: &mut Multipart) -> (Bytes, Vec<Tmpfile>) {
    let mut tmp_files: Vec<Tmpfile> = Vec::new();
    let mut data = Bytes::new();

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect(" split_payload err");
        let content_type = field.content_disposition();
        let name = content_type.get_name().unwrap();
        if name == "data" {
            while let Some(chunk) = field.next().await {
                data = chunk.expect(" split_payload err chunk");
            }
        } else {
            match content_type.get_filename() {
                Some(filename) => {
                    let tmp_file = Tmpfile::new(&sanitize_filename::sanitize(&filename));
                    let tmp_path = tmp_file.tmp_path.clone();
                    let mut f = web::block(move || std::fs::File::create(&tmp_path))
                        .await
                        .unwrap()
                        .unwrap();
                    while let Some(chunk) = field.next().await {
                        let data = chunk.unwrap();
                        f = web::block(move || f.write_all(&data).map(|_| f))
                            .await
                            .unwrap()
                            .unwrap();
                    }
                    tmp_files.push(tmp_file.clone());
                }
                None => {
                    println!("file none");
                }
            }
        }
    }
    (data, tmp_files)
}

pub async fn save_file(
    tmp_files: Vec<Tmpfile>,
    s3_upload_key: String,
) -> Result<Vec<UploadFile>, Error> {
    let mut arr: Vec<UploadFile> = Vec::with_capacity(tmp_files.len());

    for item in tmp_files {
        let mut tmp_file: Tmpfile = item.clone();
        tmp_file
            .s3_upload_and_tmp_remove(s3_upload_key.clone())
            .await;
        arr.push(UploadFile::from(tmp_file));
    }
    Ok(arr)
}

#[allow(unused)]
pub async fn delete_object(list: Vec<String>) {
    for key in list {
        Client::new().delete_object(key).await;
    }
}
