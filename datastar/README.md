# Datastar poll

Run `cargo run -p example-datastar` from the workspace root and open <http://127.0.0.1:8080>. The browser needs internet access to load Datastar from the CDN.

Open the page in two tabs. Vote for a drink in one tab. Actix Web records each click and uses the Datastar Rust SDK to send the updated HTML in a patch-elements server-sent event. Datastar replaces the element with the matching ID. The other tab fetches the results every five seconds.

All workers share the poll in memory. Restart the server to reset the counts.
