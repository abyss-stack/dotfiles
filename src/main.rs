#![allow(clippy::uninlined_format_args)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

mod cli;
mod core;
mod outcome;

use std::process::ExitCode;
//use std::path::Path;

use clap::Parser;

use outcome::{/*AppError, AppMessage, */AppResult};
use cli::{AppArgs, Commands};
use core::config::Config;

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

            println!("{:?}", config);
        },
    }
    
    Ok(())
}

