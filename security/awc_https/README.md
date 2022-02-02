The goal of this example is to show you how to use the actix-web client (awc)
for https related communication.  As of actix-web 2.0.0, one must be very
careful about setting up https communication.  **You could use the default 
awc api without configuring ssl but performance will be severely diminished**.

This example downloads a 1MB image from wikipedia.

To run:
> curl http://localhost:3000 -o image.jpg
