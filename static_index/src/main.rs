extern crate actix;
extern crate actix_web;
extern crate env_logger;

use actix_web::{fs, App, server, middleware};

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    
    let sys = actix::System::new("static_index");


	server::new(
	    || App::new()
	        // enable logger
	        .middleware(middleware::Logger::default())
	        .handler(
                "/",
                fs::StaticFiles::new("./static/").index_file("index.html")
            )  
          )
          
        .bind("127.0.0.1:8080").expect( "Can not start server on given IP/Port" )
        .start();
	
	println!("Started http server: 127.0.0.1:8080");
	let _ = sys.run();

}

