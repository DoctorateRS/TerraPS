use reqwest::get;
use std::{
    fs::{write, DirBuilder},
    path::Path,
};

pub async fn init() {
    dir();
    json().await;
}

// Create directories if they doesn't exist
fn dir() {
    const FOLDERS: [&str; 9] = [
        "./config/",
        "./data/",
        "./data/announce/",
        "./data/crisis/",
        "./data/crisisV2/",
        "./data/excel/",
        "./data/rlv2/",
        "./data/tower/",
        "./data/user/",
    ];
    for folder in FOLDERS.iter() {
        dir_init(folder);
    }
}

fn dir_init(path: &str) {
    if !Path::new(path).exists() {
        match DirBuilder::new().recursive(true).create(path) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create folder: {}", e),
        };
    }
}

async fn json() {
    json_init("./config/config.json").await;
    json_init("./data/crisisv2/cc1.json").await;
}

async fn json_init(path: &str) {
    if !Path::new(path).exists() {
        let url = path.replace('.', "https://raw.githubusercontent.com/DoctorateRS/public-data/main");
        let content = get(url).await.expect("Failed to get JSON.").text().await.expect("Failed to parse JSON.");

        write(path, content).expect("Failed to write file.");
    }
}
