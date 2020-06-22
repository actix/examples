## Usage:
This is an example on constructing async state with `App::data_factory`

## Reason:
`data_factory` would make sense in these situations:
- When async state not necessarily have to be shared between workers/threads.

- When async state would spawn tasks on `actix-rt`. If we centralized the state there could be a possibilitythe tasks get a very unbalanced distribution on the workers/threads
(`actix-rt` would spawn tasks on local thread whenever it's called)

## Requirement:
- `rustc 1.43 stable`
- `redis` server listen on `127.0.0.1:6379`(or make change to const var `REDIS_URL` in `main.rs`)

## Endpoints:
- use a work load generator(e.g wrk) to benchmark the end points:

        http://127.0.0.1:8080/pool   prebuilt shared redis pool
        http://127.0.0.1:8080/pool2  data_factory redis pool

## Context:
The real world difference can be vary by the work you are doing but in general it's a good idea to
spread your *identical* async tasks evenly between threads and have as little cross threads synchronization as possible.
