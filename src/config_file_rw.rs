use crate::constants;
use crate::util;

use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use serde_json::Result;
use std::env;

#[derive(Serialize, Deserialize)]
pub struct JSONConfig {
  pub token: String,
  pub output_path: String,
}

lazy_static! {
  static ref ABS_FILE_PATH: String = __get_file_path__(constants::CONF_FILENAME);
}

const __DEFAULT_CONFIG__: &str = r#"{"token":"","output_path":""}"#;

fn __get_file_path__(file_path: &str) -> String {
  let mut dir: std::path::PathBuf = env::current_exe().unwrap();
  dir.pop();
  dir.push(file_path);
  return dir.to_str().unwrap().to_string();
}

pub fn read_json_file() -> Result<JSONConfig> {
  if !util::file_exists(&ABS_FILE_PATH) {
    util::write_file(&ABS_FILE_PATH, __DEFAULT_CONFIG__);
  }

  let contents: String = util::read_file(&ABS_FILE_PATH);

  return serde_json::from_str(&contents);

  // serde_json::from_str(&contents).map_err(|err| {
  //     println!("Error parsing config file.");
  //     println!("Visit https://github.com/jxmked/Repo_Clone/issues");
  //     err
  // })
}

pub fn write_json_file(json_value: JSONConfig) -> Result<()> {
  let new_conf: String = serde_json::to_string_pretty(&json_value).unwrap();

  util::write_file(&ABS_FILE_PATH, &new_conf);
  Ok(())
}
