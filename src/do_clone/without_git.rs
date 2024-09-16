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

  let url = format!(
    "https://github.com/{}/{}/tarball/{}",
    data.username, data.repository, branch
  );

  let resp = reqwest::get(url).into_future().await.unwrap();

  let x_path = Path::new(&output_dir);

  let efile = &x_path.join("samp.tar.gz");

  let mut out = File::create(efile).expect("failed to create file");

  let mut ep: &[u8] = &resp.bytes().await.unwrap();
  std::io::copy(&mut ep, &mut out).expect("failed to copy content");

  let gg = File::open(efile).unwrap();

  let tar = GzDecoder::new(gg);
  let mut archive = Archive::new(tar);
  let _ = archive.unpack(x_path.join("sett"));

  Ok(())
}
