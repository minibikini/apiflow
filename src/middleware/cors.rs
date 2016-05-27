use iron::prelude::*;
use iron::{AfterMiddleware};
use iron::headers::AccessControlAllowOrigin;

pub struct Cors;

impl AfterMiddleware for Cors {
    fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(AccessControlAllowOrigin::Any);
        Ok(res)
    }
}