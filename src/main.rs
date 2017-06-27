extern crate hyper;
extern crate futures;
extern crate tokio_io;

use hyper::header::{ContentLength, From};
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};
use futures::Stream;
use futures::future::{Future, Ok};

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
                response
                    .headers_mut()
                    .set(From("Moi!!".parse().unwrap()));
                response
                    .headers_mut()
                    .set(ContentLength(text.len() as u64));
                response.set_body(text);
            }
            (&Method::Post, "/command") => {
                request
                    .body()
                    .collect()
                    .map(|chunks| {
                             let command = chunks
                                 .iter()
                                 .flat_map(|chunk| chunk.to_vec().clone())
                                 .collect::<Vec<u8>>();
                             println!("Recieved command = {}", String::from_utf8(command).unwrap());
                         });
            }
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
