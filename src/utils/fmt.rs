use anyhow::{Ok, Result};
use std::fs::read_dir;

use super::json::{read_json, write_json};

pub fn excel_fmt() -> Result<()> {
    let dir = read_dir("./data/excel")?;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().unwrap() == "json" {
            let v = read_json(path.to_str().unwrap());
            write_json(path.to_str().unwrap(), v);
        }
    }

    Ok(())
}
