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
use std::path::PathBuf;

use flate2::read::GzDecoder;
use reqwest::Response;
use tar::Archive;

use crate::constants;
use crate::git_url_destructor::GitUrlDestructor;
// use crate::r_patten_func;

// Get first folder name
fn get_first_folder_name(
  mut archive: Archive<GzDecoder<File>>,
) -> Result<String, Box<dyn std::error::Error>> {
  for first_entry in archive.entries() {
    for entry in first_entry {
      if entry.as_ref().unwrap().header().entry_type() == tar::EntryType::Directory {
        return Ok(entry.unwrap().path().unwrap().to_string_lossy().to_string());
      }
    }
  }
  Err("No root folder found in archive.".into())
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

  // File to decompress
  let open_zip_decom = File::open(&exe_root);

  match open_zip_decom {
    Ok(file) => {
      let tar: GzDecoder<File> = GzDecoder::new(file);
      let mut archive: Archive<GzDecoder<File>> = Archive::new(tar);

      match archive.unpack(output_dir) {
        Ok(_) => {
          println!("Unpacked");
        }
        Err(_err) => {
          println!("Something went wrong while unpacking...");
        }
      }
    }
    Err(_) => {
      println!("File could not be read.");
    }
  }

  // Rename shitty output folder into desired one
  let open_zip_decom = File::open(&exe_root);

  match open_zip_decom {
    Ok(file) => {
      let tar: GzDecoder<File> = GzDecoder::new(file);
      let archive: Archive<GzDecoder<File>> = Archive::new(tar);

      let res = get_first_folder_name(archive);
      match res {
        Ok(filename) => {
          let mut old_dir = PathBuf::new();

          let target_folder_name = remove_last_char(&filename);

          old_dir.push(output_dir);
          old_dir.push(target_folder_name);

          println!("Out {}", old_dir.display());
          println!("new dir {}", final_output_dir);

          match fs::rename(old_dir, final_output_dir) {
            Ok(_) => {
              println!("Ok")
            }
            Err(_) => {
              println!("Fucked")
            }
          }
        }
        Err(_) => {
          println!("Fail to fetch foldername");
        }
      }
    }
    Err(_) => {
      println!("File could not be read.");
    }
  }

  Ok(())
}
