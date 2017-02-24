#[macro_use]
extern crate nickel;

#[macro_use]
extern crate log;

extern crate log4rs;
extern crate rustc_serialize;

use nickel::{HttpRouter, Nickel, StaticFilesHandler};
use rustc_serialize::json;
use std::default::Default;
use std::env;

mod logging;

// Automatically generate `RustcDecodable` and `RustcEncodable` trait
// implementations
#[derive(RustcDecodable, RustcEncodable)]
pub struct TestStruct  {
    data_int: u8,
    data_str: String,
    data_vector: Vec<u8>,
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        panic!("You must provide a folder name where the static files for the app root is located.");
    }
    let ref app_root = args[1];

    log4rs::init_file("config/log.yaml", Default::default()).unwrap();

    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/api/sessions", middleware! {
        let object = TestStruct {
            data_int: 1,
            data_str: "homura".to_string(),
            data_vector: vec![2,3,4,5],
        };

        json::encode(&object).unwrap()
    });
    server.utilize(router);

    server.utilize(StaticFilesHandler::new(app_root));
    server.utilize(logging::Logger);

    let port = 6767;
    let host = format!("127.0.0.1:{}", port);
    info!("Listening on port {}, serving app from {}", port, app_root);
    server.listen(&host[..]);
}
