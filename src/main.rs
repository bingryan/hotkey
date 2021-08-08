mod cmd;
mod config;
mod utils;

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::{env, fs, process};
use clap::{crate_authors, crate_description, crate_version, App, Arg};

fn root_path() -> Result<PathBuf> {
    let mut path = dirs::home_dir().unwrap();
    path.push(".hotkey");
    Ok(path)
}


fn run() -> Result<bool> {
    let root_path = root_path()?;
    let config_file = &root_path.join("config.yml");

    let matches = App::new("hot")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("command")
                .help("command to run.")
                .empty_values(false),
        )
        .get_matches();


    if let Some(command) = matches.value_of("command") {
        match command {
            _ if command == "i" => cmd::init()?,
            _ => {
                // TODO:
                todo!()
            }
        }
    }
    Ok(true)
}

fn main() {
    let result = run();
    match result {
        Err(error) => {
            log::error!("hotkey Error: {}", error);
            process::exit(1);
        }
        Ok(false) => {
            process::exit(1);
        }
        Ok(true) => {
            process::exit(0);
        }
    }
}