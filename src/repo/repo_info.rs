use futures::executor::block_on;
use reqwest;
use reqwest::header::USER_AGENT;

// https://api.github.com/repos/{user}/{repo}

fn fetch_repo_info(url: &str) -> String {
  let response = reqwest::Client::new().get(url).header(
    USER_AGENT,
    "Mozilla/5.0 (platform; rv:gecko-version) Gecko/gecko-trail Firefox/firefox-version",
  ).send();

  
  let result = block_on(response);
  let head = result.as_ref();
  let refff = head.clone().unwrap();
  let head2 = refff.headers();

  let yy = head2.get("X-ratelimit-remaining");
  let yt = yy.unwrap().to_str().unwrap().to_string();
  println!("Rate remaining - {}", yt);

  match result {
    Ok(res) => {
      return match block_on(res.text()) {
        Ok(t) => t,

        Err(_) => "asd".to_string(),
      }
    }

    Err(_) => "sadasd".to_string(),
  }
}

pub fn repo_info(username: &str, repo_name: &str) {
  let url = format!("https://api.github.com/repos/{username}/{repo_name}");

  let res = fetch_repo_info(&url);
  println!("{}", res);
}
