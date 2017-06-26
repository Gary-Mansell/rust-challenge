extern crate hyper;
extern crate futures;

use hyper::header::{ContentLength, From};
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};

static NOT_FOUND: &'static str = "NOT FOUND";

struct RustService;

impl Service for RustService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, request: Request) -> Self::Future {
        let mut response = Response::new();
        match (request.method(), request.path()) {
            (&Method::Get, "/") => {
                let text: &'static str = "This is a basic Rust web service.";
                response.headers_mut().set(From("Moi!!".parse().unwrap()));
                response.headers_mut().set(ContentLength(text.len() as u64));
                response.set_body(text);
            },
            (&Method::Post, "/command") => {
                response.set_body(request.body());
            },
            _ => {
                response.set_body(NOT_FOUND);
                response.set_status(StatusCode::NotFound);
            }
        };
        futures::future::ok(response)
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(RustService)).unwrap();
    server.run().unwrap();
}