use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
  pub static ref R_IS_FLAG: Regex = Regex::new(r"^(\-)([a-zA-Z]+)$").unwrap();
  pub static ref R_GITHUB_REPO_A: Regex = Regex::new(r"^(https?\:\/\/)(www)?").unwrap();
  pub static ref R_GITHUB_REPO_B: Regex =
    Regex::new(r"(github\.com)(\/[\w\-\_0-9]{1,39})").unwrap();
  pub static ref R_GITHUB_REPO_C: Regex =
    Regex::new(r"(\/[a-zA-Z\-\_0-9]+){2,9}(\.git)?$").unwrap();
  pub static ref R_REMOVE_QOUTES: Regex = Regex::new(r#"("|')"#).unwrap();
}

pub fn is_flag(v: &str) -> bool {
  return R_IS_FLAG.is_match(v);
}

pub fn is_git_url(v: &str) -> bool {
  if !R_GITHUB_REPO_A.is_match(v) {
    return false;
  }
  if !R_GITHUB_REPO_B.is_match(v) {
    return false;
  }
  if !R_GITHUB_REPO_C.is_match(v) {
    return false;
  }

  true
}
