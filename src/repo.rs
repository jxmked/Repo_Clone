mod main_branch;
mod output_folder;
mod repo_info;

use repo_info::repo_info;

use crate::config_file_rw::JSONConfig;

pub fn repo(username: &str, repo: &str, branch: &str, jconfig:&JSONConfig) {

  repo_info(username, repo, jconfig);
}
