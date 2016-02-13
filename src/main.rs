#[macro_use]
extern crate nickel;

#[macro_use]
extern crate log;

extern crate log4rs;
extern crate hyper;

use nickel::*;
use hyper::uri::RequestUri;
use std::default::Default;
use std::env;

struct Logger;

impl<D> Middleware<D> for Logger {
    fn invoke<'mw, 'conn>(&self, req: &mut Request<'mw, 'conn, D>, res: Response<'mw, D>)
    -> MiddlewareResult<'mw, D> {
        if let RequestUri::AbsolutePath(ref path) = req.origin.uri {
            info!("{0} {1}", req.origin.method, path);
        }

        res.next_middleware()
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("You must provide a folder name where the static files for the app root is located.");
    }
    let ref app_root = args[1];

    log4rs::init_file("config/log.toml", Default::default()).unwrap();

    let mut server = Nickel::new();
    let mut router = Nickel::router();

    server.utilize(Logger);
    router.get("/api/sessions", middleware! {
        "foo"
    });
    server.utilize(router);

    server.utilize(StaticFilesHandler::new(app_root));

    let port = 6767;
    let host = format!("127.0.0.1:{}", port);
    info!("Listening on port {}, serving app from {}", port, app_root);
    server.listen(&host[..]);
}
