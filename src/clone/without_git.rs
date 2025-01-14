use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use futures::executor::block_on;
use reqwest;

use super::base_trait::BaseTrait;

use crate::config_file_rw::JSONConfig;
use crate::constants::GZIP_TEMP_FOLDER;
use crate::constants::URL_PREFIX;
use crate::repo::RepoResult;

pub struct WithoutGit {
  repository: RepoResult,
  repo_url: String,
  config: JSONConfig,
  tmp_gzip: PathBuf,
}

impl BaseTrait for WithoutGit {
  fn new(repository: RepoResult, config: JSONConfig) -> Self {
    Self {
      repository,
      repo_url: String::new(),
      config,
      tmp_gzip: PathBuf::new(),
    }
  }

  fn ready(&mut self) {
    let repo = &self.repository;
    let url = format!(
      "{}{}/{}/tarball/{}",
      URL_PREFIX, repo.user, repo.repository, repo.branch
    );

    self.repo_url.push_str(url.as_ref());

    let mut tmp_gzip = Path::new(&self.config.exec_root).to_path_buf();
    tmp_gzip.push(GZIP_TEMP_FOLDER);
    tmp_gzip.push(format!("{}-{}.tar.gz", repo.user, repo.output_folder));

    self.tmp_gzip.push(tmp_gzip);
  }

  fn fetch_download(&mut self) -> Result<(), String> {
    let response = reqwest::get(&self.repo_url);
    let result = block_on(response);

    if result.is_err() {
      let unw_err = result.unwrap_err();

      if unw_err.is_status() {
        return Err("Server response an invalid status".into());
      } else if unw_err.is_connect() {
        return Err("Connection Error".into());
      } else if unw_err.is_timeout() {
        return Err("Connection Timeout".into());
      }
      return Err("Unknown Error".into());
    }

    let resolve_byte = block_on(result.unwrap().bytes());
    let result_byte = resolve_byte.unwrap();

    return self.build_file(&result_byte);
  }

  fn build_file(&mut self, mut byte: &[u8]) -> Result<(), String> {
    let tmp_file = File::create(&self.tmp_gzip);

    if tmp_file.is_err() {
      return Err("Unable to create temporary file.".into());
    }

    let copy_result = io::copy(&mut byte, &mut tmp_file.unwrap());

    if copy_result.is_err() {
      return Err("Failed to consume temporary file".into());
    }

    Ok(())
  }
}
