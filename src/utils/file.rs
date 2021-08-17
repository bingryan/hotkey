use anyhow::Result;
use std::fs;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

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
			Err(_e) => Ok(true),
		};
		return is_entry;
	}
	Ok(false)
}

// copy all child dir(recursively) to target directory
// O: origin directory
// T: target directory
pub fn copy<O: AsRef<Path>, T: AsRef<Path>>(from: O, to: T) -> Result<()> {
	let mut stack = Vec::new();
	stack.push(PathBuf::from(from.as_ref()));

	let output_root = PathBuf::from(to.as_ref());
	let input_root = PathBuf::from(from.as_ref()).components().count();

	while let Some(working_path) = stack.pop() {
		let src: PathBuf = working_path.components().skip(input_root).collect();

		let dest = if src.components().count() == 0 {
			output_root.clone()
		} else {
			output_root.join(&src)
		};
		if fs::metadata(&dest).is_err() {
			fs::create_dir_all(&dest)?;
		}

		for entry in fs::read_dir(working_path)? {
			let entry = entry?;
			let path = entry.path();
			if path.is_dir() {
				stack.push(path);
			} else {
				match path.file_name() {
					Some(filename) => {
						let dest_path = dest.join(filename);
						fs::copy(&path, &dest_path)?;
					}
					None => {
						log::error!("Hotkey Error: failed with {:?}", path);
					}
				}
			}
		}
	}

	Ok(())
}
