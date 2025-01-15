mod clone;
mod config_file_rw;
mod constants;
mod r_patten_func;
mod repo;
mod set_config;
mod util;

mod git_url_destructor;
use crate::git_url_destructor::GitUrlDestructor as UrlDestruct;

// use crate::do_clone as clone_mod;

use std::env;

use clone::{clone, CloneMode};
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
async fn begin_clone(url: &str, mode: &CloneMode) {
  if !config_file_rw::is_output_dir_set() {
    println!("Unable to clone anything...");
    println!("Output directory is not yet set.");
    println!("Use 'x-clone set output_folder <absolute folder>' to set it.");
    util::exit(1);
  }

  let conf = config_file_rw::read_json_file();

  let mut url_destructor = UrlDestruct::new(url);
  url_destructor.exec_split();

  let repo_ret = repo(&url_destructor, &conf).unwrap();
  let mut output_folder = repo_ret.path;
  let repository = repo_ret.result;

  let wggg = match mode {
    CloneMode::With => "",
    CloneMode::Without => "out",
    CloneMode::Pull => "",
  };

  println!("\nCloning...");
  println!(
    " * https://github.com/{}/{}/tree/{}",
    repository.user, repository.repository, repository.branch
  );
  println!(" - with{} remote data...", wggg);

  clone(repository, &mut output_folder, conf, mode);

  // without_git(
  //   repository,
  //   output_folder,
  //   "asdjha",
  // )
  // .await
  // .unwrap();
}

fn main() {
  util::random(9);

  let args: Vec<_> = env::args().collect();

  if args.len() <= 1 {
    print_help();
    util::exit(1);
  }

  let mut clone_mode = CloneMode::Without;

  for i in 1..args.len() {
    let argv: String = args[i].to_string();

    if argv == "set" {
      // Prevent set mode if set flag is not the first arg
      if i != 1 {
        println!("Unable to set config with that set of arguments.\n");
        util::exit(1);
      }

      if !(args.len() > 3) {
        // Do print help and exit
        print_help();
        util::exit(1);
      }

      if args[i + 1] == "output_folder" {
        set_config::output_folder::output_folder(&args[i + 2]);
      } else if args[i + 1] == "token" {
        set_config::token::token(&args[i + 2]);
      } else {
        println!("\nNothing to recongif");
      }

      util::exit(1);
    } else {
      if r_patten_func::is_flag(&argv) {
        match &argv[..] {
          "-p" => clone_mode = CloneMode::Pull,
          "-w" => clone_mode = CloneMode::With,
          _ => {
            println!("\nFlag could not recognized!");
            util::exit(1);
          }
        }
      } else if r_patten_func::is_git_url(&argv) {
        begin_clone(&argv, &clone_mode);
      } else {
        println!("Invalid argument: {}", argv);

        util::exit(1);
      }
    }
  }
}
