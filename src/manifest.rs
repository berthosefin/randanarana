use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

pub const MANIFEST_NAME: &str = ".randanarana-undo.json";

/// The undo manifest recording every rename performed by a run.
#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub version: u32,
    pub tool_version: String,
    pub created_at: u64,
    pub renamed: Vec<RenameEntry>,
}

/// One performed rename, as paths relative to the target directory.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RenameEntry {
    pub from: String,
    pub to: String,
}

impl RenameEntry {
    pub fn from_paths(from: &Path, to: &Path, target: &Path) -> Self {
        RenameEntry {
            from: relative_string(from, target),
            to: relative_string(to, target),
        }
    }
}

fn relative_string(path: &Path, target: &Path) -> String {
    path.strip_prefix(target)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned()
}

impl Manifest {
    /// Build a manifest for the current run and tool version.
    pub fn current(renamed: Vec<RenameEntry>) -> Self {
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Manifest {
            version: 1,
            tool_version: env!("CARGO_PKG_VERSION").to_string(),
            created_at,
            renamed,
        }
    }

    /// Write the manifest atomically (temp file + rename) into `target`.
    pub fn write(&self, target: &Path) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self).map_err(io::Error::other)?;
        let tmp = target.join(format!("{MANIFEST_NAME}.tmp"));
        fs::write(&tmp, json)?;
        fs::rename(&tmp, target.join(MANIFEST_NAME))
    }

    /// Read the manifest from `target`. Returns `None` when it does not exist
    /// and an error when it exists but cannot be read or parsed.
    pub fn read(target: &Path) -> Result<Option<Manifest>> {
        let path = target.join(MANIFEST_NAME);
        if !path.is_file() {
            return Ok(None);
        }
        let json = fs::read_to_string(&path)?;
        let manifest: Manifest = serde_json::from_str(&json)?;
        Ok(Some(manifest))
    }

    /// Remove the manifest, ignoring a missing file.
    pub fn delete(target: &Path) -> io::Result<()> {
        match fs::remove_file(target.join(MANIFEST_NAME)) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn manifest_path(target: &Path) -> std::path::PathBuf {
        target.join(MANIFEST_NAME)
    }

    #[test]
    fn entries_store_relative_paths() {
        let dir = tempdir().unwrap();
        let target = dir.path().join("base");
        std::fs::create_dir(&target).unwrap();
        let entry = RenameEntry::from_paths(
            &target.join("sub/a.txt"),
            &target.join("sub/XyZ9aBc1.txt"),
            &target,
        );
        assert_eq!(entry.from, "sub/a.txt");
        assert_eq!(entry.to, "sub/XyZ9aBc1.txt");
    }

    #[test]
    fn manifest_round_trips() {
        let dir = tempdir().unwrap();
        let entry = RenameEntry {
            from: "sub/a.txt".to_string(),
            to: "sub/XyZ9aBc1.txt".to_string(),
        };
        let manifest = Manifest::current(vec![entry]);
        manifest.write(dir.path()).unwrap();
        let loaded = Manifest::read(dir.path()).unwrap().expect("manifest");
        assert_eq!(loaded.version, 1);
        assert_eq!(loaded.tool_version, env!("CARGO_PKG_VERSION"));
        assert_eq!(loaded.renamed.len(), 1);
        assert_eq!(loaded.renamed[0].from, "sub/a.txt");
        assert_eq!(loaded.renamed[0].to, "sub/XyZ9aBc1.txt");
    }

    #[test]
    fn read_returns_none_when_absent() {
        let dir = tempdir().unwrap();
        assert!(Manifest::read(dir.path()).unwrap().is_none());
    }

    #[test]
    fn read_errors_on_corrupt_json() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join(MANIFEST_NAME), b"not json").unwrap();
        assert!(Manifest::read(dir.path()).is_err());
    }

    #[test]
    fn delete_ignores_missing_file() {
        let dir = tempdir().unwrap();
        Manifest::delete(dir.path()).unwrap();
        assert!(!manifest_path(dir.path()).is_file());
    }

    #[test]
    fn delete_removes_existing_manifest() {
        let dir = tempdir().unwrap();
        Manifest::current(vec![]).write(dir.path()).unwrap();
        assert!(manifest_path(dir.path()).is_file());
        Manifest::delete(dir.path()).unwrap();
        assert!(!manifest_path(dir.path()).exists());
    }
}
