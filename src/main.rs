mod config_file_rw;
mod r_patten_func;
mod util;

use config_file_rw::read_json_file;

use std::env;
use std::path::Path;

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

      if !args.len() > 3 {
        // Do print help and exit
        print_help();
        std::process::exit(1);
      }

      if args[i + 1] == "output_folder" {
        set_conf_outfol(&args[i + 2]);
      }
    }
  }
}

fn set_conf_outfol(folder_path: &str) {
  // I just want to know where I am. Hahaha
  println!("\nSetting output folder to save your cloned repo :)");

  // Do set output folder

  // Get executable path
  // Check if config exists

  // If not create one (json type)
  // If exists, update.
  //  x-clone.d

  let given_path = r_patten_func::R_REMOVE_QOUTES
    .replace_all(&folder_path, "")
    .to_string();
  let pp = Path::new(&given_path);
  let mut has_error = false;

  println!(r#"  Path: '{}'"#, pp.display());

  // Validate given path.
  //    must be absolute.
  //    must already exists
  if !pp.is_absolute() {
    has_error = true;
    println!("Folder path must be absolute path!");
  }

  if !pp.is_dir() {
    has_error = true;
    println!("Path must be already existing!");
  }

  if has_error {
    std::process::exit(1);
  }

  println!("Set Output folder");
}
