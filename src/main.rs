#![deny(warnings)]
extern crate hyper;

use hyper::{Body, Response, Server};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};

static PHRASE: &'static [u8] = b"Hello Rust!";

fn main() {
    let addr = ([0, 0, 0, 0], 3000).into();

    let hello_world = || {
        service_fn_ok(|_| {
            Response::new(Body::from(PHRASE))
        })
    };

    let server = Server::bind(&addr)
        .serve(hello_world)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}
