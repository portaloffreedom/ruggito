extern crate iron;
extern crate hyper;

mod music;
mod router;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;

use hyper::mime::{Mime, TopLevel, SubLevel};

use music::store::Song;
use router::Router;

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

    router.add_route("song".to_string(), |_: &mut Request| {
        let song_name = "song.mp3".to_string();
        let mut song = match Song::new(&song_name) {
            Err(why) => return Ok(Response::with((status::NotFound, format!("couldn't open {}: {}", &song_name, Error::description(&why))))),
            Ok(song) => song,
        };

        let mut s = Vec::new();
        match song.read_to_end(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", &song_name, Error::description(&why)),
            Ok(_) => {},
        };

        println!("read the whole song");

        let mut res = Response::with((status::Ok, s));
        res.headers.set(
            ContentType(Mime(TopLevel::Audio, SubLevel::Ext("mp3".to_string()), vec![]))
        );

        Ok(res)
    });

    Iron::new(router).http("localhost:3000").unwrap();
}
