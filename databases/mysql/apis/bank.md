# Banks API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Adding A Bank

```sh
curl -d '{"bank_name":"Bank ABC","country":"Kenya"}' -H 'Content-Type: application/json' http://localhost:8080/bank

http POST :8080/bank bank_name="Bank ABC" country="Kenya"
```

You should expect a 204 No Content response.

## Listing Banks

```sh
curl http://localhost:8080/bank

http :8080/bank
```

The response should be a 200 OK with the following JSON body:

```json
{
  "bank_data": [
    {
      "bank_name": "bank abc",
      "country": "kenya"
    }
  ]
}
```
