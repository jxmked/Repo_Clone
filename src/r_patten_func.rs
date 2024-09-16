use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
  // check whether the string is start with - and after container only a-z chars
  pub static ref R_IS_FLAG: Regex = Regex::new(r"^(\-)([a-zA-Z]+)$").unwrap();

  // Check whether git repo url is valid. Using single line cause
  // the compiler to fail due to too large something i cannot remember.
  //
  pub static ref R_GITHUB_REPO_A: Regex = Regex::new(r"^(https?\:\/\/)(www\.)?").unwrap(); // Begining of the web
  pub static ref R_GITHUB_REPO_B: Regex =
    Regex::new(r"(github\.com)(\/[\w\-\_0-9]{1,39})").unwrap(); // Host with username
  pub static ref R_GITHUB_REPO_C: Regex =
    Regex::new(r"(\/[a-zA-Z\-\_0-9]+){2,9}(\.git)?$").unwrap(); // Repository name w/o .git at the end

  // Removing single and double qoutes from
  pub static ref R_REMOVE_QOUTES: Regex = Regex::new(r#"("|')"#).unwrap();

  // Check whether the token start with ghp and contains alpha numeric and 40 chars long
  pub static ref R_GIT_PAT: Regex = Regex::new(r"^(ghp_)[a-zA-Z0-9]{36}$").unwrap();

  // Check if /tree/ exists in url
  // pub static ref R_GIT_SUB_BRANCH: Regex = Regex::new(r"\.com\/([a-zA-Z\-\_0-9]+\/){2}tree\/").unwrap();
}

// pub fn is_sub_branch(v: &str) -> bool {
//   let a = R_GITHUB_REPO_A.replace(v, "");
//   let b = R_GITHUB_REPO_B.replace(&a, "");

//   println!("{}", b);
  
//   R_GIT_SUB_BRANCH.is_match(v)
// }

pub fn is_flag(v: &str) -> bool {
  R_IS_FLAG.is_match(v)
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

pub fn is_git_pat(v: &str) -> bool {
  R_GIT_PAT.is_match(v)
}
