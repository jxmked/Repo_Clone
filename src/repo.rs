mod output_folder;
mod repo_info;

use repo_info::repo_info;

use crate::config_file_rw::JSONConfig;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RepoResult {
  pub output_folder: String,
  pub user: String,
  pub repository: String,
  pub branch: String,
}

pub fn repo(username: &str, repo: &str, branch: &str, conf: &JSONConfig) -> RepoResult {
  let res = repo_info(username, repo, conf);
  let default_branch = res.default_branch.clone();
  let mut used_branch = res.default_branch;

  if !branch.is_empty() {
    if !branch.eq_ignore_ascii_case(&default_branch) {
      used_branch = branch.to_string();
    }
  }

  let ret =  RepoResult {
    output_folder: format!("{} ({})", res.name, used_branch),
    user: res.owner.login,
    repository: res.name,
    branch: default_branch,
  };

  output_folder(ret, conf);

  return ret;
}
