extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate futures;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate json;

use actix_web::{
    http, middleware as mw, server, App, HttpMessage,
    HttpRequest, HttpResponse, Json,
};


pub mod model;
pub mod middleware;

/// This handler uses json extractor
fn extract_item(item: Json<String>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

fn index(req: &HttpRequest) -> String {
    format!("Hello {}", "Anonymous")
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("json-example");

    server::new(|| {
        App::new()
            // enable logger
            .middleware(mw::Logger::default())
            .middleware(middleware::headers::DefaultHeaders::new())
            .resource("/extractor", |r| {
                r.method(http::Method::POST)
                    .with_config(extract_item, |(cfg,)| {
                        cfg.limit(4096); // <- limit size of the payload
                    })
            })
            .resource("/", |r| r.method(http::Method::POST).f(index))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .shutdown_timeout(1)
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
