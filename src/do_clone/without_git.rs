/**
 * What to do?
 * Extract the contents of the archive to git clone folder then.
 * Since the foldername of the output was shitty thing,
 * we just read the Archive root folder (the very first folder)
 * then match it from extracted folder into git clone folder.
 * Match it the rename it. That is so sleek!!
 */
use std::fs;
use std::future::IntoFuture;

extern crate reqwest;

use std::fs::File;
// use std::path::Path;
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use reqwest::Response;
use tar::Archive;

use crate::constants;
use crate::git_url_destructor::GitUrlDestructor;
// use crate::r_patten_func;

// Get first folder name
fn get_first_folder_name(mut ar: Archive<GzDecoder<File>>) -> Result<String, &'static str> {
  while let Ok(entry) = ar.entries() {
    for ex in entry {
      match ex {
        Ok(ez) => {
          if ez.header().entry_type() == tar::EntryType::Directory {
            return Ok(ez.path().unwrap().to_string_lossy().to_string());
          }
        }
        Err(_) => (),
      }
    }
  }

  Ok(String::new()) // Return an empty String if no folders found
}

fn remove_last_char(s: &str) -> String {
  if s.is_empty() {
      return String::new(); // Return empty string if input is empty
  }

  s[..s.len() - 1].to_string()
}

pub async fn without_git(
  data: GitUrlDestructor,
  output_dir: &str,
  final_output_dir: &str,
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

  // Setting up temp folder to store archive file
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

  // Path and name of Archive
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

  let fes = get_first_folder_name(archive);

  // Rename shitty foldername into our desired name
  match fes {
    Ok(result) => {

      let mut old_name = Path::new(output_dir).to_path_buf();
      old_name.push(remove_last_char(&result));

      let new_name = Path::new(final_output_dir).to_path_buf();

      println!(
        "{} ----- {} ----- {}",
        result.to_string(),
        old_name.to_string_lossy().to_string(),
        new_name.to_string_lossy().to_string()
      );

      // fs::rename(old_name, new_name)
    }
    Err(_) => println!("Error folder name"),
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
