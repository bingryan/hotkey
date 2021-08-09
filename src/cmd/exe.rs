use crate::root_path;
use anyhow::{Error, Result};
use cmd_lib::*;
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use serde_json::Result as SerResult;
use std::fs::File;
use std::io::BufReader;
use std::process::{exit, Command, Stdio};

pub fn execute_command(sub_command: &str, arg: &str) -> Result<()> {
	let root_path = root_path()?;

	let sub_command_json = &root_path.join(format!("commands/{}.json", sub_command));

	if !sub_command_json.exists() {
		return Err(Error::msg(format!(
			"`{}` is  an empty file. try to: `vim {} and add content`",
			sub_command_json.to_string_lossy().to_string(),
			sub_command_json.to_string_lossy().to_string()
		)));
	}

	let file = File::open(sub_command_json)?;
	let reader = BufReader::new(file);

	let command_json: serde_json::Value = serde_json::from_reader(reader)?;

	let pb = ProgressBar::new_spinner();

	let template = format!(" {} {} command: {}", "{spinner:.blue}", sub_command, "{wide_msg}");

	pb.enable_steady_tick(200);
	pb.set_style(
		ProgressStyle::default_spinner()
			.tick_chars("/|\\- ")
			.template(template.as_str()),
	);

	for commands in command_json[arg].as_array() {
		for command in commands {
			let mut cs = command.as_str().unwrap_or("").trim().split_whitespace();
			let first_args = cs.next().unwrap_or("sh");
			let mut comm = Command::new(first_args);

			let args: Vec<&str> = cs.collect();

			let output = comm
				.args(&args)
				.stdout(Stdio::inherit())
				.stderr(Stdio::inherit())
				.stdin(Stdio::inherit())
				.output()
				.unwrap_or_else(|e| {
					log::error!("Hotkey Error: {}", e);
					exit(-5);
				});
			if !output.status.success() {
				exit(output.status.code().unwrap_or(1))
			}
		}
	}

	pb.finish_and_clear();

	Ok(())
}
