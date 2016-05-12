#[macro_use]
extern crate iron;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use iron::prelude::*;
use iron::Handler;
use iron::status;

struct Router {
    // Routes here are simply matched with the url Path
    routes: HashMap<String, Box<Handler>>
}

impl Router {
    fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path.join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound))
        }
    }
}

fn main() {
    let mut router = Router::new();

    router.add_route("hello".to_string(), |_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello world!")))
    });
    router.add_route("hello/again".to_string(), |_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello again!")))
    });
    router.add_route("file".to_string(), |_: &mut Request| {
        let path = Path::new("test.txt");
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => return Ok(Response::with((status::InternalServerError,
                format!("couldn't open {}: {}", display, Error::description(&why))))),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
            Ok(_) => print!("{} contains:\n{}", display, s),
        };

        Ok(Response::with((status::Ok, s)))
    });
    router.add_route("error".to_string(), |_: &mut Request| {
        Ok(Response::with(status::BadRequest))
    });
    router.add_route("error/file".to_string(), |_: &mut Request| {
        let path = Path::new("non_existing.txt");
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => return Ok(Response::with((status::InternalServerError,
                format!("couldn't open {}: {}", display, Error::description(&why))))),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
            Ok(_) => print!("{} contains:\n{}", display, s),
        };

        Ok(Response::with((status::Ok, s)))
    });

    Iron::new(router).http("localhost:3000").unwrap();
}
