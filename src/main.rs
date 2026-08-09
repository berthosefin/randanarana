mod cli;

use anyhow::{bail, Result};

fn main() -> Result<()> {
    let args = cli::parse();

    if args.length == 0 {
        bail!("invalid length: {} (must be greater than 0)", args.length);
    }

    if !args.target.is_dir() {
        bail!("not a directory: {}", args.target.display());
    }

    println!("Arguments parsed successfully:");
    println!("{args:#?}");

    Ok(())
}
