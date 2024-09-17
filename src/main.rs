mod config_file_rw;
mod r_patten_func;
mod util;

mod do_clone;

mod constants;
mod set_config;

mod git_url_destructor;
use crate::git_url_destructor::GitUrlDestructor as UrlDestruct;

// use crate::do_clone as clone_mod;
use crate::set_config as set_conf;

use std::{env, fs};
use std::path::Path;
use do_clone::without_git::without_git;
use util::exit;

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

  let mut gud = UrlDestruct::new(url);
  gud.exec_split();

  println!(
    "{}-{}-{}",
    gud.username,
    gud.repository,
    gud.branch_defined()
  );

  let branch = if gud.branch_defined() {
    &gud.branch
  } else {
    &constants::MASTER_BRANCH.to_string()
  };

  let conf = config_file_rw::read_json_file().unwrap();


  // Prefer output directory
  let path = Path::new(&conf.output_path);
  let path = path.join(&gud.username);
  let path = path.join(format!("{} ({})", gud.repository, branch));
  let out_final_path = path.to_str().as_slice()[..][0];

  fs::create_dir_all(out_final_path).unwrap();

  println!("To {}", &out_final_path);

  without_git(gud, out_final_path, "asdjha").await.unwrap();

  // if r_patten_func::is_sub_branch(url) {
  //   println!("Sub branch");
  // } else {
  //   println!("Not sub branch");
  // }

  // let opath = config_file_rw::read_json_file().unwrap().output_path;

  // let f_url = format!("{url}/tarball/master");
  // let f_out = format!("{opath}/asdasdasd");

  // clone_mod::without_git::without_git(&f_url, &f_out).await.unwrap();
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
