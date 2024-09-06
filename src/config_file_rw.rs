use crate::util;
use serde_json::{Result, Value};
use std::env;

const __config_file__: &str = "config.json";

fn __get_file_path__(file_path: &str) -> String {
    let mut dir = env::current_exe().unwrap();
    dir.pop();
    dir.push(file_path);
    return dir.to_str().unwrap().to_string();
}

pub fn read_json_file() {
    let file_path: String = __get_file_path__(__config_file__);

    let contents = util::read_file(&file_path);
    // serde_json::from_str(&contents)
}

pub fn write_json_file(json_value: &Value) -> Result<()> {
    let json_string = serde_json::to_string_pretty(json_value)?;

    util::write_file(__config_file__, &json_string);
    Ok(())
}
