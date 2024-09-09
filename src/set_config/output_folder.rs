use crate::r_patten_func;

use std::path::Path;

pub fn output_folder(folder_path: &str) {
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
