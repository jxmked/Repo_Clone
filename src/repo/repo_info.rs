use futures::executor::block_on;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};

use crate::config_file_rw::{is_token_set, JSONConfig};
use crate::constants;

#[derive(Serialize, Deserialize, Debug)]

pub struct RepoUser {
  pub login: String,
}

// derive debug has been added having trouble for the repo function
// to understand that this Object doesnt not going to handle
// the error since error from serde_json has been filtered
// before reaching the good parsed value.
#[derive(Serialize, Deserialize, Debug)]
pub struct RepoReturn {
  pub default_branch: String,
  pub name: String,
  pub owner: RepoUser,
}

/**
 * These functions are responsible for fetching the entire
 * info about a defined repository on github.
 * Also, returns an invalid response for no network or a
 * repository does not exists...
 */

// https://api.github.com/repos/{user}/{repo}

// fn get_header_value(header_map: &HeaderMap, key: &str) -> String {
//   let value = header_map.get(key);

//   value.unwrap().to_str().unwrap().to_string()
// }

fn fetch_repo_info(url: &str, jconfig: &JSONConfig) -> Result<String, String> {
  let initialized_client = Client::new();

  let mut req_builder = initialized_client
    .get(url)
    .header(ACCEPT, "application/vnd.github+json")
    .header(USER_AGENT, constants::USER_AGENT);

  // Insert token header if we have a token available.
  // The problem was we didnt know if the token was valid
  // so we just go ahead at this time.
  if is_token_set() {
    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", jconfig.token));
  }

  let req_build = req_builder.build();
  let response = Client::execute(&initialized_client, req_build.unwrap());

  let result = block_on(response);

  if result.is_err() {
    return Err("Invalid server response or connection error...".into());
  }

  if !result.as_ref().unwrap().status().is_success() {
    return Err(format!(
      "Response: Invalid status code {}...",
      result.unwrap().status()
    ));
  }

  // What we gonna do with this???

  // let header_map = result.as_ref().clone().unwrap().headers();
  // let limit_remains = get_header_value(header_map, "x-ratelimit-remaining");
  // let limit_max = get_header_value(header_map, "x-ratelimit-limit");
  // let mut limit_diff: u16 = 0;

  // if r_patten_func::is_numeric_only(&limit_remains) && r_patten_func::is_numeric_only(&limit_max) {
  //   let ml = limit_max.parse::<u16>().unwrap();
  //   let mr = limit_remains.parse::<u16>().unwrap();

  //   limit_diff = ml - mr;
  // }

  // println!("{} - {}  :  {}", limit_remains, limit_max, response_status);

  let res = result.unwrap();
  return Ok(block_on(res.text()).unwrap());
}

pub fn repo_info(
  username: &str,
  repo_name: &str,
  jconfig: &JSONConfig,
) -> Result<RepoReturn, String> {
  let url = format!("https://api.github.com/repos/{username}/{repo_name}");

  let response = fetch_repo_info(&url, &jconfig);

  if response.is_err() {
    // Connection/Unstable/ etc... connection
    // Terminate program
    return Err(response.to_owned().unwrap_err().to_string());
  }

  let parsed_response = serde_json::from_str(response.as_ref().unwrap());

  if parsed_response.is_err() {
    return Err("Failed to parse server response...".into());
  }

  Ok(parsed_response.unwrap())
}
