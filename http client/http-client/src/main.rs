extern crate futures;
extern crate hyper;
extern crate tokio_core;

use futures::{Future};
use hyper::{Client, Uri};
use tokio_core::reactor::Core;

fn main() {
    // Core is the Tokio event loop used for making a non-blocking request
    let mut core = Core::new().unwrap();

    let client = Client::new(&core.handle());

    let url : Uri = "http://127.0.0.1:7878".parse().unwrap();
    assert_eq!(url.query(), Some("ok"));

    let request = client.get(url)
        .map(|res| {
            assert_eq!(res.status(), hyper::Ok);
        });

    // request is a Future, futures are lazy, so must explicitly run
    core.run(request).unwrap();
}