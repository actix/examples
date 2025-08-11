# Casbin

Basic integration of [Casbin-RS](https://github.com/casbin/casbin-rs) with [RBAC](https://en.wikipedia.org/wiki/Role-based_access_control) for Actix Web.

## Usage

```sh
cd auth/casbin
```

Modify the files in the `rbac` directory and the code in the `src` directory as required.

## Running Server

```sh
cd auth/casbin
cargo run # or: cargo watch -x run

# Started http server: 127.0.0.1:8080
```

In this example, you can get the:

- successful result at [http://localhost:8080/success](http://localhost:8080/success) (accessible)
- failed result at [http://localhost:8080/fail](http://localhost:8080/fail) (inaccessible, `ERR_EMPTY_RESPONSE`).

## Others

- For more related examples of [Casbin-RS](https://github.com/casbin/casbin-rs): <https://github.com/casbin-rs/examples>
