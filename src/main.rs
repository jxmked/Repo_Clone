mod config_file_rw;
mod r_patten_func;
mod set_config;
mod util;

mod constants;

use crate::set_config as set_conf;

use std::env;
use std::future::IntoFuture;
use util::exit;

extern crate reqwest;

use std::fs::File;
use std::path::Path;
use tar::Archive;

use std::fs;

use flate2::read::GzDecoder;
// use std::os::unix::OpenOptionsExt;

fn print_help() {
  println!("\nx-clone requires parameters");
  println!("  $ x-clone <git repo url>\n");
  println!("Optional:");
  println!("  -w   :   With git data. (False by default)");
  println!("  -p   :   Do pull request with existing repo.");
  println!("       :   Only works if the repo has been cloned with git data.");
  println!("\n");

  println!("Set Config:");
  println!("  set output_folder <absolute path>     : Set where all cloned repositories");
  println!("                                        : will be save.");
  println!("  set token <personal token>            : Set Github token to be allowed to");
  println!("                                        : clone respository with git data and");
  println!("                                        : clone your private respository.");
}

pub async fn download_and_extract(url: &str, output_dir: &str) -> Result<(), reqwest::Error> {
  // let mut resp = reqwest::get("https://sh.rustup.rs").await.expect("request failed");

  // let client = Client::new();
  // let response = client.get(url).send().await?;

  // let filename = url.rsplit('/').next().unwrap();
  // let output_path = output_dir.to_string() + "/" + filename;

  // // Create the output directory if it doesn't exist
  // create_dir_all(output_dir).unwrap();

  // response.

  // // Write the downloaded content to a temporary file
  // let mut temp_file = File::create(output_path).unwrap();
  // let mut content = Vec::new();
  // response.read_to_end(&mut content).unwrap();
  // temp_file.write_all(&content).unwrap();

  // // Extract the archive
  // let mut archive = Archive::new(File::open(output_path).unwrap());
  // archive.unpack(output_dir).unwrap();

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

  // let mut archive = Archive::new(out);
  // archive.unpack(x_path.join("sample")).unwrap();

  Ok(())
}

#[tokio::main]
async fn do_clone(url: &str, with_git: bool) {
  if !config_file_rw::is_output_dir_set() {
    println!("Unable to clone anything...");
    println!("Output dir is not yet set.");
    println!("Use 'x-clone set output_folder <absolute folder>' to set it.");
    exit(1);
  }

  println!("Cloning...");

  let opath = config_file_rw::read_json_file().unwrap().output_path;

  let f_url = format!("{url}/tarball/master");
  let f_out = format!("{opath}/asdasdasd");

  download_and_extract(&f_url, &f_out).await.unwrap();
}

fn main() {
  let args: Vec<_> = env::args().collect();

  if args.len() <= 1 {
    print_help();
    exit(1);
  }

  let mut is_with_git: bool = false;
  let mut is_do_pull: bool = false;

  for i in 1..args.len() {
    let argv: String = args[i].to_string();

    if argv == "set" {
      // Prevent set mode if set flag is not the first arg
      if i != 1 {
        println!("Unable to set config with that set of arguments.\n");
        exit(1);
      }

      if !(args.len() > 3) {
        // Do print help and exit
        print_help();
        exit(1);
      }

      if args[i + 1] == "output_folder" {
        set_conf::output_folder::output_folder(&args[i + 2]);
      } else if args[i + 1] == "token" {
        set_conf::token::token(&args[i + 2]);
      } else {
        println!("\nNothing to recongif");
      }

      exit(1);
    } else {
      if r_patten_func::is_flag(&argv) {
        if is_with_git || is_do_pull {
          println!("Either of flag is already raised. Only one must be raise.");
          exit(1);
        }

        match &argv[..] {
          "-w" => is_with_git = true,
          "-p" => is_do_pull = true,
          _ => {
            println!("\nFlag could not recognized!");
            exit(1);
          }
        }
      } else if r_patten_func::is_git_url(&argv) {
        println!("ASsadasd");

        if !is_do_pull {
          println!("Do cloning");

          do_clone(&argv, is_with_git);
        } else {
          println!("just d pull");
          exit(1);
        }
      } else {
        println!("Invalid argument: {}", argv);

        exit(1);
      }
    }
  }
}
