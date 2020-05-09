# yarte

Minimal example of using template [yarte](https://github.com/botika/yarte) that displays a form.

[Template benchmarks in stable](https://github.com/botika/template-bench-rs)

```bash
cargo test

cargo run
```
> open `localhost:8080`

## Generated code
```rust
impl ::std::fmt::Display for IndexTemplate {
    fn fmt(&self, _fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        _fmt.write_str(
            "<!DOCTYPE html><html><head><meta charset=\"utf-8\" /><title>Actix \
             web</title></head><body>",
        )?;
        if let Some(name__0) = self.query.get("name") {
            let lastname__1 = self.query.get("lastname").ok_or(yarte::Error)?;
            _fmt.write_str("<h1>Hi, ")?;
            ::yarte::Render::render(&(name__0), _fmt)?;
            _fmt.write_str(" ")?;
            ::yarte::Render::render(&(lastname__1), _fmt)?;
            _fmt.write_str("!</h1><p id=\"hi\" class=\"welcome\">Welcome</p>")?;
        } else {
            _fmt.write_str(
                "<h1 id=\"welcome\" class=\"welcome\">Welcome!</h1><div><h3>What is your \
                 name?</h3><form>Name: <input type=\"text\" name=\"name\" /><br/>Last name: \
                 <input type=\"text\" name=\"lastname\" /><br/><p><input \
                 type=\"submit\"></p></form></div>",
            )?;
        }
        _fmt.write_str("</body></html>")?;
        Ok(())
    }
}

impl ::yarte::Template for IndexTemplate {
    fn mime() -> &'static str {
        "text/html; charset=utf-8"
    }
    
    fn size_hint() -> usize {
        838usize
    }
}

impl ::yarte::aw::Responder for IndexTemplate {
    type Error = ::yarte::aw::Error;
    type Future = ::yarte::aw::Ready<::std::result::Result<::yarte::aw::HttpResponse, Self::Error>>;
    
    #[inline]
    fn respond_to(self, _req: &::yarte::aw::HttpRequest) -> Self::Future {
        match self.call() {
            Ok(body) => ::yarte::aw::ok(
                ::yarte::aw::HttpResponse::Ok()
                    .content_type(Self::mime())
                    .body(body),
            ),
            Err(_) => ::yarte::aw::err(::yarte::aw::ErrorInternalServerError("Some error message")),
        }
    }
}
```
