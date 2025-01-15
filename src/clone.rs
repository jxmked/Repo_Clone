mod base_trait;
mod without_git;

use base_trait::BaseTrait;
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
  let mut cloner = WithoutGit::new(repository, config);

  match mode {
    CloneMode::Pull => {
      println!("Git Pull Not Yet Available!");
      exit(0);
    }

    CloneMode::With => {
      println!("Cloning with Data Not Yet Available!");
      exit(0);
    }

    CloneMode::Without => {
      println!("Cloning without remote data...")
    }
  }

  cloner.ready();
  let download_result = cloner.fetch_download();

  if download_result.is_err() {
    println!("{}", download_result.unwrap_err());
    exit(1);
  }

  let copy_result = cloner.copy_to_output(output_folder);

  if copy_result.is_err() {
    println!("{}", copy_result.unwrap_err());
    exit(1);
  }
}
