use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
    ConfigTemplate,
    Apply,
}

#[derive(Parser)]
#[command(version = env!("VERSION"))]
pub struct AppArgs {
    #[arg(long, default_value = "/etc/abyss-dotfiles/config.json")]
    pub config: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}
