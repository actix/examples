#![feature(proc_macro, proc_macro_non_items, plugin, macro_at_most_once_rep)]

extern crate actix;
extern crate actix_web;
extern crate maud;

use actix_web::{http, server, App, Path, Responder};
use maud::{html, Markup, PreEscaped};

fn body_tpl(name: &str, message: &str) -> Markup {
    html! {
        h3#userName {
            "Hello " (name) ", " (message)
        }
    }
}

fn index(p: Path<(String, String)>) -> impl Responder {
    html! {
        (maud::DOCTYPE)
        html lang="ru" xmlns="http://www.w3.org/1999/xhtml" {
            head {
                title "Maud template"

                (PreEscaped(r#"
                    <style>
                        #userName {
                            color: red;
                        }
                    </style>
                "#))
            }

            body {
                (body_tpl(&p.0, &p.1))
            }
        }
    }
}

fn main() {
    let sys = actix::System::new("template-maud");

    server::new(move || App::new().route("/{name}/{message}", http::Method::GET, index))
        .bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
