# Static Files

Demonstrates how to serve static files. Inside the `./static` folder you will find 2 subfolders:

- `root`: A tree of files that will be served at the web root `/`. This includes the `css` and `js` folders, each containing an example file.
- `images`: A list of images that will be served at `/images` path, with file listing enabled.

## Usage

```sh
cd basics/static-files
cargo run
```

### Available Routes

- [GET /](http://localhost:8080/)
- [GET /images](http://localhost:8080/images)
