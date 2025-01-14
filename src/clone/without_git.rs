use super::base_trait::BaseTrait;

use crate::constants::URL_PREFIX;
use crate::repo::OutputFolder;
use crate::repo::RepoResult;

pub struct WithoutGit {
  repository: RepoResult,
  repo_url: String,
}

impl BaseTrait for WithoutGit {
  fn new(repository: RepoResult) -> Self {
    Self {
      repository,
      repo_url: String::new(),
    }
  }

  fn ready_url(&mut self) {
    let repo = &self.repository;
    let url = format!(
      "{}{}/{}/tarball/{}",
      URL_PREFIX, repo.user, repo.repository, repo.branch
    );

    self.repo_url.push_str(url.as_ref());
  }
}
