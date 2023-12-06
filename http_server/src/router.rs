use crate::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::http_request::{Method, Request, Resource};
use std::io::Write;

pub struct Router;

impl Router {
    pub fn route(req: Request, stream: &mut impl Write) -> () {
        match req.method {
            Method::Get => match &req.resource {
                Resource::Path(path) => {
                    let route: Vec<&str> = path.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
