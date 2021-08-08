use anyhow::{Error, Result};
use console::{style, Emoji};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::config::{LOOKING_GLASS, SPARKLE, WORKDIRS};
use crate::root_path;
use crate::utils::file::{create_file, is_empty_directory};
use std::fs::create_dir_all;

const CONFIG: &str = r#"[hotkey]
secret_key = "%SECRET_KEY%"
command_json = "commands"
"#;

pub fn init() -> Result<()> {
	let started = Instant::now();

	let step_size = 2;

	println!(
		"{} {}Create hotkey files...",
		style(format!("[1/{}]", step_size)).bold().dim(),
		LOOKING_GLASS
	);

	let path = root_path()?;

	let config = CONFIG.trim_start();

	create_file(&path.join("config.yml"), &config)?;

	println!(
		"{} {}Create Working Path...",
		style(format!("[2/{}]", step_size)).bold().dim(),
		LOOKING_GLASS
	);

	for work_path in WORKDIRS {
		create_dir_all(&path.join(work_path))?;
	}

	println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
	Ok(())
}
