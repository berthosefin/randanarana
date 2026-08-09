use crate::cli;
use crate::manifest::{self, Manifest, RenameEntry};
use anyhow::{Context, Result, bail};
use std::io::{self, Write};
use std::path::Path;

const UNDO_PREVIEW_MAX: usize = 20;

enum Action {
    Restore,
    Skip,
    Failed,
}

/// Run `randanarana undo`.
pub fn run(args: &cli::UndoArgs, dry_run: bool) -> Result<()> {
    let target = args
        .target
        .clone()
        .unwrap_or_else(|| Path::new(".").to_path_buf());
    if !target.is_dir() {
        bail!("not a directory: {}", target.display());
    }

    let manifest = match manifest::Manifest::read(&target) {
        Ok(Some(m)) => m,
        Ok(None) => {
            println!("No renames to undo.");
            return Ok(());
        }
        Err(e) => {
            return Err(e)
                .with_context(|| format!("could not read undo manifest in {}", target.display()));
        }
    };

    if manifest.renamed.is_empty() {
        println!("No renames to undo.");
        return Ok(());
    }

    print_undo_preview(&manifest, &target);
    if dry_run {
        return Ok(());
    }

    println!();
    print!("Restore these {} items? [y/N] ", manifest.renamed.len());
    io::stdout().flush()?;
    let mut line = String::new();
    if io::stdin().read_line(&mut line)? == 0 {
        return Ok(());
    }
    if !crate::is_yes(line.trim()) {
        println!("Cancelled.");
        return Ok(());
    }

    let mut restored = 0usize;
    let mut skipped = 0usize;
    let mut failed = 0usize;
    for entry in &manifest.renamed {
        match classify(entry, &target) {
            Action::Restore => {
                let from = target.join(&entry.from);
                let to = target.join(&entry.to);
                match std::fs::rename(&to, &from) {
                    Ok(()) => restored += 1,
                    Err(e) => {
                        failed += 1;
                        eprintln!("  Error: could not restore {}: {e}", entry.to);
                    }
                }
            }
            Action::Skip => {
                skipped += 1;
                println!(
                    "  Skipped (no longer present): {} -> {}",
                    entry.to, entry.from
                );
            }
            Action::Failed => {
                failed += 1;
                eprintln!("  Error: original name already exists: {}", entry.from);
            }
        }
    }

    if failed == 0 {
        manifest::Manifest::delete(&target)?;
    }
    println!();
    println!("Done: {restored} restored, {skipped} skipped, {failed} failed.");
    Ok(())
}

fn classify(entry: &RenameEntry, target: &Path) -> Action {
    let from = target.join(&entry.from);
    let to = target.join(&entry.to);
    if !to.exists() {
        Action::Skip
    } else if from.exists() {
        Action::Failed
    } else {
        Action::Restore
    }
}

fn print_undo_preview(manifest: &Manifest, target: &Path) {
    println!("Target: {}", target.display());
    if manifest.renamed.len() == 1 {
        println!("1 item to restore:");
    } else {
        println!("{} items to restore:", manifest.renamed.len());
    }
    let mut shown = 0usize;
    let mut hidden = 0usize;
    for entry in &manifest.renamed {
        let line = format!("  {} -> {}", entry.to, entry.from);
        if shown < UNDO_PREVIEW_MAX {
            println!("{line}");
            shown += 1;
        } else {
            hidden += 1;
        }
    }
    if hidden > 0 {
        println!("  ... and {hidden} more (preview truncated, showing {UNDO_PREVIEW_MAX})");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn entry(from: &str, to: &str) -> RenameEntry {
        RenameEntry {
            from: from.to_string(),
            to: to.to_string(),
        }
    }

    #[test]
    fn classify_restores_when_target_present_and_original_free() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("XyZ9aBc1.txt"), b"x").unwrap();
        let e = entry("a.txt", "XyZ9aBc1.txt");
        assert!(matches!(classify(&e, dir.path()), Action::Restore));
    }

    #[test]
    fn classify_skips_when_new_name_is_gone() {
        let dir = tempdir().unwrap();
        let e = entry("a.txt", "XyZ9aBc1.txt");
        assert!(matches!(classify(&e, dir.path()), Action::Skip));
    }

    #[test]
    fn classify_skips_when_already_restored() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("a.txt"), b"x").unwrap();
        let e = entry("a.txt", "XyZ9aBc1.txt");
        assert!(matches!(classify(&e, dir.path()), Action::Skip));
    }

    #[test]
    fn classify_fails_when_original_name_taken() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("a.txt"), b"x").unwrap();
        std::fs::write(dir.path().join("XyZ9aBc1.txt"), b"y").unwrap();
        let e = entry("a.txt", "XyZ9aBc1.txt");
        assert!(matches!(classify(&e, dir.path()), Action::Failed));
    }
}
