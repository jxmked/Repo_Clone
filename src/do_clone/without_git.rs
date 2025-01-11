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

use std::error;
use std::fs::File;
use std::path::{PathBuf, Path};

use flate2::read::GzDecoder;
use reqwest::Response;
use tar::Archive;

use crate::constants;
use crate::git_url_destructor::GitUrlDestructor;
use crate::util;
// use crate::r_patten_func;

// Get first folder name
fn get_first_folder_name(
  mut archive: Archive<GzDecoder<File>>,
) -> Result<String, Box<dyn error::Error>> {
  while let Ok(first_entry) = archive.entries() {
    for entry in first_entry {
      if entry.as_ref().unwrap().header().entry_type() == tar::EntryType::Directory {
        return Ok(entry.unwrap().path().unwrap().to_string_lossy().to_string());
      }
    }
  }
  Err("No root folder found in archive.".into())
}

fn open_archive_for_read(path: PathBuf) -> Result<Archive<GzDecoder<File>>, &'static str> {
  match File::open(path) {
    Ok(file) => {
      let tar: GzDecoder<File> = GzDecoder::new(file);
      return Ok(Archive::new(tar));
    }

    Err(_) => {}
  }
  Err("Unable to open file as Archive")
}

// Decom archive file
fn extract_archive_to_folder(file_to_decomp: PathBuf, output_dir: &str) {
  // File to decompress
  let opened_archive = open_archive_for_read(file_to_decomp);

  match opened_archive {
    Ok(mut archive) => match archive.unpack(output_dir) {
      Ok(_) => {
        println!("Unpacked")
      }
      Err(_err) => {
        println!("Something went wrong while unpacking...");
      }
    },

    Err(_) => {}
  }
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


  // Creating connection and downloading repo
  let resp: Response = reqwest::get(url).into_future().await.unwrap();  // TODO: Handle any internet error
  let mut u8_bytes: &[u8] = &resp.bytes().await.unwrap();

  let mut outfile_writer: File = File::create(&exe_root).expect("failed to create file");

  // Write file into temp folder
  std::io::copy(&mut u8_bytes, &mut outfile_writer).expect("failed to copy content");

  // Unpack file to final destination
  extract_archive_to_folder(exe_root.to_path_buf(), output_dir);

  // Rename shitty output folder into desired one
  match open_archive_for_read(exe_root.to_path_buf()) {
    Ok(archive) => match get_first_folder_name(archive) {
      Ok(filename) => {
        let mut relative_path = Path::new(&output_dir.to_string()).to_path_buf();

        relative_path.push(util::remove_last_char(&filename));

        let old_dir = relative_path.to_string_lossy().to_string();

        relative_path.pop();
        relative_path.push(final_output_dir);

        let new_dir = relative_path.to_string_lossy().to_string();

        match fs::rename(old_dir, new_dir) {
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
    },
    Err(_) => todo!(),
  }

  Ok(())
}
