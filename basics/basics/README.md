# Basics

## Usage

### Server

```sh
cd basics/basics
cargo run
# Server: http://localhost:8080
```

### Browser Client

- [http://localhost:8080/async-body/bob](http://localhost:8080/async-body/bob)
- [http://localhost:8080/user/bob](http://localhost:8080/user/bob) text/plain download
- [http://localhost:8080/test](http://localhost:8080/test) (return status switch GET or POST or other)
- [http://localhost:8080/favicon](http://localhost:8080/favicon)
- [http://localhost:8080/welcome](http://localhost:8080/welcome)
- [http://localhost:8080/notexit](http://localhost:8080/notexit) display 404 page
- [http://localhost:8080/error](http://localhost:8080/error) Panic after request
