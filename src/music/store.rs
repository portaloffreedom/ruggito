use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;

pub struct Song {
    file: File,
}

impl Read for Song {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.file.read(buf)
    }
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        self.file.read_to_end(buf)
    }
    fn read_to_string(&mut self, buf: &mut String) -> Result<usize> {
        self.file.read_to_string(buf)
    }
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        self.file.read_exact(buf)
    }
}

impl Song {
    pub fn new(file_path: &String) -> Result<Song> {
        let path = Path::new(&file_path);

        let file = match File::open(&path) {
            Err(why) => return Err(why),
            Ok(file) => file,
        };

        Ok(Song {
            file: file
        })
    }
}

pub struct MusicStore {
    map: HashMap<String, String>
}

impl MusicStore {
    pub fn new() -> Self {
        MusicStore { map: HashMap::new() }
    }

    pub fn add_song(&mut self, id: String, file_path: String) -> Result<()> {
        {
            let path = Path::new(&file_path);
            let display = path.display();

            let mut file = match File::open(&path) {
                Err(why) => {
                    println!("couldn't open {}: {}", display, Error::description(&why));
                    return Err(why);
                },
                Ok(file) => file,
            };

            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => {
                    println!("couldn't read {}: {}", display, Error::description(&why));
                    return Err(why);
                },
                Ok(_) => {},
            };
        }

        self.map.insert(id, file_path);
        Ok(())
    }
}
