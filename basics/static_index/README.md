# static_index

Demonstrates how to serve static files. Inside the `./static` folder you will find 2 subfolders:

* `root`: A tree of files that will be served at the web root `/`. This includes the `css` and `js` folders, each
  containing an example file.
* `images`: A list of images that will be served at `/images` path, with file listing enabled.

## Usage

```bash
$ cd examples/static_index
$ cargo run
```

This will start the server on port 8080, it can be viewed at [http://localhost:8080](http://localhost:8080).
