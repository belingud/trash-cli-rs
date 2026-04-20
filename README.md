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
trash [rm-options...] <file> [<file> ...]
trash --help
trash --version
```

## Examples

```bash
trash file1.txt file2.txt
trash /path/to/file.txt
trash -rf build
trash --force --recursive build
trash -- -rf
trash ./-rf
```

## rm Alias Compatibility

`trash-cli-rs` is designed to work well with aliases such as `alias rm="trash"`.
Common `rm` compatibility flags like `-f`, `-r`, `-rf`, `--force`, and `--recursive`
are ignored so the remaining operands are still moved to trash.

If you need to trash a file whose name starts with `-`, use an explicit separator or path:

```bash
trash -- -rf
trash ./-rf
```

Quoting `"-rf"` alone is not enough, because the shell removes quotes before passing
arguments to the program.

## License

MIT
