use lazy_static::lazy_static;
use regex::Regex;
use std::env;

use std::ops::Not;

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=1434052276de34362138cba939f6967a
fn not<T: Not>(x: T) -> <T as Not>::Output {
    Not::not(x)
}

// https://github.com/jxmked/Repo_Clone
// https://github.com/jxmked/Repo_Clone/tree/gh-pages

lazy_static! {
    static ref R_IS_FLAG: Regex = Regex::new(r"^(\-)([a-zA-Z]+)$").unwrap();
    static ref R_GITHUB_REPO_A: Regex = Regex::new(r"^(https?\:\/\/)(www)?").unwrap();
    static ref R_GITHUB_REPO_B: Regex = Regex::new(r"(github\.com)(\/[\w\-\_0-9]{1,39})").unwrap();
    static ref R_GITHUB_REPO_C: Regex = Regex::new(r"(\/[a-zA-Z\-\_0-9]+){2,9}(\.git)?$").unwrap();
}

// fn print_help() {
//     println!("\nx-clone requires parameters");
//     println!("  $ x-clone <git repo url>\n");
//     println!("Optional");
//     println!("  -w   :   With git data. (False by default)");
//     println!("  -p   :   Do pull request with existing repo.");
//     println!("       :   Only works if the repo has been cloned with git data.");
//     println!("\n");

// }

fn is_flag(v: &str) -> bool {
    return R_IS_FLAG.is_match(v);
}

fn is_git_url(v: &str) -> bool {
    if not(R_GITHUB_REPO_A.is_match(v)) {
        return false;
    }
    if not(R_GITHUB_REPO_B.is_match(v)) {
        return false;
    }
    if not(R_GITHUB_REPO_C.is_match(v)) {
        return false;
    }

    true
}

fn main() {
    let args: Vec<_> = env::args().collect();

    for e in &args {
        println!("{} : {}", e, is_git_url(e));
    }
}
