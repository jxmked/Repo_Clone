use crate::config_file_rw::JSONConfig;
use crate::repo::RepoResult;

pub trait BaseTrait {
  fn new(repository: RepoResult, config: JSONConfig) -> Self;
  fn ready(&mut self);
  fn fetch_download(&mut self) -> Result<(), String> ;
  fn build_file(&mut self, path: &[u8]) -> Result<(), String>;
}



