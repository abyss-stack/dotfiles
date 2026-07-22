use clap::{
    Parser,
};
use std::path::PathBuf;

/*
#[derive(Subcommand)]
pub enum Commands {
}
*/

#[derive(Parser)]
#[command(version)]
pub struct AppArgs {
    #[arg(required = true)]
    pub packages: Vec<String>,
    
    #[arg(long, default_value = ".")]
    pub source: PathBuf,

    #[arg(long)]
    pub target: Option<PathBuf>,
}
