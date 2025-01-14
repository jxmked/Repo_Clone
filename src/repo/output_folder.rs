// These function will handle the output folder,
// Verify if its already exists or the folder is not empty

mod directory_creator;

use std::fs;
use std::path::{Path, PathBuf};

use directory_creator::DirectoryCreator;

use super::RepoResult;

pub struct OutputFolder {
  pub is_owner_exists: bool,
  pub is_repository_exists: bool,
  pub create: DirectoryCreator,
  output_folder: String,
  repo_result: RepoResult,
}

impl OutputFolder {
  pub fn new(repo_result: RepoResult, output_folder: &String) -> Self {
    Self {
      is_owner_exists: false,
      is_repository_exists: false,
      create: DirectoryCreator::new(),
      output_folder: output_folder.clone(),
      repo_result,
    }
  }

  pub fn set_directory_creator(&mut self) {
    let mut base_path = Path::new(&self.output_folder).to_path_buf();

    // Push the user/owner of the repository first
    base_path.push(&self.repo_result.user);

    // Set Owner Path To Create soon if not exists, also flag it if already exists
    self.create.set_owner_path(base_path.clone());
    self.is_owner_exists = base_path.exists();

    // Push the repository output folder after the user/owner
    base_path.push(&self.repo_result.output_folder);

    self.create.set_repository_path(base_path.clone());
    self.is_repository_exists = base_path.exists();
  }

  pub fn validate_repository_folder(&self) -> Result<(), String> {
    // If owner's or repository folder doesn't exists
    // Its can be a valid output folder.
    if !self.is_owner_exists {
      return Ok(());
    }

    if !self.is_repository_exists {
      return Ok(());
    }

    // Check if the directory is empty. If not, return error
    if self.directory_not_empty(&self.create.repository_path) {
      return Ok(());
    }

    Err(format!(
      "Directory '{}' does exists and not empty.",
      &self.create.repository_path.display()
    ))
  }

  fn directory_not_empty(&self, directory: &PathBuf) -> bool {
    let entries = fs::read_dir(directory);
    let first_entry = entries.unwrap().next();
    first_entry.is_none()
  }
}
