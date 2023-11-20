use glob::glob;
use log::{error, info};
use std::{
    fs::{self, File},
    path::Path,
};

pub(crate) struct SyncPath {
    pub path: String,
    pub is_file: bool,
    pub is_dir: bool,
}

impl SyncPath {
    pub fn new(path: &Path) -> Self {
        let is_file = Path::is_file(path);
        let is_dir = Path::is_dir(path);
        Self {
            path: path.to_str().as_deref().unwrap().to_string(),
            is_file,
            is_dir,
        }
    }

    pub fn copy_to(&self, dest: &str) {
        if self.is_file {
            create_file(&self.path);
        } else if self.is_dir {
            create_dir(&self.path);
        } else {
            error!("Path '{}' is not a file or directory!", self.path);
        }
    }

    pub fn remove(&self) {
        if self.is_file {
            remove_file(&self.path);
        } else if self.is_dir {
            remove_dir(&self.path);
        } else {
            error!("Path '{}' is not a file or directory!", self.path);
        }
    }
}

pub(crate) fn create_dir(path: &str) {
    match fs::create_dir(path) {
        Ok(_) => info!("Folder '{path}' created successfully!"),
        Err(e) => error!("Error creating folder '{}': {:?}", path, e),
    }
}

pub(crate) fn remove_dir(path: &str) {
    match fs::remove_dir_all(path) {
        Ok(_) => info!("Folder '{path}' and its contents removed successfully!"),
        Err(e) => error!("Error removing folder '{}': {:?}", path, e),
    }
}

pub(crate) fn create_file(path: &str) {
    match File::create(path) {
        Ok(_) => info!("File '{path}' created successfully!"),
        Err(e) => error!("Error creating file '{}: {:?}", path, e),
    }
}

pub(crate) fn remove_file(path: &str) {
    match fs::remove_file(path) {
        Ok(_) => info!("File '{path}' removed successfully!"),
        Err(e) => error!("Error removing file '{}: {:?}", path, e),
    }
}
