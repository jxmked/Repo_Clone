use crate::util;
use serde_derive::Deserialize;
use serde_json::{Result, Value};
use std::env;

#[derive(Deserialize)]
pub struct JSONConfig {
    pub token: String,
    pub output_path: String,
}

const __CONFIG_FILE__: &str = "config.json";
const __DEFAULT_CONFIG__: &str = r#"{
    "token": "",
    "output_path": ""
}
"#;

fn __get_file_path__(file_path: &str) -> String {
    let mut dir: std::path::PathBuf = env::current_exe().unwrap();
    dir.pop();
    dir.push(file_path);
    return dir.to_str().unwrap().to_string();
}

pub fn read_json_file() -> Result<JSONConfig> {
    let file_path: String = __get_file_path__(__CONFIG_FILE__);

    if !util::file_exists(&file_path) {
        util::write_file(&file_path, __DEFAULT_CONFIG__);
    }

    let contents: String = util::read_file(&file_path);
    
    return serde_json::from_str(&contents);

    // serde_json::from_str(&contents).map_err(|err| {
    //     println!("Error parsing config file.");
    //     println!("Visit https://github.com/jxmked/Repo_Clone/issues");
    //     err
    // })
}

pub fn write_json_file(json_value: &Value) -> Result<()> {
    let json_string: String = serde_json::to_string_pretty(json_value)?;

    util::write_file(__CONFIG_FILE__, &json_string);
    Ok(())
}
