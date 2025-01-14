use crate::repo::OutputFolder;
use crate::repo::RepoResult;

pub trait BaseTrait {
  fn new(repository: RepoResult) -> Self;
  fn ready_url(&mut self);
  // pub fn begin_fetch();
}
