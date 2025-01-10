mod main_branch;
mod output_folder;
mod repo_info;

use repo_info::repo_info;

pub fn repo(username: &str, repo: &str, branch: &str) {

  repo_info(username, repo);
}
