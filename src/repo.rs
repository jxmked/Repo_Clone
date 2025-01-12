mod output_folder;
mod repo_info;

use std::path::PathBuf;

use output_folder::output_folder;
use repo_info::repo_info;

use crate::{config_file_rw::JSONConfig, util::exit};

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RepoResult {
  pub output_folder: String,
  pub user: String,
  pub repository: String,
  pub branch: String,
}

#[derive(Serialize, Deserialize)]
pub struct RepoReturnValue {
  pub result: RepoResult,
  pub path: PathBuf,
}

pub fn repo(username: &str, repo: &str, branch: &str, conf: &JSONConfig) -> RepoResult {
  // Fetch repository info
  let res = repo_info(username, repo, conf);

  if res.is_err() {
    println!("{}", res.as_ref().unwrap_err());
    println!("  Exiting...");
    exit(1);
  }

  let info = res.ok().unwrap();

  let default_branch = info.default_branch.clone();
  let mut used_branch = info.default_branch;

  if !branch.is_empty() {
    if !branch.eq_ignore_ascii_case(&default_branch) {
      used_branch = branch.to_string();
    }
  }

  let ret = RepoResult {
    output_folder: format!("{} ({})", info.name, used_branch),
    user: info.owner.login,
    repository: info.name,
    branch: default_branch,
  };

  let abs_path = output_folder(&ret, conf);

  if abs_path.is_err() {
    println!("{}", abs_path.unwrap_err());
    println!("  Exiting...");
    exit(1);
  }

  return ret;
}
