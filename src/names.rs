use rand::Rng;
use rand::rng;
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;
use std::collections::HashSet;
use std::path::Path;

const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// Number of distinct bodies a name can take for the given length, or `None`
/// when that number overflows `u64` (length >= 11), meaning no practical limit.
pub fn max_names(length: usize) -> Option<u64> {
    62u64.checked_pow(length as u32)
}

pub fn random_body(length: usize, rng: &mut impl Rng) -> String {
    (0..length)
        .map(|_| *CHARSET.choose(rng).expect("charset is not empty") as char)
        .collect()
}

/// Extension of a file name, including the leading dot, e.g. "photo.jpg" -> Some(".jpg").
/// Returns `None` for names without an extension (including directories and dotfiles).
pub fn extension_with_dot(file_name: &str) -> Option<String> {
    let ext = Path::new(file_name).extension()?;
    let ext = ext.to_str().expect("extension is not valid UTF-8");
    Some(format!(".{ext}"))
}

/// The "bare" part of a name, i.e. without the last extension (for files only).
/// Directories are never stripped, even if their name contains a dot.
fn bare_name(file_name: &str, is_dir: bool) -> Option<&str> {
    if is_dir {
        Some(file_name)
    } else {
        Path::new(file_name).file_stem()?.to_str()
    }
}

/// Assemble the final name: prefix + body + suffix [+ extension].
pub fn assemble_name(prefix: &str, body: &str, suffix: &str, ext: Option<&str>) -> String {
    let mut name = format!("{prefix}{body}{suffix}");
    if let Some(ext) = ext {
        name.push_str(ext);
    }
    name
}

/// Generates unique random names, remembering existing names so that
/// a generated name never collides with them.
pub struct NameGenerator<R: Rng> {
    prefix: String,
    suffix: String,
    length: usize,
    rng: R,
    used: HashSet<String>,
}

impl<R: Rng> NameGenerator<R> {
    pub fn new(prefix: String, suffix: String, length: usize, rng: R) -> Self {
        Self {
            prefix,
            suffix,
            length,
            rng,
            used: HashSet::new(),
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    /// Remember a name that must never be generated again (e.g. an existing item).
    pub fn reserve(&mut self, name: &str) {
        self.used.insert(name.to_string());
    }

    /// Generate a unique random name for the given file name / directory.
    pub fn generate(&mut self, file_name: &str, is_dir: bool) -> String {
        let ext = if is_dir {
            None
        } else {
            extension_with_dot(file_name)
        };
        loop {
            let body = random_body(self.length, &mut self.rng);
            let candidate = assemble_name(&self.prefix, &body, &self.suffix, ext.as_deref());
            if self.used.insert(candidate.clone()) {
                return candidate;
            }
        }
    }

    /// True if `file_name` already matches the random pattern of this generator
    /// (prefix + alnum(LENGTH) + suffix [+ extension]).
    pub fn matches_pattern(&self, file_name: &str, is_dir: bool) -> bool {
        let Some(bare) = bare_name(file_name, is_dir) else {
            return false;
        };
        let Some(bare) = bare.strip_prefix(&self.prefix) else {
            return false;
        };
        let Some(bare) = bare.strip_suffix(&self.suffix) else {
            return false;
        };
        bare.len() == self.length && bare.chars().all(|c| c.is_ascii_alphanumeric())
    }
}

/// Convenience constructor using the thread-local RNG.
impl NameGenerator<ThreadRng> {
    pub fn new_thread(prefix: String, suffix: String, length: usize) -> Self {
        Self::new(prefix, suffix, length, rng())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn body_has_expected_length() {
        let mut rng = rng();
        assert_eq!(random_body(8, &mut rng).len(), 8);
    }

    #[test]
    fn body_uses_only_charset_characters() {
        let mut rng = rng();
        let body = random_body(32, &mut rng);
        assert!(body.chars().all(|c| CHARSET.contains(&(c as u8))));
    }

    #[test]
    fn max_names_exact_values() {
        assert_eq!(max_names(1), Some(62));
        assert_eq!(max_names(2), Some(3844));
        assert_eq!(max_names(3), Some(238_328));
    }

    #[test]
    fn max_names_overflows_for_large_lengths() {
        assert_eq!(max_names(10), Some(62u64.pow(10)));
        assert_eq!(max_names(11), None);
    }

    #[test]
    fn max_names_grows_with_length() {
        let mut previous = 0u64;
        for length in 1..10 {
            let current = max_names(length).unwrap();
            assert!(current > previous, "not monotonic at length {length}");
            previous = current;
        }
    }

    #[test]
    fn extension_keeps_dot_and_case() {
        assert_eq!(extension_with_dot("photo.jpg").as_deref(), Some(".jpg"));
        assert_eq!(extension_with_dot("archive.tar.gz").as_deref(), Some(".gz"));
        assert_eq!(extension_with_dot("photo.JPG").as_deref(), Some(".JPG"));
    }

    #[test]
    fn no_extension_for_dir_or_plain_name() {
        assert_eq!(extension_with_dot("photos"), None);
        assert_eq!(extension_with_dot("my_dir"), None);
        assert_eq!(extension_with_dot(".bashrc"), None);
    }

    #[test]
    fn assemble_prefix_suffix_and_extension() {
        let name = assemble_name("img_", "Ab3x9Qpz", "_2026", Some(".jpg"));
        assert_eq!(name, "img_Ab3x9Qpz_2026.jpg");
    }

    #[test]
    fn assemble_directory_without_extension() {
        let name = assemble_name("", "Ab3x9Qpz", "", None);
        assert_eq!(name, "Ab3x9Qpz");
    }

    #[test]
    fn generated_names_are_unique() {
        let mut generator =
            NameGenerator::new(String::new(), String::new(), 2, StdRng::seed_from_u64(42));
        let mut seen = HashSet::new();
        for _ in 0..500 {
            let name = generator.generate("a.jpg", false);
            assert!(seen.insert(name), "generated a duplicate name");
        }
    }

    #[test]
    fn reserved_names_are_never_generated() {
        let mut generator =
            NameGenerator::new(String::new(), String::new(), 2, StdRng::seed_from_u64(7));
        generator.reserve("ab.jpg");
        for _ in 0..500 {
            let name = generator.generate("a.jpg", false);
            assert_ne!(name, "ab.jpg", "generated a reserved name");
        }
    }

    #[test]
    fn directories_get_names_without_extension() {
        let mut generator =
            NameGenerator::new(String::new(), String::new(), 8, StdRng::seed_from_u64(1));
        let name = generator.generate("some_dir", true);
        assert!(
            !name.contains('.'),
            "directory name has an extension: {name}"
        );
    }

    #[test]
    fn matches_generated_pattern() {
        let generator = NameGenerator::new(
            "img_".to_string(),
            "_2026".to_string(),
            8,
            StdRng::seed_from_u64(1),
        );
        assert!(generator.matches_pattern("img_Ab3x9Qpz_2026.jpg", false));
        assert!(generator.matches_pattern("img_Ab3x9Qpz_2026", true));
    }

    #[test]
    fn does_not_match_other_names() {
        let generator =
            NameGenerator::new(String::new(), String::new(), 8, StdRng::seed_from_u64(1));
        assert!(!generator.matches_pattern("photo.jpg", false));
        assert!(!generator.matches_pattern("img_photo.jpg", false));
        assert!(!generator.matches_pattern("aB3x9QpZ-extra.jpg", false));
    }

    #[test]
    fn matches_respects_prefix_and_suffix() {
        let generator = NameGenerator::new(
            "pre_".to_string(),
            "_suf".to_string(),
            5,
            StdRng::seed_from_u64(1),
        );
        assert!(generator.matches_pattern("pre_Abc12_suf.jpg", false));
        assert!(!generator.matches_pattern("no_Abc12_suf.jpg", false));
        assert!(!generator.matches_pattern("pre_Abc12.jpg", false));
    }
}
