use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version)]
pub struct AppArgs {
    /// Список пакетов для линковки (например: nvim helix zsh)
    #[arg(required = true)]
    pub packages: Vec<String>,

    /// Директория с дотфайлами (если не задана, берется текущая папка)
    #[arg(short, long)]
    pub source: Option<PathBuf>,

    /// Целевая директория (если не задана, берется $HOME)
    #[arg(short, long)]
    pub target: Option<PathBuf>,
}
