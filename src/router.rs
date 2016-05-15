use std::collections::HashMap;

use iron::prelude::*;
use iron::Handler;
use iron::status;
use iron::typemap;

pub struct StringParameter;

impl typemap::Key for StringParameter { type Value = String; }

pub struct Router {
    // Routes here are simply matched with the url Path
    routes: HashMap<String, Box<Handler>>
}

impl Router {
    pub fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    pub fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path[0]) {
            Some(handler) => {
                req.url.path.remove(0);
                handler.handle(req)
            },
            None => Ok(Response::with(status::NotFound))
        }
    }
}

pub struct ParamRouter {
    next_handler: Box<Handler>
}

impl ParamRouter {
    pub fn new<H>(handler: H)-> Self where H: Handler {
        ParamRouter { next_handler: Box::new(handler) }
    }

    pub fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.next_handler = Box::new(handler);
    }
}

impl Handler for ParamRouter {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let param = req.url.path[0].clone();
        req.extensions.insert::<StringParameter>(param);
        req.url.path.remove(0);
        self.next_handler.handle(req)
    }
}
