mod config_file_rw;
mod util;

use config_file_rw::read_json_file;
use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::io;
use std::path::{Path, PathBuf};

// https://github.com/jxmked/Repo_Clone
// https://github.com/jxmked/Repo_Clone/tree/gh-pages

lazy_static! {
  static ref R_IS_FLAG: Regex = Regex::new(r"^(\-)([a-zA-Z]+)$").unwrap();
  static ref R_GITHUB_REPO_A: Regex = Regex::new(r"^(https?\:\/\/)(www)?").unwrap();
  static ref R_GITHUB_REPO_B: Regex = Regex::new(r"(github\.com)(\/[\w\-\_0-9]{1,39})").unwrap();
  static ref R_GITHUB_REPO_C: Regex = Regex::new(r"(\/[a-zA-Z\-\_0-9]+){2,9}(\.git)?$").unwrap();
}

fn print_help() {
  println!("\nx-clone requires parameters");
  println!("  $ x-clone <git repo url>\n");
  println!("Optional:");
  println!("  -w   :   With git data. (False by default)");
  println!("  -p   :   Do pull request with existing repo.");
  println!("       :   Only works if the repo has been cloned with git data.");
  println!("\n");

  println!("Set Config:");
  println!("  set output_folder <absolute path>     : Set where all cloned repositories");
  println!("                                        : will be save.");
  println!("  set token <personal token>            : Set Github token to be allowed to");
  println!("                                        : clone respository with git data and");
  println!("                                        : clone your private respository.");
}

fn is_flag(v: &str) -> bool {
  return R_IS_FLAG.is_match(v);
}

fn is_git_url(v: &str) -> bool {
  if util::not(R_GITHUB_REPO_A.is_match(v)) {
    return false;
  }
  if util::not(R_GITHUB_REPO_B.is_match(v)) {
    return false;
  }
  if util::not(R_GITHUB_REPO_C.is_match(v)) {
    return false;
  }

  true
}

fn main() {
  let args: Vec<_> = env::args().collect();

  if args.len() <= 0 {
    print_help();
    std::process::exit(1);
  }

  for i in 0..args.len() {
    if args[i] == "set" {
      if i != 1 {
        println!("Unable to set config with that set of arguments.\n");
        std::process::exit(1);
      }

      if util::not(args.len() > 3) {
        // Do print help and exit
        print_help();
        std::process::exit(1);
      }

      if args[i + 1] == "output_folder" {
        // Do set output folder

        // Get executable path
        // Check if config exists

        // If not create one (json type)
        // If exists, update.
        //  x-clone.d

        let given_path = &args[i + 2];

        // Validate given path.
        //    must be absolute.
        //    must already exists

        println!("Set Output folder");
      }
    }
  }
}
