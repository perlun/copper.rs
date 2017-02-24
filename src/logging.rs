use nickel::{Middleware, MiddlewareResult, Request, Response};

pub struct Logger;

impl<D> Middleware<D> for Logger {
    fn invoke<'mw, 'conn>(&self, req: &mut Request<'mw, 'conn, D>, res: Response<'mw, D>)
    -> MiddlewareResult<'mw, D> {
        info!("{0} {1} {2}", res.status().to_u16(), req.origin.method, req.origin.uri);
        res.all_on_start();

        res.next_middleware()
    }
}
