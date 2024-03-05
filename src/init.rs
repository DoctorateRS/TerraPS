use std::{fs::DirBuilder, path::Path};

// Create directories if they doesn't exist
pub fn dir_init() {
    if !Path::new("config/").exists() {
        DirBuilder::new()
            .recursive(true)
            .create("config/")
            .expect("Failed to create config directory");
    }
    if !Path::new("data/").exists() {
        DirBuilder::new()
            .recursive(true)
            .create("data/")
            .expect("Failed to create data directory");
    }
}
