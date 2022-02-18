This example illustrates how to use nested resource registration through application-level configuration.  
The endpoints do nothing.

## Usage

```sh
cd basics/nested-routing
cargo run
# Started http server: 127.0.0.1:8080
```

### Available Routes

- [POST /products](http://localhost:8080/products)
- [GET /products](http://localhost:8080/products)
- [GET /products/:product_id](http://localhost:8080/products/:product_id)
- [DELETE /products/:product_id](http://localhost:8080/products/:product_id)
- [GET /products/:product_id/parts](http://localhost:8080/products/:product_id/parts)
- [POST /products/:product_id/parts](http://localhost:8080/products/:product_id/parts)
- [GET /products/:product_id/parts/:part_id](http://localhost:8080/products/:product_id/parts/:part_id)
