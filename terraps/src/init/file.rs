use std::{
    fs::{create_dir_all, File},
    path::Path,
};

use anyhow::{anyhow, Result};

pub fn mkdir<P: AsRef<Path>>(path: P) -> Result<()> {
    Ok(create_dir_all(path)?)
}

pub fn mkfile<P: AsRef<Path>>(path: P) -> Result<File> {
    let path = path.as_ref();
    let parent = path.parent().ok_or_else(|| anyhow!("No parent"))?;
    mkdir(parent)?;
    Ok(File::create(path)?)
}
