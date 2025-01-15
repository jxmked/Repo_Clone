use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use futures::executor::block_on;
use reqwest;
use tar::Archive;

use super::base_trait::BaseTrait;

use crate::config_file_rw::JSONConfig;
use crate::constants::GZIP_TEMP_FOLDER;
use crate::constants::URL_PREFIX;
use crate::repo::RepoResult;

pub struct WithoutGit {
  repository: RepoResult,
  repo_url: String,
  config: JSONConfig,
  tmp_gzip: PathBuf,
  out_dst: PathBuf,
}

impl BaseTrait for WithoutGit {
  fn new(repository: RepoResult, config: JSONConfig) -> Self {
    Self {
      repository,
      repo_url: String::new(),
      config,
      tmp_gzip: PathBuf::new(),
      out_dst: PathBuf::new(),
    }
  }

  fn ready(&mut self) {
    let repo = &self.repository;
    let conf = &self.config;

    // Prepare fetching url
    let url = format!(
      "{}{}/{}/tarball/{}",
      URL_PREFIX, repo.user, repo.repository, repo.branch
    );

    self.repo_url.push_str(url.as_ref());

    // Prefer temp tar.gz file
    let mut tmp_gzip = Path::new(&self.config.exec_root).to_path_buf();
    tmp_gzip.push(GZIP_TEMP_FOLDER);
    tmp_gzip.push(format!("{}-{}.tar.gz", repo.user, repo.output_folder));

    self.tmp_gzip.push(tmp_gzip);

    self.out_dst.push(&conf.output_path);
    self.out_dst.push(&repo.user);
    self.out_dst.push(&repo.output_folder);
  }

  fn fetch_download(&mut self) -> Result<(), &str> {
    let response = reqwest::get(&self.repo_url);
    let result = block_on(response);

    if result.is_err() {
      let unw_err = result.unwrap_err();

      if unw_err.is_status() {
        return Err("Server responded an invalid status code");
      } else if unw_err.is_connect() {
        return Err("Connection Error");
      } else if unw_err.is_timeout() {
        return Err("Connection Timeout");
      }
      return Err("Unknown Error");
    }

    let resolve_byte = block_on(result.unwrap().bytes());
    let result_byte = resolve_byte.unwrap();

    return self.build_file(&result_byte);
  }

  fn build_file(&mut self, mut byte: &[u8]) -> Result<(), &str> {
    let tmp_file = File::create(&self.tmp_gzip);

    if tmp_file.is_err() {
      return Err("Unable to create temporary file.");
    }

    let copy_result = io::copy(&mut byte, &mut tmp_file.unwrap());

    if copy_result.is_err() {
      return Err("Failed to consume temporary file");
    }

    Ok(())
  }

  fn copy_to_output(&mut self) -> Result<(), &str> {
    let extraction_result = &self.extract_file_to_output();

    if extraction_result.is_err() {
      return Err(&extraction_result.unwrap_err());
    }

    match self.read_file_as_archive() {
      Ok(mut archive) => {
        while let Ok(first_entry) = archive.entries() {
          for entry in first_entry {
            if entry.as_ref().unwrap().header().entry_type() == tar::EntryType::Directory {
              return Ok(entry.unwrap().path().unwrap().to_string_lossy().to_string());
            }
          }
        }
        return Err("Something went wrong at the end...");
      }
      Err(_) => {
        return Err("Failed while opening file");
      }
    }

    Ok(())
  }
}

impl WithoutGit {
  fn read_file_as_archive(&mut self) -> Result<Archive<GzDecoder<File>>, &'static str> {
    let tmp_gzip = &self.tmp_gzip;
    let open_file = File::open(tmp_gzip);

    if open_file.is_err() {
      return Err("Unable to open file");
    }

    let decoder = GzDecoder::new(open_file.unwrap());
    let archive = Ok(Archive::new(decoder));

    return archive;
  }

  fn extract_file_to_output(&mut self) -> Result<(), &str> {
    match self.read_file_as_archive() {
      Ok(mut archive) => match archive.unpack(&self.out_dst) {
        Ok(_) => {
          return Ok(());
        }
        Err(_) => {
          return Err("Failed while unpacking an archive");
        }
      },
      Err(_) => {
        return Err("Failed while opening file");
      }
    }
  }

  fn get_first_folder_name(&mut self) -> Result<String, &str> {
    match self.read_file_as_archive() {
      Ok(mut archive) => {
        while let Ok(first_entry) = archive.entries() {
          for entry in first_entry {
            if entry.as_ref().unwrap().header().entry_type() == tar::EntryType::Directory {
              return Ok(entry.unwrap().path().unwrap().to_string_lossy().to_string());
            }
          }
        }
        return Err("Something went wrong at the end...");
      }
      Err(_) => {
        return Err("Failed while opening file");
      }
    }
  }
}
