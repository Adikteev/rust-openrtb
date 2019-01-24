extern crate actix_web;

use actix_web::middleware::{Finished, Middleware, Response, Started};
use actix_web::{HttpRequest, HttpResponse, Result, http::{header, HttpTryFrom}};

pub struct DefaultHeaders;

impl DefaultHeaders {
    pub fn new() -> DefaultHeaders {
        DefaultHeaders { }
    }
}

impl<S> Middleware<S> for DefaultHeaders {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        println!("Hi from start. You requested: {}", req.path());
        Ok(Started::Done)
    }

    fn response(&self, _req: &HttpRequest<S>, mut resp: HttpResponse) -> Result<Response> {
        resp.headers_mut().insert(
            header::HeaderName::try_from("x-openrtb-version").unwrap(),
            header::HeaderValue::from_static("2.3"));
        Ok(Response::Done(resp))
    }

    fn finish(&self, _req: &HttpRequest<S>, _resp: &HttpResponse) -> Finished {
        println!("Hi from finish");
        Finished::Done
    }
}
