use tokio::fs;

/// Info for a temporary file to be uploaded to S3.
#[derive(Debug, Clone)]
pub struct TempFile {
    path: String,
    name: String,
}

impl TempFile {
    /// Constructs info container with sanitized file name.
    pub fn new(filename: &str) -> TempFile {
        let filename = sanitize_filename::sanitize(filename);

        TempFile {
            path: format!("./tmp/{filename}"),
            name: filename,
        }
    }

    /// Returns name of temp file.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns path to temp file.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Deletes temp file from disk.
    pub async fn delete_from_disk(self) {
        fs::remove_file(&self.path).await.unwrap();
    }
}
