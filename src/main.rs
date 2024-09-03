use std::fs::{File, OpenOptions};
use serde_json::{Result, Value};

mod dir;

fn main() -> Result<()> {
    // Create a new Dir instance
    let dir = dir::Dir::new("my_path".to_string());

    // Serialize the Dir to a JSON file
    let json_data = serde_json::to_string(&dir)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("dir.json")?;
    file.write_all(json_data.as_bytes())?;

    // Deserialize the JSON file back to a Dir instance
    let file = File::open("dir.json")?;
    let deserialized_dir: dir::Dir = serde_json::from_reader(file)?;

    // Call the method on the deserialized Dir
    deserialized_dir.call_path();

    Ok(())
}