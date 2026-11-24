# Datastar poll

Run `cargo run -p example-datastar` from the workspace root and open <http://127.0.0.1:8080>. The browser needs internet access to load Datastar from the CDN.

Open the page in two tabs. Each tab opens an SSE connection and receives the current tally. Vote for a drink in either tab. Actix Web records the vote and sends the updated results to both tabs through their open connections. The Datastar Rust SDK formats each update as a patch-elements event.

All workers share the poll in memory. Restart the server to reset the counts.
