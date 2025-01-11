mod output_folder;
mod repo_info;

use repo_info::repo_info;

use crate::config_file_rw::JSONConfig;

pub fn repo(username: &str, repo: &str, jconfig: &JSONConfig) {
  let res = repo_info(username, repo, jconfig);

  println!(
    "{} - {} - {}",
    res.owner.login, res.name, res.default_branch
  );
}
