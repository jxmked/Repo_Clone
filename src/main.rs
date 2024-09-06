#![feature(fs_try_exists)]


use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::io;
use std::path::PathBuf;
use std::path::Path;

mod util;





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
    println!("Optional");
    println!("  -w   :   With git data. (False by default)");
    println!("  -p   :   Do pull request with existing repo.");
    println!("       :   Only works if the repo has been cloned with git data.");
    println!("\n");
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

fn get_config_file() -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    // dir.push("config");
    dir.push("ini.io");
    Ok(dir)
}



fn main() {
    // let path = inner_main().expect("Couldn't");
    // println!("{}", path.display());

    let args: Vec<_> = env::args().collect();
    let mut index = 0;

    for e in &args {
        index += 1;

        if e == "set" {
             if util::not(args.len() > 2) {
                // Do print help and exit
                print_help();
                std::process::exit(1);
            }

            if args[index] == "output_folder" {
                // Do set output folder

                // Get executable path
                // Check if config exists


                // If not create one (json type)
                // If exists, update.
                //  x-clone.d

                let pp = get_config_file().expect("shutdown");

                if Path::new(&pp).is_file() {
                    println!("Modify");
                } else {
                    println!("Create");
                }

                println!("Set Output folder")
            }
        }

    }
}
