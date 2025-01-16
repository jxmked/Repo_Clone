use chrono::{self, Datelike};
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::repo::RepoResult;

use super::CloneMode;

fn add_spaces(v: String, len: usize) -> String {
  let str_len = v.chars().count();
  let s = String::from(" ").repeat((len - str_len) - 1);
  format!("{}{}", v, s)
}

fn timestamp() -> String {
  let now = chrono::offset::Local::now();
  let mon_now: usize = now.month0().try_into().unwrap();
  let month = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
  ][mon_now];

  let ret = format!(
    "{} {} {} {}",
    month,
    now.day(),
    now.year(),
    now.format("%H:%M")
  );

  return add_spaces(ret, 20);
}

fn write_log(filepath: PathBuf, data: RepoResult, mode: CloneMode) -> io::Result<()> {
  let mode = match mode {
    CloneMode::Pull => "Pull",
    CloneMode::With => "Clone*",
    CloneMode::Without => "Clone",
  };

  let target = format!("{} > {} > {}", data.user, data.repository, data.branch);

  let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(filepath)?;

  writeln!(
    file,
    "{}| {}| {}",
    timestamp(),
    add_spaces(mode.into(), 8),
    target
  )?;

  Ok(())
}

pub fn log_repo(repository: RepoResult, mode: &CloneMode, path_to_log: String) {
  let mut log_file = Path::new(&path_to_log).to_path_buf();
  log_file.push("log.txt");

  let _ = write_log(log_file, repository, mode.clone()).unwrap();
}
