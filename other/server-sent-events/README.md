# actix-sse
Example of server-sent events, aka `EventSource`, with actix web.

```sh
cargo run
```

Open http://localhost:8080/ with a browser, then send events with another HTTP client:

```sh
curl localhost:8080/broadcast/my_message
```

*my_message* should appear in the browser with a timestamp.

## Performance
This implementation serve thousand of clients on a 2013 macbook air without problems.

Run [benchmark.js](benchmark.js) to benchmark your own system:

```sh
$ node benchmark.js
Connected: 1000, connection time: 867 ms, total broadcast time: 23 ms^C⏎
```

### Error *Too many open files*
You may be limited to a maximal number of connections (open file descriptors). Setting maximum number of open file descriptors to 2048:

```sh
ulimit -n 2048
```

Test maximum number of open connections with [drain.js](drain.js):

```sh
$ node drain.js
Connections dropped: 5957, accepting connections: false^C⏎
```

_Accepting connections_ indicates wheter resources for the server have been exhausted.