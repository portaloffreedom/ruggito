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
use router::{Router,ParamRouter,StringParameter};

fn main() {
    let mut router = Router::new();

    router.add_route("hello".to_string(), |_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello world!")))
    });
    router.add_route("error".to_string(), |_: &mut Request| {
        Ok(Response::with(status::BadRequest))
    });

    let mut music_router = Router::new();

    music_router.add_route("a".to_string(), |req: &mut Request| {
        let song_name = match req.extensions.get_mut::<StringParameter>() {
            Some(param) => param,
            None => return Ok(Response::with(status::NotFound)),
        };
        let mut song = match Song::new(song_name) {
            Err(why) => return Ok(Response::with((status::NotFound, format!("couldn't open {}: {}", song_name, Error::description(&why))))),
            Ok(song) => song,
        };

        let mut s = Vec::new();
        match song.read_to_end(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", song_name, Error::description(&why)),
            Ok(_) => {},
        };

        println!("read the whole song");

        let mut res = Response::with((status::Ok, s));
        res.headers.set(
            ContentType(Mime(TopLevel::Audio, SubLevel::Ext("mp3".to_string()), vec![]))
        );

        Ok(res)
    });

    let song_name_router = ParamRouter::new(music_router);

    router.add_route("song".to_string(), song_name_router);

    Iron::new(router).http("localhost:3000").unwrap();
}
