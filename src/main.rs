#![allow(clippy::uninlined_format_args)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

mod cli;
mod core;
mod outcome;

use std::process::ExitCode;

use clap::Parser;

use cli::{AppArgs, Commands};
use core::config::{Config, Strategy};
use outcome::AppResult;

use crate::core::{copy::apply_copy, symlink::apply_symlink};

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            e.emit();
            ExitCode::FAILURE
        }
    }
}

fn run() -> AppResult<()> {
    let args = AppArgs::parse();

    match args.command {
        Commands::ConfigTemplate => println!("{}", Config::TEMPLATE),
        Commands::Apply => {
            let config = Config::load(&args.config)?;

            for pkg in &config.packages {
                match pkg.strategy {
                    Strategy::Copy => apply_copy(pkg)?,
                    Strategy::Symlink => apply_symlink(pkg)?,
                }
            }
        }
    }

    Ok(())
}
