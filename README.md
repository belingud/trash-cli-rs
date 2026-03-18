# trasher

A friendly command-line tool to move files to trash/recycle bin, based on the `trash-rs` crate.

## Installation

```bash
cargo install trasher
```

Install from source:

```bash
git clone https://github.com/belingud/trasher.git
cd trasher
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
