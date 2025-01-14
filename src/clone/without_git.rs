use super::base_trait::Base_Trait;
use crate::repo::RepoResult;

pub struct WithoutGit {}

impl Base_Trait for WithoutGit {
   fn new(repository: RepoResult) -> Self {
    Self {}
  }
}
