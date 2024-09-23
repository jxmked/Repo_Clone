use std::fs;
use std::future::IntoFuture;

extern crate reqwest;

use std::fs::File;
// use std::path::Path;
use std::path::PathBuf;

use flate2::read::GzDecoder;
use reqwest::Response;
use tar::Archive;

use crate::constants;
use crate::git_url_destructor::GitUrlDestructor;
// use crate::r_patten_func;

pub async fn without_git(
  data: GitUrlDestructor,
  output_dir: &str,
  tmp_dir: &str,
  random_str: &str,
) -> Result<(), reqwest::Error> {
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

  println!("{}", url);

  // Setting up temp folder
  let mut exe_root: PathBuf = std::env::current_exe().unwrap();
  exe_root.pop();
  exe_root.push(&constants::GZIP_TEMP_FOLDER);

  match fs::create_dir_all(&exe_root) {
    Ok(_) => {}
    Err(_err) => {
      println!(
        "Folder already exists, {}",
        &exe_root.to_str().as_slice()[..][0]
      );
    }
  }

  let exe_root: &PathBuf = &exe_root.join(&format!("{}.tar.gz", &random_str));

  println!("File, {}", &exe_root.to_str().as_slice()[..][0]);

  // Creating connection and downloading repo
  let resp: Response = reqwest::get(url).into_future().await.unwrap();
  let mut u8_bytes: &[u8] = &resp.bytes().await.unwrap();

  let mut outfile_writer: File = File::create(&exe_root).expect("failed to create file");

  // Write file into temp folder
  std::io::copy(&mut u8_bytes, &mut outfile_writer).expect("failed to copy content");

  // Unpack file to final destination

  let open_zip_decom: File = File::open(&exe_root).unwrap();

  let tar: GzDecoder<File> = GzDecoder::new(open_zip_decom);
  let mut archive: Archive<GzDecoder<File>> = Archive::new(tar);

  match archive.unpack(output_dir) {
    Ok(_) => {
      println!("Okay")
    }
    Err(_err) => {
      println!("Something went wrong");
    }
  }

  // println!("{}", first_dir_entry.unwrap().path().unwrap().display());

  // if Some(first_dir_entry) {
  //   first_dir_en
  // }
  // if let Some(entry) = first_dir_entry {
  //     let dir_path = entry.path().unwrap();

  //     // entry.unpack_in(output_dir).unwrap();
  //     println!("Extracted contents of {} to {}", dir_path.display(), output_dir);
  // } else {
  //     println!("No directories found in the archive.");
  // }
  // // archive.unpack(output_dir).unwrap();

  Ok(())
}
