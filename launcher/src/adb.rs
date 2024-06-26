use std::{
    env::consts::OS,
    fs::File,
    io::{copy, Cursor},
    path::Path,
};

use anyhow::Result;
use reqwest::get;
use zip::ZipArchive;

const BASE_ADB_URL: &str = "https://dl.google.com/android/repository/platform-tools-latest-";

pub enum Os {
    Mac,
    Windows,
    Linux,
}

impl Os {
    pub fn new() -> Self {
        match OS {
            "linux" => Os::Linux,
            "macos" => Os::Mac,
            "windows" => Os::Windows,
            _ => panic!("Unsupported OS"),
        }
    }

    pub async fn install_adb(&self) -> Result<()> {
        async fn download_adb(url: &str) -> Result<()> {
            let tmp_dir = Path::new("./tmp");
            let response = get(url).await?;
            let mut adb_file = File::create(tmp_dir.join("adb.zip"))?;
            let mut cursor = Cursor::new(response.bytes().await?);
            copy(&mut cursor, &mut adb_file)?;
            Ok(())
        }

        fn extract_adb() -> Result<()> {
            let install_dir = Path::new("./platform_tools");
            let mut zipfile = ZipArchive::new(File::open("./tmp/adb.zip")?)?;
            zipfile.extract(install_dir)?;
            Ok(())
        }

        let url = match self {
            Os::Mac => format!("{}darwin.zip", BASE_ADB_URL),
            Os::Windows => format!("{}windows.zip", BASE_ADB_URL),
            Os::Linux => format!("{}linux.zip", BASE_ADB_URL),
        };
        download_adb(&url).await?;
        extract_adb()?;

        Ok(())
    }
}
