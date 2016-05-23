extern crate iron;
extern crate hyper;

mod music;
mod router;

use std::error::Error;
use std::io::prelude::*;

use iron::prelude::*;
use iron::status;
use iron::headers::{ContentType,AcceptRanges,RangeUnit,Range};
use iron::headers::Range::{Bytes,Unregistered};
use iron::headers::ByteRangeSpec::{FromTo,AllFrom,Last};
use iron::headers::ContentRangeSpec::Bytes as ContentRangeBytes;
use iron::headers::ContentRange;

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

        let mut byte_data = Vec::new();
        match song.read_to_end(&mut byte_data) {
            Err(why) => panic!("couldn't read {}: {}", song_name, Error::description(&why)),
            Ok(_) => {},
        };

        println!("read the whole song");

        let mut res = match req.headers.get::<Range>() {
            Some(range) => {
                println!("range: {:?}", range);
                let mut data_start = None;
                let mut data_end = None;
                let byte_data_original_size = byte_data.len() as u64;
                match range {
                    &Bytes(ref byte_range) => {
                        let mut sliced_result: Vec<u8> = Vec::new();
                        {
                            // make byte_data read only
                            let byte_data = &byte_data;

                            // range for all the sequences
                            for r in byte_range {
                                println!("r: {:?}", r);
                                let slice = match r {
                                    &FromTo(start, end) => {
                                        data_start = Some(start);
                                        data_end = Some(end);
                                        &byte_data[start as usize .. end as usize]
                                    },
                                    &AllFrom(start) => {
                                        data_start = Some(start);
                                        data_end = Some(byte_data_original_size);
                                        &byte_data[start as usize..]
                                    },
                                    &Last(end) => {
                                        data_start = Some(0);
                                        data_end = Some(end);
                                        &byte_data[..end as usize]
                                    },
                                };
                                sliced_result.extend_from_slice(slice);
                            }
                        }
                        byte_data = sliced_result;
                    },
                    _ => {},
                };

                // Prepare response
                let mut status = status::Ok;
                //let mut res = Response::with((status::PartialContent, byte_data));
                let range = match data_start {
                    Some(start) => match data_end {
                        Some(end) => {
                            Some((start,end))
                        },
                        None => None,
                    },
                    None => None,
                };
                if byte_data.len() != byte_data_original_size as usize {
                    status = status::PartialContent;
                }

                let mut res = Response::with((status, byte_data));
                res.headers.set(ContentRange(ContentRangeBytes {
                    range: range,
                    instance_length: Some(byte_data_original_size),
                }));

                res
            },
            None => Response::with((status::Ok, byte_data)),
        };

        // Content-Type: audio/mp3
        res.headers.set(
            ContentType(Mime(TopLevel::Audio, SubLevel::Ext("mp3".to_string()), vec![]))
        );
        // Accept-Ranges: bytes
        res.headers.set(AcceptRanges(vec![RangeUnit::Bytes]));

        Ok(res)
    });

    let song_name_router = ParamRouter::new(music_router);

    router.add_route("song".to_string(), song_name_router);

    Iron::new(router).http("localhost:3000").unwrap();
}
