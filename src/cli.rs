use clap::Parser;
use std::path::PathBuf;

/// Rename the files in a directory with random names (a-zA-Z0-9),
/// keeping the original extension.
#[derive(Parser, Debug)]
#[command(
    name = "randanarana",
    version,
    about = "Rename files to random names, keeping extensions."
)]
pub struct Cli {
    /// Length of the random part
    #[arg(short = 'l', long, default_value_t = 8)]
    pub length: usize,

    /// Prefix added to the name (e.g. img_)
    #[arg(short, long, default_value = "")]
    pub prefix: String,

    /// Suffix added to the name (e.g. _2026)
    #[arg(short, long, default_value = "")]
    pub suffix: String,

    /// Also process files in subdirectories (subdirectory names are kept)
    #[arg(short, long)]
    pub recursive: bool,

    /// Also rename subdirectories (implies --recursive)
    #[arg(short = 'D', long)]
    pub dirs: bool,

    /// Confirm each item individually
    #[arg(short, long)]
    pub interactive: bool,

    /// Rename items that already match the random pattern
    #[arg(short, long)]
    pub force: bool,

    /// Show the preview only, without renaming
    #[arg(short = 'n', long)]
    pub dry_run: bool,

    /// Directory containing the files to rename
    pub target: PathBuf,
}

/// Parse the command-line arguments (exits with help/error messages as needed).
pub fn parse() -> Cli {
    Cli::parse()
}
