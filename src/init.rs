use reqwest::get;
use std::{
    fs::{write, DirBuilder},
    path::Path,
};

pub async fn init() {
    dir_init();
    json_init().await;
}

// Create directories if they doesn't exist
fn dir_init() {
    if !Path::new("./config/").exists() {
        DirBuilder::new().recursive(true).create("./config/").expect("Failed to create config directory");
    }
    if !Path::new("./data/").exists() {
        DirBuilder::new().recursive(true).create("./data/").expect("Failed to create data directory");
    }
    if !Path::new("./data/announce/").exists() {
        DirBuilder::new().recursive(true).create("./data/announce/").expect("Failed to create announce directory");
    }
    if !Path::new("./data/crisis/").exists() {
        DirBuilder::new().recursive(true).create("./data/crisis/").expect("Failed to create crisis directory");
    }
    if !Path::new("./data/crisisV2/").exists() {
        DirBuilder::new().recursive(true).create("./data/crisisV2/").expect("Failed to create crisisV2 directory");
    }
    if !Path::new("./data/excel/").exists() {
        DirBuilder::new().recursive(true).create("./data/excel/").expect("Failed to create excel directory");
    }
    if !Path::new("./data/rlv2/").exists() {
        DirBuilder::new().recursive(true).create("./data/excel/").expect("Failed to create rlv2 directory");
    }
    if !Path::new("./data/tower/").exists() {
        DirBuilder::new().recursive(true).create("./data/tower/").expect("Failed to create tower directory");
    }
    if !Path::new("./data/user/").exists() {
        DirBuilder::new().recursive(true).create("./data/user/").expect("Failed to create raid directory");
    }
}

async fn json_init() {
    if !Path::new("./config/config.json").exists() {
        let config = get("https://raw.githubusercontent.com/DoctorateRS/public-data/main/config/config.json")
            .await
            .expect("Failed to get config.json")
            .text()
            .await
            .expect("Failed to get config.json");

        write("./config2/config.json", config).expect("Failed to write config.json");
    }
    if !Path::new("./data/crisisv2/cc1.json").exists() {
        let content = get("https://raw.githubusercontent.com/DoctorateRS/public-data/main/crisisv2/cc1.json")
            .await
            .expect("Failed to get config.json")
            .text()
            .await
            .expect("Failed to get config.json");

        write("./data/crisisv2/cc1.json", content).expect("Failed to write cc1.json");
    }
}
