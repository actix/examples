# Multipart File Upload

Uploads files with the Actix Web multipart form extractor.

## Usage

```sh
cd forms/multipart
cargo run
```

Open <http://localhost:8080> in your browser and upload one or more files.

## Result

The files are saved in `forms/multipart/tmp`.

This example uses `unwrap` for file persistence and can panic if a file cannot be saved.
