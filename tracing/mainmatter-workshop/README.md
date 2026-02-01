# Telemetry Workshop Solution

## Overview

A solution to the capstone project at the end of [Mainmatter's telemetry workshop](https://github.com/mainmatter/rust-telemetry-workshop).

As stated in the exercise brief, this example will:

- Configure a `tracing` subscriber that exports data to both Honeycomb and stdout, in JSON format;
- Configure a suitable panic hook;
- Configure a `metric` recorder that exposes metric data at `/metrics`~~, using a different port than your API endpoints~~ (this example shows how to use the existing HTTP server);
- Add one or more middleware that:
  - Create a top-level INFO span for each incoming request;
  - Track the number of concurrent requests using a gauge;
  - Track request duration using a histogram;
  - Track the number of handled requests
- All metrics should include success/failure as a label.

## Usage

```console
$ cd tracing/mainmatter-workshop
$ cargo run
```
