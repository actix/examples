# Customers API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Adding A Customer

```sh
curl -d '{"customer_name":"Peter Paul", "branch_name":"Central Business District"}' -H 'Content-Type: application/json' http://localhost:8080/customer

http POST :8080/customer customer_name="Peter Paul" branch_name="Central Business District"
```

You should expect a 204 No Content response.

## Listing Customers

```sh
curl http://localhost:8080/customer

http :8080/customer
```

The response should be a 200 OK with the following JSON body:

```json
{
  "customer_data": [
    {
      "customer_name": "peter paul",
      "branch_name": "central business district"
    }
  ]
}
```
