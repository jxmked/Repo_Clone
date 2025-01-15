use super::base_trait::BaseTrait;

use crate::config_file_rw::JSONConfig;
use crate::repo::{OutputFolder, RepoResult};

use std::process;

pub struct PullGit {
  repository: RepoResult,
}

impl BaseTrait for PullGit {
  fn new(repository: RepoResult, _config: JSONConfig) -> Self {
    Self { repository }
  }

  fn ready(&mut self) {}

  fn fetch_download(&mut self) -> Result<(), &str> {
    Ok(())
  }

  fn build_file(&mut self, mut _byte: &[u8]) -> Result<(), &str> {
    Ok(())
  }

  fn copy_to_output(&mut self, output_folder: &mut OutputFolder) -> Result<(), &str> {
    let branch = &self.repository.branch;
    let dst = &output_folder.create.repository_path;

    let mut proc = process::Command::new("git");
    proc
      .arg("-C")
      .arg(dst.to_string_lossy().to_string())
      .arg("pull")
      .arg("origin")
      .arg(&branch);

    let spawn = proc.spawn();

    if spawn.is_err() {
      return Err("Git Program did not start!");
    }

    let respond = spawn.unwrap().wait_with_output();

    if respond.is_err() {
      return Err("Execution error");
    }

    println!("\nComplete.");

    Ok(())
  }
}
