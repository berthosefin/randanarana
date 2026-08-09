# randanarana

Rename files in a directory to random alphanumeric names, keeping their
extensions. A Rust reimplementation of an existing bash script, with tests,
CI, and the same CLI and behavior.

```
$ randanarana ~/Pictures
Target: /home/thos/Pictures
Items to rename (3):
  IMG_2020.jpg -> 7Qkz3YpT.jpg
  photo.png    -> Lw4aVb9x.png
  notes.txt    -> VtS8CwQj.txt

Rename these 3 items? [y/N] y

Done: 3 renamed, 0 skipped, 0 failed.
```

## Install

Requires a recent Rust toolchain (edition 2024):

```
rustup update stable
cargo install --path .
```

Or build from source and use the binary at `target/release/randanarana`:

```
cargo build --release
```

## Usage

```
Usage: randanarana [OPTIONS] <TARGET> [COMMAND]

Commands:
  rename  Rename files in a directory (default)
  undo    Restore the last rename run
  help    Print this message or the help of the given subcommand(s)

Arguments:
  <TARGET>  Directory containing the files to rename

Options:
  -l, --length <LENGTH>  Length of the random part [default: 8]
  -p, --prefix <PREFIX>  Prefix added to the name (e.g. img_) [default: ""]
  -s, --suffix <SUFFIX>  Suffix added to the name (e.g. _2026) [default: ""]
  -r, --recursive        Also process files in subdirectories (subdirectory names are kept)
  -D, --dirs             Also rename subdirectories (implies --recursive)
  -i, --interactive      Confirm each item individually
  -f, --force            Rename items that already match the random pattern
  -n, --dry-run          Show the preview only, without renaming
  -h, --help             Print help
  -V, --version          Print version
```

## Undo

Every rename run that actually renames at least one item records the change in
a hidden manifest (`.randanarana-undo.json`) inside the target directory, so
you can restore the previous names:

```
$ randanarana ~/Pictures
...
$ randanarana undo ~/Pictures
Target: /home/thos/Pictures
2 items to restore:
  7Qkz3YpT.jpg -> IMG_2020.jpg
  Lw4aVb9x.png -> photo.png

Restore these 2 items? [y/N] y

Done: 2 restored, 0 skipped, 0 failed.
```

`randanarana undo` without a directory defaults to the current directory. Use
`-n` to preview without restoring. The manifest only records the last run: a
new run that renames something overwrites it, and a successful undo removes it.

## Examples

Preview what would change, without touching anything:

```
randanarana -n ~/Pictures
```

Rename everything in a directory tree, asking for each item:

```
randanarana -r -i ~/Pictures
```

Rename subdirectories too, with a custom prefix and suffix:

```
randanarana -D -l 12 -p img_ -s _2026 ~/Pictures
```

Undo the last rename run in a directory:

```
randanarana undo ~/Pictures
```

## Behavior

- Names are made of `a-zA-Z0-9` and keep the original file extension.
  Directories get names without an extension.
- Names already matching the random pattern are skipped unless you pass
  `--force`.
- Hidden items (leading dot) are never touched, including the undo manifest.
- When more than 20 items are planned, the preview is truncated with a note.
- Interactive mode accepts `y`/`N`/`a` (yes to all)/`q` (quit).
- The exit code is `130` if the run is interrupted; a partial run can still be
  undone via the manifest.
- `undo` skips entries whose new name no longer exists, fails on entries whose
  original name is taken, and keeps the manifest when anything failed.

## Development

```
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## License

MIT
