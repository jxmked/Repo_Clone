/**
 * These functions exit the program for any invalid or fail response
 * or runtime error.
 */
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

pub fn repo(
  username: &str,
  repo: &str,
  branch: &str,
  conf: &JSONConfig,
) -> Result<RepoReturnValue, ()> {
  // Fetch repository info
  let res = repo_info(username, repo, conf);

  if res.is_err() {
    println!("{}", res.as_ref().unwrap_err());
    println!("  Exiting...");
    exit(1);
  }

  let info = res.ok().unwrap();

  let mut used_branch = info.default_branch.clone();

  if !branch.is_empty() {
    if !branch.eq_ignore_ascii_case(info.default_branch.as_ref()) {
      used_branch = branch.to_string();
    }

    // Either, we can delete the folder since it doesn't contain anything or
    // move files from extracted to into that folder...?????
  }

  let ret = RepoResult {
    output_folder: format!("{} ({})", info.name, used_branch),
    user: info.owner.login,
    repository: info.name,
    branch: used_branch,
  };

  let abs_path = output_folder(&ret, conf);

  if abs_path.is_err() {
    println!("{}", abs_path.as_ref().unwrap_err());
    println!("  Exiting...");
    exit(1);
  }

  return Ok(RepoReturnValue {
    path: abs_path.ok().unwrap(),
    result: ret,
  });
}
