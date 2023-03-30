# actix-sse

Example of server-sent events, aka `EventSource`, with actix web.

```sh
cd server-sent-events
cargo run
```

Open http://127.0.0.1:8080/ with a browser, then send events with another HTTP client:

```sh
curl -X POST 127.0.0.1:8080/broadcast/my_message
```

_my_message_ should appear in the browser with a timestamp.

## Performance

This implementation can serve thousands of clients on a 2021 MacBook with no problems.

Run `node ./benchmark.js` to benchmark your own system:

```sh
$ node benchmark.js
Connected: 1000, connection time: 201 ms, total broadcast time: 20 ms^C⏎
```

### Error _Too many open files_

You may be limited to a maximal number of connections (open file descriptors). Setting maximum number of open file descriptors to 2048:

```sh
ulimit -n 2048
```

Test maximum number of open connections with `node ./drain.js`:

```sh
$ node drain.js
Connections dropped: 10450, accepting connections: false^C⏎
```

_Accepting connections_ indicates whether resources for the server have been exhausted.
