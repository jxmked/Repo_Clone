mod directory_creator;
/**
 * These functions exit the program for any invalid or fail response
 * or runtime error.
 */
mod output_folder;
mod repo_info;

use directory_creator::DirectoryCreator;
use repo_info::repo_info;

use crate::clone::CloneMode;
use crate::config_file_rw::JSONConfig;
use crate::git_url_destructor::GitUrlDestructor;
use crate::util::exit;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RepoResult {
  pub output_folder: String,
  pub user: String,
  pub repository: String,
  pub branch: String,
}

pub struct RepoReturnValue {
  pub result: RepoResult,
  pub path: OutputFolder,
}

pub struct OutputFolder {
  pub is_owner_exists: bool,
  pub is_repository_exists: bool,
  pub create: DirectoryCreator,
  output_folder: String,
  repo_result: RepoResult,
}

pub fn repo(
  url_destructor: &GitUrlDestructor,
  conf: &JSONConfig,
  mode: &CloneMode,
) -> Result<RepoReturnValue, ()> {
  let username = &url_destructor.username;
  let repo = &url_destructor.repository;
  let branch = &url_destructor.branch;

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

  let mut output_folder = OutputFolder::new(ret.clone(), &conf.output_path);

  output_folder.set_directory_creator();

  let is_valid_directory = output_folder.validate_repository_folder();

  match mode {
    CloneMode::Pull => {
      // With pull request, wWe need to verify the validity of already cloned repository
      // in an opposite way.
      if !is_valid_directory.is_err() {
        println!("Directory doesn't exists!");
        println!("Add '-w' to your command to clone it with remote data");
        exit(1);
      }

      // We should also check if the cloned repository has
      // remote data...
      let mut sample = output_folder.create.repository_path.clone();
      sample.push(".git");

      if !sample.exists() {
        println!("Directory does not have remote data!");
        println!("Delete the current repository and clone it with '-w' remote data");
        exit(1);
      }
    }

    CloneMode::With | CloneMode::Without => {
      if is_valid_directory.is_err() {
        println!("{}", is_valid_directory.as_ref().unwrap_err());
        println!("  Exiting...");
        exit(1);
      }
    }
  }

  return Ok(RepoReturnValue {
    path: output_folder,
    result: ret,
  });
}
