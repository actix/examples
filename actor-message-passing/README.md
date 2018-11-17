## Actix Actor Message Passsing

This is my example repository where I am attempting to understand how to
compose the output of various actors into something that actix_web can use
as a response.

The example shows how to construct a chain of actors (sync) in an asynchronous
way and provides an ergonomic way of doing so.  The important things to not are
the usage of a single error struct `struct SystemError` and the `flatten()`
method to coalesce the future response types.

### Main Endpoint Code
```rust
fn index((params, state): (Path<(i32, i32)>, State<AppState>)) -> FutureResponse<HttpResponse> {
    let add = params.0;
    let sub = params.1;

    state
        .incr
        .send(Add { input: add })
        .join(state.decr.send(Sub { input: sub }))
        .flatten()
        .and_then(move |(a, b)| state.sum.send(Sum { a, b }).flatten())
        .map(|sum| actix_web::HttpResponse::Ok().json(sum))
        .map_err(actix_web::Error::from)
        .responder()
}
```

### Error Handling
```rust
#[derive(Debug)]
pub enum SystemError {
    Decr(DecrError),
    Incr(IncrError),
    Mailbox(actix::MailboxError),
    Sum(SumError),
}

impl std::convert::From<actix::MailboxError> for SystemError {
    fn from(e: actix::MailboxError) -> Self {
        SystemError::Mailbox(e)
    }
}

impl std::convert::From<SystemError> for actix_web::Error {
    fn from(e: SystemError) -> Self {
        match e {
            SystemError::Mailbox(e) => {
                println!("MAIL BOX ERROR: {:?}", e);
                actix_web::error::ErrorInternalServerError(e)
            }
            SystemError::Decr(e) => {
                println!("DECR ERROR: {:?}", e);
                actix_web::error::ErrorBadRequest(e)
            }
            SystemError::Incr(e) => {
                println!("INCR ERROR: {:?}", e);
                actix_web::error::ErrorBadRequest(e)
            }
            SystemError::Sum(e) => {
                println!("SUM ERROR: {:?}", e);
                actix_web::error::ErrorBadRequest(e)
            }
        }
    }
}
```

## Usage

### run
```sh
cd examples/actor-message-passing
cargo run
# Started http server 127.0.0.1:8088
```

### Endpoints

* http://127.0.0.1:8088/{i32}/{i32} -> i32
