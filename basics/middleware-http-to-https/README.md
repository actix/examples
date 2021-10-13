## Middleware eg - redirect any http connection to use https connection

This example is the next step after implementing this example : [Setup TLS via rustls](https://github.com/actix/examples/tree/master/security/rustls).

You might have already implemented TLS(using one of the ways mentioned in the example of security section), and have setup your server to listen to port 443(for https).

Now, the only problem left to solve is, to listen to **http** connections as well and redirect them to use **https**

## Usage

**Note :** You will be required to use sudo while running the binary to access port 80 and 443

