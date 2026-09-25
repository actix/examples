# OpenTelemetry integration

## Prerequisites

To execute this example you need a running Jaeger instance.  
You can launch one using Docker:

```bash
docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p4317:4317 jaegertracing/all-in-one:latest
```

## Running

You can launch this example with

```bash
cd tracing/opentelemetry
cargo run
```

An `actix-web` application will be listening on port `8080`.  
You can fire requests to it with:

```bash
curl -v http://localhost:8080/hello
```

```text
Hello world!
```

## Traces

You can look at the exported traces in your browser by visiting [http://localhost:16686](http://localhost:16686).  
Spans will be also printed to the console in JSON format, as structured log records.
