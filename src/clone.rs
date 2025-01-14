mod base_trait;
mod without_git;

use base_trait::BaseTrait;
use without_git::WithoutGit;

use crate::repo::OutputFolder;
use crate::repo::RepoResult;

pub enum CloneMode {
  With,
  Without,
  Pull
}

pub fn clone(repository: RepoResult, output_folder: &OutputFolder, mode: CloneMode) {
  let cloner = WithoutGit::new(repository, output_folder);

    

}
