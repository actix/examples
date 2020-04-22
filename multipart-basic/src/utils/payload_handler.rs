use actix_multipart::{Field, Multipart};
use actix_web::{web};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{str};



#[derive(Debug, Clone)]
pub struct UploadedFiles {
    pub name: String,
    pub path: String,
}
impl UploadedFiles {
    fn new(filename: &str) -> UploadedFiles {
        UploadedFiles {
            name: filename.to_string(),
            path: format!("./files/{}", filename),
        }
    }

}

#[derive(Deserialize, Serialize, Debug)]
pub struct FormOut {
    title: String,
    description: String, 
    count: u32
}

pub async fn split_payload(payload: &mut Multipart) -> (FormOut, Vec<UploadedFiles>) {
    let mut files: Vec<UploadedFiles> = Vec::new();

    /* fill with default values for now */
    let mut form : FormOut=FormOut {
        title: "".to_string(),
        description: "".to_string(),
        count: 0
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect(" split_payload err");
        let content_type = field.content_disposition().unwrap();
        let name = content_type.get_name().unwrap();
        if name != "file" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                /* convert bytes to string and print it  (just for testing) */
                if let Ok(s)=str::from_utf8(&data){
                    println!("{:?}", s);
                };

                /* all not file fields of your form (feel free to fix this mess) */
                if name=="title" {
                    if let Ok(s)=str::from_utf8(&data){
                        form.title=s.to_string();
                    }
                
                }
                else if name=="description" {
                    if let Ok(s)=str::from_utf8(&data){
                        form.description=s.to_string();
                    }
                }
                else if name=="count"{
                    if let Ok(s)=str::from_utf8(&data){
                        /* bytes to string */
                        let numstr: String=s.to_string();
                        /* string to u32 number */
                        form.count=numstr.parse().expect("not a number");
                    };
                }
            }
        } else {
            match content_type.get_filename() {
                Some(filename) => {
                    let file = UploadedFiles::new(filename);
                    let file_path = file.path.clone();
                    let mut f = web::block(move || std::fs::File::create(&file_path))
                        .await
                        .unwrap();
                    while let Some(chunk) = field.next().await {
                        let data = chunk.unwrap();
                        f = web::block(move || f.write_all(&data).map(|_| f))
                            .await
                            .unwrap();
                    }
                    files.push(file.clone());
                }
                None => {
                    println!("file none");
                }
            }
        }
    }
    (form, files)
}

