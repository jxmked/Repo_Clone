use futures::executor::block_on;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::Client;

use crate::config_file_rw::JSONConfig;
use crate::constants;


/**
 * These functions are esponsible for fetching the entire
 * info about a defined repository on github.
 * Also, returns an invalid response for no network or a
 * repository does not exists...
 */

// https://api.github.com/repos/{user}/{repo}

fn fetch_repo_info(url: &str, jconfig: &JSONConfig) -> Result<String, String> {
  let initialized_client = Client::new();

  let mut req_builder = initialized_client
    .get(url)
    .header(ACCEPT, "application/vnd.github+json")
    .header(USER_AGENT, constants::USER_AGENT);

  // Insert token header if we have a token available.
  // The problem was we didnt know if the token was valid
  // so we just go ahead at this time.
  if !jconfig.token.is_empty() {
    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", jconfig.token));
  }

  let req_build = req_builder.build();
  let response = Client::execute(&initialized_client, req_build.unwrap());

  let result = block_on(response);

  let head = result.as_ref();
  let refff = head.clone().unwrap();
  let head2 = refff.headers();

  for hh in head2.values() {
    println!("{}", hh.to_str().unwrap())
  }

  let yy = head2.get("x-ratelimit-limit");
  let yt = yy.unwrap().to_str().unwrap().to_string();

  println!("Rate remaining - {}", yt);

  match result {
    Ok(res) => {
      return Ok(block_on(res.text()).unwrap());
      // match block_on(res.text()) {
      //   Ok(t) => return Ok(t),

      //   Err(_) => return Err("Invalid response".into()),
      // }
    }

    Err(_) => return Err("Invalid response".into()),
  }
}

pub fn repo_info(username: &str, repo_name: &str, jconfig: &JSONConfig) {
  let url = format!("https://api.github.com/repos/{username}/{repo_name}");

  let res = fetch_repo_info(&url, &jconfig);
  // println!("{}", res.unwrap());
}
