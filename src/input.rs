use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "kmeans", author, version, about, long_about = None)] // Get info in Cargo.toml
pub struct Args {
    #[arg(short, long, default_value = "2")]
    pub k: usize,

    #[arg(short, long)]
    pub output_file: Option<PathBuf>,

    #[arg(short, long)]
    pub mode: Mode,

    #[arg(short, long, default_value = "true")]
    pub replace_entry: bool,

    #[arg(short, long, default_value = "false")]
    pub random_initial: bool,

    /// Main entry
    pub input_file: Option<PathBuf>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Hash)]
pub enum Mode {
    /// Run in serial
    S,

    /// Run in parallel
    Par,

    /// Run in Tokio
    Tokio,
}
