This project illustrates how to send multiple cache requests to redis in bulk, asynchronously.
This asyncio approach resembles traditional redis pipelining.  Details about how this
is so can be read at https://github.com/benashford/redis-async-rs/issues/19#issuecomment-412208018



To test the demo, POST a json object containing three strings to the /stuff endpoint:
	{"one": "first entry",
	 "two": "second entry",
	 "three": "third entry" }


These three entries will cache to redis, keyed accordingly.

to delete these, simply issue a DELETE http request to /stuff endpoint
