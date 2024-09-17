use std::fs::File;
use std::io::{Read, Write};
// use std::ops::Not;
use crate::constants;
use std::path::Path;

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=1434052276de34362138cba939f6967a
// pub fn not<T: Not>(x: T) -> <T as Not>::Output {
//   Not::not(x)
// }

pub fn exit(exit_code: i32) {
  std::process::exit(exit_code);
}

pub fn read_file(file_path: &str) -> String {
  create_if_not_exists(file_path);

  let mut file = File::open(file_path).unwrap();
  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .expect("Failed to read file");

  return contents;
}

pub fn write_file(file_path: &str, contents: &str) {
  create_if_not_exists(file_path);

  let mut file = File::create(file_path).unwrap();
  file
    .write_all(contents.as_bytes())
    .expect("Failed to write file");
}

pub fn create_if_not_exists(file_path: &str) {
  if !file_exists(file_path) {
    File::create(file_path).expect("Failed to create file");
  }
}

pub fn file_exists(file_path: &str) -> bool {
  Path::new(file_path).is_file()
}
pub fn random(len: usize) -> String {
  random_string::generate(len, constants::RANDOM_CHARSET)
}
