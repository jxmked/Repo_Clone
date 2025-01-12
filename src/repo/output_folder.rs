// These function will handle the output folder,
// Verify if its already exists or the folder is not empty

use std::{
  fs,
  path::{Path, PathBuf},
};

use crate::config_file_rw::JSONConfig;

use super::RepoResult;

fn is_directory_empty(path: &PathBuf) -> bool {
  let entries = fs::read_dir(path);
  let first_entry = entries.unwrap().next();
  first_entry.is_none()
}

fn is_directory_exists(path: &PathBuf) -> bool {
  path.exists()
}

pub fn output_folder(ret: &RepoResult, conf: &JSONConfig) -> Result<PathBuf, String> {
  let mut abs_out_folder = Path::new(&conf.output_path).to_path_buf();

  abs_out_folder.push(ret.user.clone());
  abs_out_folder.push(ret.output_folder.clone());

  if is_directory_exists(&abs_out_folder) {
    if !is_directory_empty(&abs_out_folder) {
      return Err(format!(
        "Directory '{}' is not empty.",
        abs_out_folder.to_string_lossy().to_string()
      ));
    }
  }

  // We don't need to return the final output folder where the
  // cloned files are about to place.
  // We may need to make sure that the .../username/<repository>... are existing...
  // outside of this function.

  // Means, we need to push the final directory into path before someshit inside to use it.

  abs_out_folder.pop();

  Ok(abs_out_folder)
}
