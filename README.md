# trash-cli-rs

A friendly command-line tool to move files to trash/recycle bin, based on the `trash-rs` crate.

Just a reminder, new macOS (newer than macOS 14) has built-in trash command.

## Installation

```bash
cargo install trash-cli-rs
```

Install from source:

```bash
git clone https://github.com/belingud/trash-cli-rs.git
cd trash-cli-rs
cargo install --path .
```

## Usage

```bash
trash <file> [<file> ...]
```

## Examples

```bash
trash file1.txt file2.txt
trash /path/to/file.txt
```

## License

MIT
