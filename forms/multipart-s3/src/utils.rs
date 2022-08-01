use actix_multipart::{Field, Multipart};
use actix_web::web::{Bytes, BytesMut};
use futures_util::StreamExt as _;
use tokio::{fs, io::AsyncWriteExt as _};

use crate::TempFile;

/// Returns tuple of `meta` field contents and a list of temp file info to upload.
pub async fn split_payload(payload: &mut Multipart) -> (Bytes, Vec<TempFile>) {
    let mut meta = Bytes::new();
    let mut temp_files = vec![];

    while let Some(item) = payload.next().await {
        let mut field = item.expect("split_payload err");
        let cd = field.content_disposition();

        if matches!(cd.get_name(), Some(name) if name == "meta") {
            // if field name is "meta", just collect those bytes in-memory and return them later
            meta = collect_meta(&mut field).await;
        } else {
            match cd.get_filename() {
                Some(filename) => {
                    // if file has a file name, we stream the field contents into a temp file on
                    // disk so that large uploads do not exhaust memory

                    // create file info
                    let file_info = TempFile::new(filename);

                    // create file on disk from file info
                    let mut file = fs::File::create(file_info.path()).await.unwrap();

                    // stream field contents to file
                    while let Some(chunk) = field.next().await {
                        let data = chunk.unwrap();
                        file.write_all(&data).await.unwrap();
                    }

                    // return file info
                    temp_files.push(file_info);
                }

                None => {
                    log::warn!("field {:?} is not a file", cd.get_name());
                }
            }
        }
    }

    (meta, temp_files)
}

async fn collect_meta(field: &mut Field) -> Bytes {
    let mut buf = BytesMut::new();

    while let Some(chunk) = field.next().await {
        let chunk = chunk.expect("split_payload err chunk");
        buf.extend(chunk);
    }

    buf.freeze()
}
