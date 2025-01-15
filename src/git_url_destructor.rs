use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
  // Match from start of the url into before username
  static ref git_host: Regex = Regex::new(r"^(http(s)\:\/\/)?(www\.)?github\.com\/").unwrap();

  // Match any non slashes characters
  static ref git_non_slash: Regex = Regex::new(r"([a-zA-Z0-9\-\_]+)").unwrap();
}

pub struct GitUrlDestructor {
  pub url: String,
  pub username: String,
  pub repository: String,
  pub branch: String,
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

  pub fn exec_split(&mut self) {
    let n_prot_host = git_host.replace(&self.url, "").to_string(); // No Protocol and host
    let res: Vec<&str> = git_non_slash
      .find_iter(&n_prot_host)
      .map(|m| m.as_str())
      .collect();

    self.username = res[0].to_string();
    self.repository = res[1].to_string();

    if res.len() >= 3 {
      self.branch = res[3].to_string();
    }
  }

  // pub fn branch_defined(&self) -> bool {
  //   !self.branch.is_empty()
  // }
}
