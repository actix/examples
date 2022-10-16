# Async Data Factory

This is an example demonstrating the construction of async state with `App::data_factory`.

## Reason:

Use of a `data_factory` would make sense in these situations:

- When async state does not necessarily have to be shared between workers/threads.
- When an async state would spawn tasks. If state was centralized there could be a possibility the tasks get an unbalanced distribution on the workers/threads.

## Context

The real world difference can be vary by the work you are doing but in general it's a good idea to spread your similarly-expensive async tasks evenly between threads and have as little cross threads synchronization as possible.
