use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// Rename the files in a directory with random names (a-zA-Z0-9),
/// keeping the original extension.
#[derive(Parser, Debug)]
#[command(
    name = "randanarana",
    version,
    about = "Rename files to random names, keeping extensions."
)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Cli {
    /// Show the preview only, without renaming
    #[arg(short = 'n', long, global = true)]
    pub dry_run: bool,

    #[command(subcommand)]
    pub command: Option<Command>,

    #[command(flatten)]
    pub rename: Option<RenameArgs>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Rename files in a directory (default)
    Rename(RenameArgs),
    /// Restore the last rename run
    Undo(UndoArgs),
}

#[derive(Args, Debug)]
pub struct RenameArgs {
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

    /// Directory containing the files to rename
    pub target: PathBuf,
}

#[derive(Args, Debug)]
pub struct UndoArgs {
    /// Directory to undo renames in (defaults to the current directory)
    pub target: Option<PathBuf>,
}

/// Parse the command-line arguments (exits with help/error messages as needed).
pub fn parse() -> Cli {
    Cli::parse()
}

/// Print the standard clap "missing target" error and exit with code 2.
/// Used when neither a subcommand nor a bare rename argument is given.
pub fn missing_target() -> ! {
    use clap::CommandFactory;
    use clap::error::ErrorKind;
    Cli::command()
        .error(
            ErrorKind::MissingRequiredArgument,
            "the following required arguments were not provided:\n  <TARGET>",
        )
        .exit()
}
