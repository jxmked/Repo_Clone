use std::fs;
use std::path::PathBuf;

pub struct DirectoryCreator {
  owner_path: PathBuf,
  repository_path: PathBuf,
}

impl DirectoryCreator {
  pub fn new() -> Self {
    Self {
      owner_path: PathBuf::new(),
      repository_path: PathBuf::new(),
    }
  }

  fn create_directory(&self, path: PathBuf) {
    fs::create_dir_all(path);
  }

  pub fn set_owner_path(&mut self, path: PathBuf) {
    self.owner_path = path.clone();
  }

  pub fn set_repository_path(&mut self, path: PathBuf) {
    self.repository_path = path.clone();
  }

  pub fn owner(&self) {
    let path = self.owner_path.clone();
    self.create_directory(path);
  }

  pub fn repository(&self) {
    let path = self.repository_path.clone();
    self.create_directory(path);
  }
}
