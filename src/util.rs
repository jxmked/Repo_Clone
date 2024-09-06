use std::ops::Not;
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write, BufReader};


// https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=1434052276de34362138cba939f6967a
pub fn not<T: Not>(x: T) -> <T as Not>::Output {
    Not::not(x)
}


pub fn read(file_path: &str) -> String {

  __create_if_not_exists(&file_path);

  let mut file = File::open(file_path).unwrap();
  let mut contents = String::new();

  file.read_to_string(&mut contents);

  return contents;
}

pub fn write(file_path: &str, contents: &str) {

  __create_if_not_exists(&file_path);

  let mut file = File::open(&file_path).unwrap();
  file.write_all(contents.as_bytes());

}

fn __create_if_not_exists(file_path: &str) {
  if not(Path::new(&file_path).is_file()) {
    File::create(&file_path);
  }
}