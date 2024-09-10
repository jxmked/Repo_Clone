use crate::config_file_rw as conf_rw;
use crate::r_patten_func;

pub fn token(gh_token: &str) {
  println!("\nSetting token to clone your private repo and do pull-push :)");

  let trimmed_token = r_patten_func::R_REMOVE_QOUTES
    .replace_all(&gh_token, "")
    .to_string();

  let mut has_error = false;

  if !r_patten_func::is_git_pat(gh_token) {
    has_error = true;

    println!("Token must be a classic type token and valid.");
  }

  if has_error {
    println!("\n\tSetting did not set. Exiting...\n");
    std::process::exit(1);
  }

  let mut conf_r = conf_rw::read_json_file().unwrap();

  conf_r.token = trimmed_token;

  let _ = conf_rw::write_json_file(conf_r);

  println!("\nToken has been set!");
}
