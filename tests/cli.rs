use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn randanarana() -> Command {
    Command::cargo_bin("randanarana").unwrap()
}

fn write_files(dir: &Path, names: &[&str]) {
    for name in names {
        fs::write(dir.join(name), b"x").unwrap();
    }
}

fn dir_entries(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|n| !n.starts_with('.'))
        .collect();
    names.sort();
    names
}

#[test]
fn help_prints_usage() {
    randanarana()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("--length"))
        .stdout(predicate::str::contains("--dry-run"));
}

#[test]
fn missing_directory_exits_with_clap_error() {
    randanarana()
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("required arguments"));
}

#[test]
fn path_is_not_a_directory_exits_with_one() {
    randanarana()
        .arg("/dev/null")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("not a directory"));
}

#[test]
fn invalid_length_is_rejected() {
    randanarana()
        .args(["--length", "0", "."])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("invalid length"));
}

#[test]
fn dry_run_does_not_rename() {
    let dir = tempdir().unwrap();
    write_files(dir.path(), &["a.jpg", "b.png"]);

    randanarana()
        .args(["-n", dir.path().to_str().unwrap()])
        .assert()
        .success();

    let names = dir_entries(dir.path());
    assert_eq!(names, ["a.jpg", "b.png"]);
}

#[test]
fn confirm_renames_and_keeps_extensions() {
    let dir = tempdir().unwrap();
    write_files(dir.path(), &["a.jpg", "b.png", "c.txt"]);

    randanarana()
        .args([dir.path().to_str().unwrap()])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Done: 3 renamed, 0 skipped, 0 failed.",
        ));

    assert!(!dir.path().join("a.jpg").exists());
    let names = dir_entries(dir.path());
    assert_eq!(names.len(), 3);
    for name in &names {
        assert!(
            name.ends_with(".jpg") || name.ends_with(".png") || name.ends_with(".txt"),
            "unexpected name {name}"
        );
    }
}

#[test]
fn declined_confirmation_changes_nothing() {
    let dir = tempdir().unwrap();
    write_files(dir.path(), &["a.jpg", "b.png"]);

    randanarana()
        .args([dir.path().to_str().unwrap()])
        .write_stdin("n\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Cancelled."));

    let names = dir_entries(dir.path());
    assert_eq!(names, ["a.jpg", "b.png"]);
}

#[test]
fn already_random_files_are_skipped() {
    let dir = tempdir().unwrap();
    write_files(dir.path(), &["Ab3x9Qpz.jpg", "plain.txt"]);

    randanarana()
        .args(["-n", dir.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "1 already-random item(s) skipped (use --force to rename them).",
        ))
        .stdout(predicate::str::contains("Items to rename (1):"));
}

#[test]
fn force_renames_already_random_items() {
    let dir = tempdir().unwrap();
    write_files(dir.path(), &["Ab3x9Qpz.jpg", "plain.txt"]);

    randanarana()
        .args(["-f", dir.path().to_str().unwrap()])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Done: 2 renamed, 0 skipped, 0 failed.",
        ));
}

#[test]
fn recursive_renames_files_but_keeps_subdirectories() {
    let dir = tempdir().unwrap();
    fs::create_dir(dir.path().join("sub")).unwrap();
    write_files(dir.path(), &["a.txt"]);
    write_files(&dir.path().join("sub"), &["deep.txt"]);

    randanarana()
        .args(["-r", dir.path().to_str().unwrap()])
        .write_stdin("y\n")
        .assert()
        .success();

    assert!(dir.path().join("sub").is_dir());
    let inner = dir_entries(&dir.path().join("sub"));
    assert_eq!(inner.len(), 1);
    assert!(inner[0].ends_with(".txt"));
}

#[test]
fn dirs_mode_renames_directories_bottom_up() {
    let dir = tempdir().unwrap();
    fs::create_dir(dir.path().join("sub")).unwrap();
    write_files(&dir.path().join("sub"), &["deep.txt"]);

    randanarana()
        .args(["-D", dir.path().to_str().unwrap()])
        .write_stdin("y\n")
        .assert()
        .success();

    assert!(!dir.path().join("sub").exists());
    let dirs: Vec<String> = fs::read_dir(dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().unwrap().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(dirs.len(), 1);
    assert!(!dirs[0].contains('.'));
}

#[test]
fn hidden_files_are_ignored() {
    let dir = tempdir().unwrap();
    write_files(dir.path(), &[".hidden", "visible.txt"]);

    randanarana()
        .args([dir.path().to_str().unwrap()])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Done: 1 renamed, 0 skipped, 0 failed.",
        ));

    assert!(dir.path().join(".hidden").exists());
}

#[test]
fn interactive_answers_are_honored() {
    let dir = tempdir().unwrap();
    write_files(dir.path(), &["a.txt", "b.txt", "c.txt"]);

    randanarana()
        .args(["-i", dir.path().to_str().unwrap()])
        .write_stdin("y\nn\na\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Done: 2 renamed, 1 skipped, 0 failed.",
        ));

    let names = dir_entries(dir.path());
    assert_eq!(names.len(), 3);
    assert!(
        names.iter().any(|n| n == "b.txt"),
        "b.txt should be skipped"
    );
}

#[test]
fn interactive_quit_stops_and_reports_skips() {
    let dir = tempdir().unwrap();
    write_files(dir.path(), &["a.txt", "b.txt", "c.txt"]);

    randanarana()
        .args(["-i", dir.path().to_str().unwrap()])
        .write_stdin("y\nq\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Done: 1 renamed, 2 skipped, 0 failed.",
        ));
}

#[test]
fn interrupted_input_exits_with_130() {
    let dir = tempdir().unwrap();
    write_files(dir.path(), &["a.txt"]);

    randanarana()
        .args(["-i", dir.path().to_str().unwrap()])
        .write_stdin("")
        .assert()
        .code(130)
        .stdout(predicate::str::contains(
            "Done: 0 renamed, 1 skipped, 0 failed.",
        ));
}

#[test]
fn preview_is_truncated_after_twenty_items() {
    let dir = tempdir().unwrap();
    let mut names: Vec<String> = (0..25).map(|i| format!("f{i:02}.txt")).collect();
    let refs: Vec<&str> = names.iter().map(String::as_str).collect();
    write_files(dir.path(), &refs);
    names.clear();

    randanarana()
        .args(["-n", dir.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Items to rename (25):"))
        .stdout(predicate::str::contains(
            "... and 5 more (preview truncated, showing 20)",
        ));
}

#[test]
fn empty_directory_reports_no_items() {
    let dir = tempdir().unwrap();

    randanarana()
        .args([dir.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("No items to rename."));
}

fn only_file(dir: &Path) -> String {
    let entries = dir_entries(dir);
    assert_eq!(entries.len(), 1, "expected exactly one visible file");
    entries[0].clone()
}

#[test]
fn undo_restores_previous_rename() {
    let dir = tempdir().unwrap();
    write_files(dir.path(), &["a.txt"]);

    randanarana()
        .args([dir.path().to_str().unwrap()])
        .write_stdin("y\n")
        .assert()
        .success();
    let new_name = only_file(dir.path());
    assert_ne!(new_name, "a.txt");

    randanarana()
        .args(["undo", dir.path().to_str().unwrap()])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Done: 1 restored, 0 skipped, 0 failed.",
        ));

    assert_eq!(only_file(dir.path()), "a.txt");
    assert!(!dir.path().join(".randanarana-undo.json").exists());
}

#[test]
fn undo_without_manifest_reports_nothing() {
    let dir = tempdir().unwrap();
    write_files(dir.path(), &["a.txt"]);

    randanarana()
        .args(["undo", dir.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("No renames to undo."));
}

#[test]
fn undo_with_corrupt_manifest_fails() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join(".randanarana-undo.json"),
        b"{ not valid json",
    )
    .unwrap();

    randanarana()
        .args(["undo", dir.path().to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("could not read undo manifest in"));
}

#[test]
fn undo_dry_run_does_not_modify_files() {
    let dir = tempdir().unwrap();
    write_files(dir.path(), &["a.txt"]);

    randanarana()
        .args([dir.path().to_str().unwrap()])
        .write_stdin("y\n")
        .assert()
        .success();
    let new_name = only_file(dir.path());

    randanarana()
        .args(["undo", "-n", dir.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("1 item to restore:"));

    assert_eq!(only_file(dir.path()), new_name);
    assert!(dir.path().join(".randanarana-undo.json").exists());
}

#[test]
fn undo_declined_leaves_files_untouched() {
    let dir = tempdir().unwrap();
    write_files(dir.path(), &["a.txt"]);

    randanarana()
        .args([dir.path().to_str().unwrap()])
        .write_stdin("y\n")
        .assert()
        .success();
    let new_name = only_file(dir.path());

    randanarana()
        .args(["undo", dir.path().to_str().unwrap()])
        .write_stdin("n\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Cancelled."));

    assert_eq!(only_file(dir.path()), new_name);
    assert!(dir.path().join(".randanarana-undo.json").exists());
}

#[test]
fn undo_conflict_skips_and_keeps_manifest() {
    let dir = tempdir().unwrap();
    write_files(dir.path(), &["a.txt"]);

    randanarana()
        .args([dir.path().to_str().unwrap()])
        .write_stdin("y\n")
        .assert()
        .success();
    let new_name = only_file(dir.path());

    fs::write(dir.path().join("a.txt"), b"new").unwrap();

    randanarana()
        .args(["undo", dir.path().to_str().unwrap()])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Done: 0 restored, 0 skipped, 1 failed.",
        ));

    let mut names = dir_entries(dir.path());
    names.sort();
    assert!(
        names.contains(&new_name),
        "renamed file must still be present"
    );
    assert!(
        names.contains(&"a.txt".to_string()),
        "blocking file must remain"
    );
    assert_eq!(names.len(), 2);

    assert!(dir.path().join(".randanarana-undo.json").exists());
}

#[test]
fn undo_restores_recursive_renames() {
    let dir = tempdir().unwrap();
    write_files(dir.path(), &["a.txt"]);
    let sub = dir.path().join("sub");
    fs::create_dir(&sub).unwrap();
    write_files(&sub, &["deep.txt"]);

    randanarana()
        .args(["-r", dir.path().to_str().unwrap()])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Done: 2 renamed, 0 skipped, 0 failed.",
        ));

    assert!(!dir.path().join("sub/deep.txt").exists());

    randanarana()
        .args(["undo", dir.path().to_str().unwrap()])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Done: 2 restored, 0 skipped, 0 failed.",
        ));

    assert!(dir.path().join("a.txt").exists());
    assert!(dir.path().join("sub/deep.txt").exists());
    assert!(!dir.path().join(".randanarana-undo.json").exists());
}

#[test]
fn rename_overwrites_previous_manifest() {
    let dir = tempdir().unwrap();
    write_files(dir.path(), &["a.txt", "b.txt"]);

    randanarana()
        .args([dir.path().to_str().unwrap()])
        .write_stdin("y\n")
        .assert()
        .success();
    let mut first: Vec<String> = dir_entries(dir.path());
    assert_eq!(first.len(), 2);
    first.sort();

    randanarana()
        .args(["-f", dir.path().to_str().unwrap()])
        .write_stdin("y\n")
        .assert()
        .success();
    let mut second: Vec<String> = dir_entries(dir.path());
    assert_eq!(second.len(), 2);
    second.sort();
    assert_ne!(first, second, "second run should rename again");

    randanarana()
        .args(["undo", dir.path().to_str().unwrap()])
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Done: 2 restored, 0 skipped, 0 failed.",
        ));

    assert_eq!(dir_entries(dir.path()).len(), 2);
    let mut restored: Vec<String> = dir_entries(dir.path());
    restored.sort();
    assert_eq!(
        restored, first,
        "undo should restore names from the last run"
    );
}
