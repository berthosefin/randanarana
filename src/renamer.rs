use rand::Rng;
use std::borrow::Cow;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::names::NameGenerator;

pub const PREVIEW_MAX: usize = 20;

/// An item planned for renaming.
pub struct Item {
    pub path: PathBuf,
    pub new_name: String,
}

/// Result of planning: the items to rename and how many were skipped.
pub struct Plan {
    pub items: Vec<Item>,
    pub skipped: usize,
}

/// Collect the files (and optionally directories) to rename under `target`.
///
/// Returns `(files, dirs)`:
/// - `files`: non-hidden files, sorted alphabetically; recursed into
///   subdirectories when `recursive` is true.
/// - `dirs`: non-hidden subdirectories, deepest first (bottom-up), only when
///   `rename_dirs` is true.
pub fn discover(
    target: &Path,
    recursive: bool,
    rename_dirs: bool,
) -> io::Result<(Vec<PathBuf>, Vec<PathBuf>)> {
    let mut files = Vec::new();
    collect_files(target, recursive, &mut files)?;
    files.sort();

    let mut dirs = Vec::new();
    if rename_dirs {
        collect_dirs(target, &mut dirs)?;
        dirs.sort_by(|a, b| {
            b.components()
                .count()
                .cmp(&a.components().count())
                .then_with(|| a.cmp(b))
        });
    }
    Ok((files, dirs))
}

/// Recursively collect non-hidden files (skips hidden directories when not recursive).
fn collect_files(dir: &Path, recursive: bool, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        let file_type = entry.file_type()?;
        let path = entry.path();
        if file_type.is_dir() {
            if recursive {
                collect_files(&path, recursive, out)?;
            }
        } else if file_type.is_file() {
            out.push(path);
        }
    }
    Ok(())
}

/// Recursively collect non-hidden directories, deepest first.
fn collect_dirs(dir: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        if entry.file_type()?.is_dir() {
            let path = entry.path();
            collect_dirs(&path, out)?;
            out.push(path);
        }
    }
    Ok(())
}

/// Build the rename plan: reserve original names, partition items into
/// "to rename" vs "already-random (skip)", and generate the new names.
pub fn plan<R: Rng>(
    generator: &mut NameGenerator<R>,
    files: &[PathBuf],
    dirs: &[PathBuf],
    force: bool,
) -> Plan {
    for path in files.iter().chain(dirs.iter()) {
        generator.reserve(basename(path));
    }

    let mut items = Vec::new();
    let mut skipped = 0;

    for path in files {
        let name = basename(path);
        if !force && generator.matches_pattern(name, false) {
            skipped += 1;
            continue;
        }
        let new_name = generator.generate(name, false);
        items.push(Item {
            path: path.clone(),
            new_name,
        });
    }

    for path in dirs {
        let name = basename(path);
        if !force && generator.matches_pattern(name, true) {
            skipped += 1;
            continue;
        }
        let new_name = generator.generate(name, true);
        items.push(Item {
            path: path.clone(),
            new_name,
        });
    }

    Plan { items, skipped }
}

/// A path displayed relative to the target directory.
pub fn display_path<'a>(path: &'a Path, target: &Path) -> Cow<'a, str> {
    path.strip_prefix(target).unwrap_or(path).to_string_lossy()
}

/// Print the preview: target, the renames to perform, truncating after
/// `PREVIEW_MAX` lines, plus a note about skipped items.
pub fn print_preview(items: &[Item], skipped: usize, target: &Path) {
    println!("Target: {}", target.display());
    println!("Items to rename ({}):", items.len());
    let mut shown = 0usize;
    let mut hidden = 0usize;
    for item in items {
        let new_path = item
            .path
            .parent()
            .unwrap_or(Path::new(""))
            .join(&item.new_name);
        let line = format!(
            "  {} -> {}",
            display_path(&item.path, target),
            display_path(&new_path, target)
        );
        if shown < PREVIEW_MAX {
            println!("{line}");
            shown += 1;
        } else {
            hidden += 1;
        }
    }
    if hidden > 0 {
        println!("  ... and {hidden} more (preview truncated, showing {PREVIEW_MAX})");
    }
    if skipped > 0 {
        println!("{skipped} already-random item(s) skipped (use --force to rename them).");
    }
}

fn basename(path: &Path) -> &str {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
}

/// Rename a single item to its new name. Prints an error on failure.
pub fn rename_one(item: &Item, target: &Path) -> bool {
    let new_path = item
        .path
        .parent()
        .unwrap_or(Path::new(""))
        .join(&item.new_name);
    match fs::rename(&item.path, &new_path) {
        Ok(()) => true,
        Err(e) => {
            eprintln!(
                "  Error: could not rename {}: {e}",
                display_path(&item.path, target)
            );
            false
        }
    }
}

/// Print the final summary line.
pub fn print_summary(renamed: usize, skipped: usize, failed: usize) {
    println!();
    println!("Done: {renamed} renamed, {skipped} skipped, {failed} failed.");
}
