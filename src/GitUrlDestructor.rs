use lazy_static::lazy_static;
use regex::Regex;

lazy_static!{
  static ref git_host: Regex = Regex::new(r"(http(s)\:\/\/)?(www\.)?github\.com\/").unwrap();

}

pub struct GitUrlDestructor {
  url: String,
  username: String,
  repository: String,
  branch: String
}

impl GitUrlDestructor {
  pub fn new(url: &str) -> Self {
    Self {
      url: url.to_string(),
      username: "".to_string(),
      repository: "".to_string(),
      branch: "".to_string(),
    }
  }

  pub fn exec_split(&self) {
    let n_prot = git_host.replace(&self.url, "");


  }
}
