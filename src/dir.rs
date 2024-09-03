use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Dir {
  path: String,
} 

impl Dir {
  pub fn new(path: String) -> Self {
      Self { path }
  }

  pub fn call_path(&self) {
      println!("Hey you, {}", self.path);
  }
}
