const http = require('http')

let drop_goal = 10_000;
let dropped = 0;

let query = {
    host: 'localhost',
    port: 8080,
    path: '/events'
}

setInterval(() => {
    if (dropped < drop_goal) {
        let request = http.get(query, response => {
            response.on('data', data => {
                if (data.includes("data: connected\n")) {
                    // drop connection after welcome message
                    dropped += 1;
                    request.abort()
                }
            })
        })
        .on('error', () => {})
    }
}, 1)

setInterval(() => {
    http.get('http://localhost:8080/', () => print_status(true))
        .setTimeout(100, () => print_status(false))
        .on('error', () => {})
}, 20)

function print_status(accepting_connections) {
    process.stdout.write("\r\x1b[K");
    process.stdout.write(`Connections dropped: ${dropped}, accepting connections: ${accepting_connections}`);
}
