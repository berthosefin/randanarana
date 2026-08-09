mod cli;
mod names;
mod renamer;

use anyhow::{Context, Result, bail};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() -> Result<()> {
    let args = cli::parse();

    if args.length == 0 {
        bail!("invalid length: {} (must be greater than 0)", args.length);
    }

    let target = &args.target;
    if !target.is_dir() {
        bail!("not a directory: {}", target.display());
    }

    let (files, dirs) = renamer::discover(target, args.recursive || args.dirs, args.dirs)
        .with_context(|| format!("could not read directory {}", target.display()))?;

    let mut generator = names::NameGenerator::new_thread(args.prefix, args.suffix, args.length);
    let plan = renamer::plan(&mut generator, &files, &dirs, args.force);

    if plan.items.is_empty() {
        if plan.skipped > 0 {
            println!(
                "No items to rename ({} already random, use --force to rename them).",
                plan.skipped
            );
        } else {
            println!("No items to rename.");
        }
        return Ok(());
    }

    if args.dry_run {
        renamer::print_preview(&plan.items, plan.skipped, target);
        return Ok(());
    }

    if args.interactive {
        return run_interactive(&plan, target);
    }

    renamer::print_preview(&plan.items, plan.skipped, target);
    println!();
    print!("Rename these {} items? [y/N] ", plan.items.len());
    io::stdout().flush()?;
    let mut line = String::new();
    if io::stdin().read_line(&mut line)? == 0 {
        return Ok(());
    }
    if !is_yes(line.trim()) {
        println!("Cancelled.");
        return Ok(());
    }

    let mut renamed = 0;
    let mut failed = 0;
    for item in &plan.items {
        if renamer::rename_one(item, target) {
            renamed += 1;
        } else {
            failed += 1;
        }
    }
    renamer::print_summary(renamed, plan.skipped, failed);
    Ok(())
}

fn is_yes(answer: &str) -> bool {
    matches!(answer.to_ascii_lowercase().as_str(), "y" | "yes")
}

fn run_interactive(plan: &renamer::Plan, target: &Path) -> Result<()> {
    println!("Target: {}", target.display());
    if plan.items.len() == 1 {
        println!("1 item to rename.");
    } else {
        println!("{} items to rename.", plan.items.len());
    }
    if plan.skipped > 0 {
        println!(
            "{} already-random item(s) skipped (use --force to rename them).",
            plan.skipped
        );
    }
    println!();

    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut renamed = 0usize;
    let mut failed = 0usize;
    let mut all = false;
    let mut k = 0usize;

    for item in &plan.items {
        k += 1;
        let new_path = item
            .path
            .parent()
            .unwrap_or(Path::new(""))
            .join(&item.new_name);
        if !all {
            println!(
                "  [{k}/{}]  {} -> {}",
                plan.items.len(),
                renamer::display_path(&item.path, target),
                renamer::display_path(&new_path, target)
            );
            print!("  Rename? [y/N/a/q] ");
            io::stdout().flush()?;
            let mut line = String::new();
            match input.read_line(&mut line) {
                Ok(0) | Err(_) => interrupted_summary(plan, renamed, failed),
                Ok(_) => {}
            }
            match line.trim().to_ascii_lowercase().as_str() {
                "a" => all = true,
                "q" => break,
                "y" => {}
                _ => continue,
            }
        }
        if renamer::rename_one(item, target) {
            renamed += 1;
        } else {
            failed += 1;
        }
    }

    let skipped = plan.skipped + plan.items.len() - renamed - failed;
    renamer::print_summary(renamed, skipped, failed);
    Ok(())
}

fn interrupted_summary(plan: &renamer::Plan, renamed: usize, failed: usize) -> ! {
    let skipped = plan.skipped + plan.items.len() - renamed - failed;
    renamer::print_summary(renamed, skipped, failed);
    std::process::exit(130);
}
