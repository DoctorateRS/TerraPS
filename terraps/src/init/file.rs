use std::{fs::create_dir_all, path::Path};

use anyhow::Result;

pub fn mkdir<P: AsRef<Path>>(path: P) -> Result<()> {
    Ok(create_dir_all(path)?)
}
