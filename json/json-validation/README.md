This is a contrived example intended to illustrate a few important Actix Web features.

_Imagine_ that you have a process that involves 3 steps. The steps here are dumb in that they do nothing other than call an HTTP endpoint that returns the json that was posted to it. The intent here is to illustrate how to chain these steps together as futures and return a final result in a response.

Actix Web features illustrated here include:

    1. handling json input param
    2. validating user-submitted parameters using the 'validator' crate
    2. `awc` client features:
          - POSTing json body
    3. chaining futures into a single response used by an asynch endpoint

### server

```sh
cd basics/json-validation
cargo run
# Started http server: 127.0.0.1:8080
```

Example query from the command line using httpie: `echo '{"id":"1", "name": "JohnDoe"}' | http 127.0.0.1:8080/something`
