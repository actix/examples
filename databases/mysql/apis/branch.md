# Branches API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Adding A Branch

```sh
curl -d '{"branch_name":"HQ branch", "location":"Central Business District"}' -H 'Content-Type: application/json' http://localhost:8080/branch

http POST :8080/branch branch_name="HQ branch" branch_name="Central Business District"
```

You should expect a 204 No Content response.

## Listing Branches

```sh
curl http://localhost:8080/branch

http :8080/branch
```

The response should be a 200 OK with the following JSON body:

```json
{
  "branch_data": [
    {
      "branch_name": "hq branch",
      "location": "central business district"
    }
  ]
}
```
