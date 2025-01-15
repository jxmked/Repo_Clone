use crate::config_file_rw::JSONConfig;
use crate::repo::{OutputFolder, RepoResult};

pub trait BaseTrait {
  fn new(repository: RepoResult, config: JSONConfig) -> Self;
  fn ready(&mut self);
  fn fetch_download(&mut self) -> Result<(), &str>;
  fn build_file(&mut self, byte: &[u8]) -> Result<(), &str>;
  fn copy_to_output(&mut self, output_folder: &mut OutputFolder) -> Result<(), &str>;
}
