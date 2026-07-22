mod args;
mod outcome;

use crate::outcome::{
    AppMessage,
    AppError,
    AppResult,
};
use crate::args::{
    AppArgs,
    Commands,  
};

use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{}", err);
            ExitCode::FAILURE
        },
    }
}

fn run() -> AppResult<()> {
    let args = AppArgs::parse();

    Ok(())
}
