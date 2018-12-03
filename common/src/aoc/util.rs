use std::env;
use std::io::{Error, ErrorKind};
use std::io::Result;

pub fn get_filename() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        Ok(args[1].clone())
    } else {
        Err(Error::from(ErrorKind::InvalidInput))
    }
}