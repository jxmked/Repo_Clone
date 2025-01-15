use super::base_trait::BaseTrait;

use crate::config_file_rw::JSONConfig;
use crate::constants::{CLONE_DEPTH, URL_PREFIX};
use crate::repo::{OutputFolder, RepoResult};

use std::process;

pub struct WithGit {
  repository: RepoResult,
  repo_url: String,
}

impl BaseTrait for WithGit {
  fn new(repository: RepoResult, _config: JSONConfig) -> Self {
    Self {
      repository,
      repo_url: String::new(),
    }
  }

  fn ready(&mut self) {
    let repo = &self.repository;

    self.repo_url.push_str(URL_PREFIX);
    self
      .repo_url
      .push_str(&format!("{}/{}", &repo.user, &repo.repository));
  }

  fn fetch_download(&mut self) -> Result<(), &str> {
    Ok(())
  }

  fn build_file(&mut self, mut _byte: &[u8]) -> Result<(), &str> {
    Ok(())
  }

  fn copy_to_output(&mut self, output_folder: &mut OutputFolder) -> Result<(), &str> {
    output_folder.create.owner();

    let branch = &self.repository.branch;
    let dst = &output_folder.create.repository_path;

    let mut proc = process::Command::new("git");
    proc
      .arg("clone")
      .arg("--depth")
      .arg(CLONE_DEPTH.to_string())
      .arg("--single-branch")
      .arg("--branch")
      .arg(&branch)
      .arg(&self.repo_url)
      .arg(dst.to_string_lossy().to_string());

    let spawn = proc.spawn();

    if spawn.is_err() {
      return Err("Git Program did not start!");
    }

    let respond = spawn.unwrap().wait_with_output();

    if respond.is_err() {
      return Err("Execution error");
    }

    println!("Cloned");

    Ok(())
  }
}
