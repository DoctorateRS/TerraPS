use std::{fs::DirBuilder, path::Path};

// Create directories if they doesn't exist
pub fn dir_init() {
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
}
