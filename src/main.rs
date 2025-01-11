mod config_file_rw;
mod r_patten_func;
mod util;

mod repo;

mod do_clone;

mod constants;
mod set_config;

mod git_url_destructor;
use crate::git_url_destructor::GitUrlDestructor as UrlDestruct;

// use crate::do_clone as clone_mod;
use crate::set_config as set_conf;

use do_clone::without_git::without_git;
use std::path::Path;
use std::{env, fs};
use util::exit;

use repo::repo;

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

#[tokio::main]
async fn begin_clone(url: &str, with_git: bool) {
  if !config_file_rw::is_output_dir_set() {
    println!("Unable to clone anything...");
    println!("Output directory is not yet set.");
    println!("Use 'x-clone set output_folder <absolute folder>' to set it.");
    exit(1);
  }

  let conf: config_file_rw::JSONConfig = config_file_rw::read_json_file().unwrap();

  let mut gud = UrlDestruct::new(url);
  gud.exec_split();

  let branch = if gud.branch_defined() {
    &gud.branch
  } else {
    &constants::MASTER_BRANCH.to_string()
  };

  repo(&gud.username, &gud.repository, &gud.branch, &conf);

  let wggg = if with_git { "" } else { "out" };

  println!("\nCloning...");
  println!(
    " * github.com/{}/{}/tree/{}",
    gud.username, gud.repository, branch
  );
  println!(" - with{} data...", wggg);

  // Prefer output directory
  let mut path: std::path::PathBuf = Path::new(&conf.output_path).to_path_buf();
  path.push(&gud.username);

  let mut f_path: std::path::PathBuf = path.clone();
  f_path.push(format!("{} ({})", gud.repository, branch));

  let out_final_path = path.to_str().as_slice()[..][0];

  fs::create_dir_all(out_final_path).unwrap();

  without_git(
    gud,
    out_final_path,
    f_path.to_str().as_slice()[..][0],
    "asdjha",
  )
  .await
  .unwrap();
}

fn main() {
  util::random(9);

  let args: Vec<_> = env::args().collect();

  if args.len() <= 1 {
    print_help();
    exit(1);
  }

  let mut is_with_git: bool = false;
  let mut is_do_pull: bool = false;

  for i in 1..args.len() {
    let argv: String = args[i].to_string();

    if argv == "set" {
      // Prevent set mode if set flag is not the first arg
      if i != 1 {
        println!("Unable to set config with that set of arguments.\n");
        exit(1);
      }

      if !(args.len() > 3) {
        // Do print help and exit
        print_help();
        exit(1);
      }

      if args[i + 1] == "output_folder" {
        set_conf::output_folder::output_folder(&args[i + 2]);
      } else if args[i + 1] == "token" {
        set_conf::token::token(&args[i + 2]);
      } else {
        println!("\nNothing to recongif");
      }

      exit(1);
    } else {
      if r_patten_func::is_flag(&argv) {
        if is_with_git || is_do_pull {
          println!("Either of flag is already raised. Only one must be raise.");
          exit(1);
        }

        match &argv[..] {
          "-w" => is_with_git = true,
          "-p" => is_do_pull = true,
          _ => {
            println!("\nFlag could not recognized!");
            exit(1);
          }
        }
      } else if r_patten_func::is_git_url(&argv) {
        if !is_do_pull {
          begin_clone(&argv, is_with_git);
        } else {
          println!("just d pull");
          exit(1);
        }
      } else {
        println!("Invalid argument: {}", argv);

        exit(1);
      }
    }
  }
}
