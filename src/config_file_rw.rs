use crate::constants;
use crate::util;

use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct JSONConfig {
  pub token: String,
  pub output_path: String,
  pub exec_root: String,
}

lazy_static! {
  static ref ABS_FILE_PATH: String = __get_file_path__(constants::CONF_FILENAME);
}

const __DEFAULT_CONFIG__: &str = r#"{"token":"","output_path":"", "exec_root":""}"#;

fn __get_file_path__(file_path: &str) -> String {
  let mut dir: std::path::PathBuf = env::current_exe().unwrap();
  dir.pop();
  dir.push(file_path);
  return dir.to_str().unwrap().to_string();
}

pub fn read_json_file() -> JSONConfig {
  if !util::file_exists(&ABS_FILE_PATH) {
    util::write_file(&ABS_FILE_PATH, __DEFAULT_CONFIG__);
  }

  let contents: String = util::read_file(&ABS_FILE_PATH);
  let mut json_parsed: JSONConfig = serde_json::from_str(&contents).unwrap();

  let mut exec_root = std::env::current_exe().unwrap();
  exec_root.pop();

  json_parsed.exec_root = exec_root.to_string_lossy().to_string();

  return json_parsed;
}

pub fn write_json_file(json_value: JSONConfig) {
  let new_conf: String = serde_json::to_string_pretty(&json_value).unwrap();

  util::write_file(&ABS_FILE_PATH, &new_conf);
}

pub fn is_output_dir_set() -> bool {
  let rd: JSONConfig = read_json_file();

  !rd.output_path.is_empty()
}

pub fn is_token_set() -> bool {
  let rd: JSONConfig = read_json_file();

  !rd.token.is_empty()
}
