use std::future::IntoFuture;

use futures::TryFutureExt;
use reqwest;
use serde_json::Value;

// https://api.github.com/repos/{user}/{repo}

fn fetch_branch_info(re_url: String) {
  let response = reqwest::get(re_url);
  
  return response.into_future()

}

pub fn get_main_branch(url: &str) {}
