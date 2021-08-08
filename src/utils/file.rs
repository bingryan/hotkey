use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::{Path};
use anyhow::Result;

pub fn create_file(path: &Path, content: &str) -> Result<()> {
    if let Some(p) = path.parent() {
        create_dir_all(p)?;
    }
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn is_empty_directory(path: &Path) -> Result<bool> {
    if path.is_dir() {
        let is_entry = match path.read_dir() {
            Ok(_entries) => Ok(false),
            Err(_e) => Ok(true)
        };
        return is_entry;
    }
    Ok(false)
}