// These function will handle the output folder,
// Verify if its already exists or has a valid folder name

use std::path::Path;


use crate::config_file_rw::JSONConfig;

use super::RepoResult;

let mut is_exists = false;

fn is_output_folder_exists(folder: &str, conf: JSONConfig) -> bool {


  Path::new(folder).exists()
}

pub fn output_folder(ret: RepoResult, conf:JSONConfig) {

  let abs_out_folder = Path::new(&conf.output_path).to_path_buf();
  abs_out_folder.push(ret.output_folder);


  if is_output_folder_exists(folder, conf) {

  }
}
