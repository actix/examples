const http = require("http")

let drop_goal = 5_000
let dropped = 0

let query = {
  method: "POST",
  host: "127.0.0.1",
  port: 8080,
  path: "/events",
}

setInterval(() => {
  if (dropped < drop_goal) {
    let request = http
      .request(query, response => {
        response.on("data", data => {
          if (data.includes("data: connected\n")) {
            // drop connection after welcome message
            dropped += 1
            request.abort()
          }
        })
      })
      .on("error", () => {})
      .end()
  }
}, 0)

setInterval(() => {
  http
    .request({ ...query, path: "/" }, () => print_status(true))
    .setTimeout(100, () => print_status(false))
    .on("error", () => {})
}, 20)

function print_status(accepting_connections) {
  process.stdout.write("\r\x1b[K")
  process.stdout.write(
    `Connections dropped: ${dropped}, accepting connections: ${accepting_connections}`,
  )
}
