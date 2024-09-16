use std::future::IntoFuture;

extern crate reqwest;

use std::fs::File;
use std::path::Path;
use tar::Archive;

use flate2::read::GzDecoder;

use crate::constants;
use crate::git_url_destructor::GitUrlDestructor;

pub async fn without_git(data: GitUrlDestructor, output_dir: &str) -> Result<(), reqwest::Error> {
  // We have a problem...
  // If the main branch has different names and the master/main
  // branch remains as sub branch...
  let branch: String = if data.branch_defined() {
    data.branch
  } else {
    constants::MASTER_BRANCH.to_string()
  };

  let url: String = format!(
    "https://github.com/{}/{}/tarball/{}",
    data.username, data.repository, branch
  );

  // Setting up temp folder
  let root_exe = std::env::current_exe().unwrap();
  let root_exe_temp_folder = constants::GZIP_TEMP_FOLDER;

  // hahahaha
  let exe_root = Path::new(&root_exe);
  let exe_root = &exe_root.join(&root_exe_temp_folder);
  let exe_root = &exe_root.join("temp.tar.gz");

  // Setting final output folder
  

  // Creating connection and downloading repo
  let resp: reqwest::Response = reqwest::get(url).into_future().await.unwrap();
  let mut u8_bytes: &[u8] = &resp.bytes().await.unwrap();

  let x_path = Path::new(&output_dir);

  let efile = &x_path.join("samp.tar.gz");

  let mut out = File::create(exe_root).expect("failed to create file");

  std::io::copy(&mut u8_bytes, &mut out).expect("failed to copy content");

  let gg = File::open(exe_root).unwrap();

  let tar = GzDecoder::new(gg);
  let mut archive = Archive::new(tar);
  let _ = archive.unpack(x_path.join("sett"));

  Ok(())
}
