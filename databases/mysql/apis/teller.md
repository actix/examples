# Tellers API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Adding A Teller

```sh
curl -d '{"teller_name":"John Doe", "branch_name":"Central Business District"}' -H 'Content-Type: application/json' http://localhost:8080/teller

http POST :8080/teller teller_name="John Doe" branch_name="Central Business District"
```

You should expect a 204 No Content response.

## Listing Tellers

```sh
curl http://localhost:8080/teller

http :8080/teller
```

The response should be a 200 OK with the following JSON body:

```json
{
  "teller_data": [
    {
      "teller_name": "john doe",
      "branch_name": "central business district"
    }
  ]
}
```
