# trash-cli-rs

[中文](./README-zh.md)

A friendly command-line tool to move files to trash/recycle bin, based on the `trash-rs` crate.
It is designed first for safe trashing, and second for working well with aliases such as `alias rm="trash"`.

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
trash [rm-compat-options...] <file> [<file> ...]
trash --help
trash --version
```

## Project Scope

`trash-cli-rs` is not a semantic replacement for `rm`.
It ignores a documented set of `rm`-style compatibility flags so common alias workflows keep working,
but it does not implement `rm` semantics such as force-delete behavior, interactive prompts,
secure overwrite, whiteout recovery, or recursive traversal logic.

The public compatibility contract lives in:

- [docs/rm-option-compatibility.md](docs/rm-option-compatibility.md)
- [docs/platform-differences.md](docs/platform-differences.md)
- [docs/release-checklist.md](docs/release-checklist.md)

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
Documented compatibility flags are ignored before operands so the remaining paths are still moved to trash.
Unknown dashed arguments are treated as literal operands and will fail normally if no such path exists.
That operand treatment is an intentional long-term compatibility promise, not a temporary behavior.

If you need to trash a file whose name starts with `-`, use an explicit separator or path:

```bash
trash -- -rf
trash ./-rf
```

Quoting `"-rf"` alone is not enough, because the shell removes quotes before passing
arguments to the program.

## License

MIT
