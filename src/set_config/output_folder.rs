use crate::config_file_rw as conf_rw;
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
  let pp: &Path = Path::new::<str>(&given_path);
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
    println!("\n\tSetting did not set. Exiting...\n");
    std::process::exit(1);
  }

  let mut conf_r = conf_rw::read_json_file().unwrap();

  conf_r.output_path = given_path;

  

  conf_rw::write_json_file(conf_r);

  println!("Set Output folder");
}
