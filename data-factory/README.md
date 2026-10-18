# Async Data Factory

This is an example demonstrating the construction of async state with `App::data_factory`.

## Use Cases

Use of a `data_factory` would make sense in these situations:

- When async state does not necessarily have to be shared between workers/threads.
- When an async state would spawn tasks. If state was centralized there could be a possibility the tasks get an unbalanced distribution on the workers/threads.

## Context

The effect depends on the workload. Spread tasks with similar costs across worker threads and reduce synchronization between threads where possible.

## Usage

```sh
cd data-factory
cargo run
```

## Client

```sh
curl -i http://localhost:8080/
```

The handler reads the state created by `data_factory` and returns `204 No Content`.
