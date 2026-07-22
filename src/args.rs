use clap::{
    Parser,
    Subcommand
};
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
}

#[derive(Parser)]
#[command(version)]
pub struct AppArgs {
    #[arg(long)]
    source: PathBuf,

    #[arg(long)]
    target: Option<PathBuf>,
    
    #[command(subcommand)]
    pub command: Commands,
}
