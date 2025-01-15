mod base_trait;
mod pull_git;
mod with_git;
mod without_git;

use base_trait::BaseTrait;
use with_git::WithGit;
use without_git::WithoutGit;

use crate::config_file_rw::JSONConfig;
use crate::repo::OutputFolder;
use crate::repo::RepoResult;
use crate::util::exit;

pub enum CloneMode {
  With,
  Without,
  Pull,
}

pub fn clone(
  repository: RepoResult,
  output_folder: &mut OutputFolder,
  config: JSONConfig,
  mode: &CloneMode,
) {
  match mode {
    CloneMode::Pull => {
      println!("Git Pull Not Yet Available!");
      exit(0);
    }

    CloneMode::With => {
      println!("Cloning with remote data!");

      let mut cloner = WithGit::new(repository, config);

      cloner.ready();

      let copy_result = cloner.copy_to_output(output_folder);

      if copy_result.is_err() {
        println!("{}", copy_result.unwrap_err());
      }
    }

    CloneMode::Without => {
      println!("Cloning without remote data...");

      let mut cloner = WithoutGit::new(repository, config);
      cloner.ready();

      let download_result = cloner.fetch_download();

      if download_result.is_err() {
        println!("{}", download_result.unwrap_err());
        exit(1);
      }

      let copy_result = cloner.copy_to_output(output_folder);

      if copy_result.is_err() {
        println!("{}", copy_result.unwrap_err());
      }
    }
  }

  exit(0);
}
