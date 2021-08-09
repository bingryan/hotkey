#![allow(dead_code)]
#[macro_use]
extern crate shells;

mod cmd;
mod config;
mod utils;

use anyhow::{Result, Error};
use clap::{crate_authors, crate_description, crate_version, App, Arg};
use std::path::{Path, PathBuf};
use std::{env, fs, process};
use std::io::Write;

use yansi::Paint;
use chrono::Local;
use log::{self, LevelFilter};
use env_logger::{fmt::Color, Builder, Env};


fn init_logger() {
    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        let mut style = formatter.style();
        style.set_bold(true);

        let tar = Paint::blue("Hotkey").bold();

        match record.level() {
            log::Level::Error => writeln!(
                formatter,
                "{} {} ({}): {}",
                tar,
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                Paint::red(record.level()).bold(),
                Paint::red(record.args()).wrap()
            ),
            log::Level::Warn => writeln!(
                formatter,
                "{} {} ({}): {}",
                tar,
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                Paint::yellow(record.level()).bold(),
                Paint::yellow(record.args()).wrap()
            ),
            _ => writeln!(
                formatter,
                "{} {} ({}): {}",
                tar,
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                Paint::blue(record.level()).bold(),
                Paint::blue(record.args()).wrap()
            ),
        }
    });

    if let Ok(var) = env::var("RUST_LOG") {
        builder.parse_filters(&var);
    } else {
        // if no RUST_LOG provided, default to logging at the Warn level
        builder.filter(None, LevelFilter::Info);
    }

    builder.init();
}

fn root_path() -> Result<PathBuf> {
    let mut path = dirs::home_dir().unwrap();
    path.push(".hotkey");
    Ok(path)
}

fn run() -> Result<bool> {
    init_logger();
    let mut args_params = std::env::args();
    let _hot_command = args_params.next();
    let sub_command = args_params.next();

    if let Some(command) = sub_command {
        match command {
            _ if command == "i" => cmd::init()?,
            _ => {
                for arg in args_params {
                    cmd::execute_command(command.as_str(), arg.as_str())?;
                }
            }
        }
    }
    Ok(true)
}

fn main() {
    let result = run();
    match result {
        Err(error) => {
            log::error!("Hotkey Error: {}", error);
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
