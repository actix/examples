This is a contrived example intended to illustrate a few important actix-web features.

*Imagine* that you have a process that involves 3 steps.  The steps here
are dumb in that they do nothing other than call an
httpbin endpoint that returns the json that was posted to it.  The intent here
is to illustrate how to chain these steps together as futures and return
a final result in a response.

Actix-web features illustrated here include:

    1. handling json input param
    2. validating user-submitted parameters using the 'validator' crate
    2. actix-web client features:
          - POSTing json body
    3. chaining futures into a single response used by an asynch endpoint


Example query from the command line using httpie:
	```echo '{"id":"1", "name": "JohnDoe"}' | http 127.0.0.1:8080/something```
