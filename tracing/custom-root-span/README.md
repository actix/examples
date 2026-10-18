# Custom Root Span

## Usage

You can launch this example with

```sh
cd tracing/custom-root-span
cargo run
```

An `actix-web` application will be listening on port `8080`.  
You can fire requests to it with:

```sh
curl -v http://localhost:8080/hello
```

```text
Hello world!
```

or

```sh
curl -v http://localhost:8080/hello/my-name
```

```text
Hello my-name!
```

## Traces

Spans will be also printed to the console in JSON format, as structured log records.

You can look at the exported spans in your browser by visiting [http://localhost:16686](http://localhost:16686) if you launch a Jaeger instance:

```sh
docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p4317:4317 jaegertracing/all-in-one:latest
```
