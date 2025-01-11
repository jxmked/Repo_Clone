use futures::executor::block_on;
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::Client;

use crate::constants;
use crate::{config_file_rw, r_patten_func};

/**
 * These functions are esponsible for fetching the entire
 * info about a defined repository on github.
 * Also, returns an invalid response for no network or a
 * repository does not exists...
 */

// https://api.github.com/repos/{user}/{repo}

fn get_header_value(header_map: &HeaderMap, key: &str) -> String {
  let value = header_map.get(key);

  value.unwrap().to_str().unwrap().to_string()
}

fn fetch_repo_info(url: &str, jconfig: &config_file_rw::JSONConfig) -> Result<String, String> {
  let initialized_client = Client::new();

  let mut req_builder = initialized_client
    .get(url)
    .header(ACCEPT, "application/vnd.github+json")
    .header(USER_AGENT, constants::USER_AGENT);

  // Insert token header if we have a token available.
  // The problem was we didnt know if the token was valid
  // so we just go ahead at this time.
  if config_file_rw::is_token_set() {
    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", jconfig.token));
  }

  let req_build = req_builder.build();
  let response = Client::execute(&initialized_client, req_build.unwrap());

  let result = block_on(response);

  if (result.is_err()) {
    println!("errror at here");
    return Err("".into());
  }

  let header_map = result.as_ref().clone().unwrap().headers();

  let response_status = result.as_ref().unwrap().status();

  let limit_remains = get_header_value(header_map, "x-ratelimit-remaining");
  let limit_max = get_header_value(header_map, "x-ratelimit-limit");
  let mut limit_diff: u16 = 0;

  if r_patten_func::is_numeric_only(&limit_remains) && r_patten_func::is_numeric_only(&limit_max) {
    let ml = limit_max.parse::<u16>().unwrap();
    let mr = limit_remains.parse::<u16>().unwrap();

    limit_diff = ml - mr;
  }
  if !response_status.is_success() {}

  println!("{} - {}  :  {}", limit_remains, limit_max, response_status);

  match result {
    Ok(res) => {
      return Ok(block_on(res.text()).unwrap());
    }

    Err(_) => return Err("Invalid response".into()),
  }
}

pub fn repo_info(username: &str, repo_name: &str, jconfig: &config_file_rw::JSONConfig) {
  let url = format!("https://api.github.com/repos/{username}/{repo_name}");

  let res = fetch_repo_info(&url, &jconfig);
  // println!("{}", res.unwrap());
}
