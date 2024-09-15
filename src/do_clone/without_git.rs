use std::future::IntoFuture;

extern crate reqwest;

use std::fs::File;
use std::path::Path;
use tar::Archive;

use flate2::read::GzDecoder;

pub async fn without_git(url: &str, output_dir: &str) -> Result<(), reqwest::Error> {
  let resp = reqwest::get(url);

  let ii = resp.into_future();

  let ee = ii.await;

  let x_path = Path::new(&output_dir);

  let efile = &x_path.join("samp.tar.gz");

  let mut out = File::create(efile).expect("failed to create file");

  let ss = ee.unwrap();

  let mut ep: &[u8] = &ss.bytes().await.unwrap();
  std::io::copy(&mut ep, &mut out).expect("failed to copy content");

  let gg = File::open(efile).unwrap();

  let tar = GzDecoder::new(gg);
  let mut archive = Archive::new(tar);
  let _ = archive.unpack(x_path.join("sett"));

  Ok(())
}
