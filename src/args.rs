use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version)]
pub struct AppArgs {
    #[arg(required = true)]
    pub packages: Vec<String>,

    #[arg(long)]
    pub source: Option<PathBuf>,

    #[arg(long)]
    pub target: Option<PathBuf>,
}
