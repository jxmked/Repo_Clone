// These function will handle the output folder,
// Verify if its already exists or the folder is not empty

mod directory_creator;

use std::{
  fs,
  path::{Path, PathBuf},
};

use directory_creator::DirectoryCreator;

use crate::config_file_rw::JSONConfig;

use super::RepoResult;

pub struct Output_Folder {
  pub folder_path: PathBuf,
  pub is_owner_exists: bool,
  pub is_repository_exists: bool,

  pub directory_creator: DirectoryCreator,
}

impl Output_Folder {
  pub fn new() -> Self {
    Self {
      folder_path: PathBuf::new(),
      is_owner_exists: false,
      is_repository_exists: false,
      directory_creator: DirectoryCreator::new()
    }
  }

  pub fn create() {}
}

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
