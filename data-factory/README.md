# Usage:

This is an example demonstrating the construction of async state with `App::data_factory`

## Reason:

Use of a `data_factory` would make sense in these situations:

- When async state does not necessarily have to be shared between workers/threads.

- When an async state would spawn tasks on `actix-rt`. If state was centralized there could be a possibility the tasks get an unbalanced distribution on the workers/threads (`actix-rt` would spawn tasks on local thread whenever it's called)

## Requirement:

- `rustc 1.58 stable`
- `redis` server listen on `127.0.0.1:6379`(or use `REDIS_URL` env argument when starting the example)

## Endpoints:

- use a work load generator(e.g wrk) to benchmark the end points:

        http://127.0.0.1:8080/pool   prebuilt shared redis pool
        http://127.0.0.1:8080/pool2  data_factory redis pool

## Context:

The real world difference can be vary by the work you are doing but in general it's a good idea to spread your _identical_ async tasks evenly between threads and have as little cross threads synchronization as possible.
