use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::str::FromStr;

pub use vec2::*;

mod vec2;

pub fn file_string(path: &str) -> String {
    let mut buf = String::new();
    File::open(path).unwrap().read_to_string(&mut buf).unwrap();
    buf
}

pub fn file_lines(path: &str) -> impl Iterator<Item = String> {
    let reader = BufReader::new(File::open(path).unwrap());
    return reader.lines().map(|l| l.unwrap());
}

pub fn file_lines_as<T>(path: &str) -> impl Iterator<Item = T> 
    where T: FromStr, <T as FromStr>::Err: Debug
{
    file_lines(path).map(|l| l.parse().expect("failed to parse line from file"))
}
