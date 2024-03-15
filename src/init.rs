use reqwest::get;
use std::{
    fs::{write, DirBuilder},
    path::Path,
};

pub(super) async fn init() {
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
    const JSON_LIST: [&str; 21] = [
        "./config/config.json",
        "./config/squads.json",
        "./data/crisisv2/cc1.json",
        "./data/crisis/cc0.json",
        "./data/crisis/cc1.json",
        "./data/crisis/cc2.json",
        "./data/crisis/cc3.json",
        "./data/crisis/cc4.json",
        "./data/crisis/cc5.json",
        "./data/crisis/cc6.json",
        "./data/crisis/cc7.json",
        "./data/crisis/cc8.json",
        "./data/crisis/cc9.json",
        "./data/crisis/cc10.json",
        "./data/crisis/cc11.json",
        "./data/crisis/cc12.json",
        "./data/crisis/ccbeta.json",
        "./data/user/user.json",
        "./data/announce/announcement.meta.json",
        "./data/announce/preannouncement.meta.json",
        "./data/tower/towerData.json",
    ];
    for json in JSON_LIST.iter() {
        json_init(json).await;
    }
}

async fn json_init(path: &str) {
    if !Path::new(path).exists() {
        let url = path.replace("./", "https://raw.githubusercontent.com/DoctorateRS/public-data/main/");
        let content = get(url)
            .await
            .expect("Failed to get JSON.")
            .text()
            .await
            .expect("Failed to parse JSON.");

        write(path, content).expect("Failed to write file.");
    }
}
