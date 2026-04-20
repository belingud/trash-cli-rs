# rm Option Compatibility

This document is the public compatibility contract for `trash-cli-rs`.

## Scope

- Primary use case: aliases such as `alias rm="trash"`.
- Primary action: move the given paths to the system trash.
- Parsing rule: documented compatibility flags are ignored only before an exact `--`.
- Literal rule: after `--`, every following argument is treated as a literal operand.
- Safety rule: unknown dashed arguments are not silently swallowed; they are treated as operands.
- Diagnostics rule: missing-path errors intentionally use `trash`-prefixed wording such as `trash: <path>: No such file or directory`.

## Long-Term Compatibility Promise

- Documented ignored flags will not silently change behavior in a patch release.
- Unsupported dashed arguments will continue to be treated as literal operands unless a future major version explicitly documents a breaking change.
- Argument parsing rules are intended to stay platform-independent; backend differences belong in [platform-differences.md](platform-differences.md).

## Non-Goals

`trash-cli-rs` does not try to replicate `rm` semantics.
Ignoring a flag only means "accept it without error while trashing the remaining operands".
It does not mean the tool implements the original `rm` behavior behind that flag.

The tool does not promise:

- force-delete behavior
- interactive confirmation prompts
- recursive directory walking with per-entry logic
- secure overwrite / shredding
- whiteout undelete behavior
- mount-point traversal controls

## Built-In Options

These are `trash-cli-rs` options, not `rm` compatibility options.

| Option            | Status    | Behavior                                                                        |
| ----------------- | --------- | ------------------------------------------------------------------------------- |
| `-h`, `--help`    | Supported | Show help text and exit `0`.                                                    |
| `-V`, `--version` | Supported | Show version and exit `0`.                                                      |
| `--`              | Supported | Stop parsing compatibility flags; all following arguments are literal operands. |

## macOS / BSD rm Short Options

Reference dialect: macOS `/bin/rm` (`rm [-f | -i] [-dIRrvWx] file ...`).

| Option | rm meaning                               | `trash-cli-rs` status | Actual behavior in `trash-cli-rs`                                               |
| ------ | ---------------------------------------- | --------------------- | ------------------------------------------------------------------------------- |
| `-d`   | Remove directories                       | Ignored               | Directory operands are still passed to trash as paths.                          |
| `-f`   | Force, suppress missing-file diagnostics | Ignored               | Missing paths still produce errors.                                             |
| `-i`   | Prompt before each removal               | Ignored               | No prompt is shown.                                                             |
| `-I`   | Prompt once for risky removals           | Ignored               | No prompt is shown.                                                             |
| `-P`   | Backward-compat no-op on macOS           | Ignored               | Accepted as a no-op compatibility flag.                                         |
| `-R`   | Recursive removal                        | Ignored               | No recursive walk is implemented; the operand path is trashed as a single path. |
| `-r`   | Same as `-R`                             | Ignored               | Same behavior as `-R`.                                                          |
| `-v`   | Verbose output                           | Ignored               | Success remains silent.                                                         |
| `-W`   | Attempt undelete for whiteouts           | Not supported         | Treated as a literal operand.                                                   |
| `-x`   | Do not cross mount points                | Ignored               | Accepted as a no-op compatibility flag.                                         |

## GNU rm Long-Option Compatibility

These options are accepted because many `rm` aliases and shell habits come from GNU/Linux.

| Option                 | `trash-cli-rs` status | Actual behavior in `trash-cli-rs`                           |
| ---------------------- | --------------------- | ----------------------------------------------------------- |
| `--dir`                | Ignored               | No special semantics beyond trashing the operand path.      |
| `--force`              | Ignored               | Missing paths still produce errors.                         |
| `--interactive`        | Ignored               | No prompt is shown.                                         |
| `--interactive=always` | Ignored               | No prompt is shown.                                         |
| `--interactive=never`  | Ignored               | Missing paths still produce errors.                         |
| `--interactive=once`   | Ignored               | No prompt is shown.                                         |
| `--recursive`          | Ignored               | No recursive walk is implemented.                           |
| `--verbose`            | Ignored               | Success remains silent.                                     |
| `--one-file-system`    | Ignored               | No mount-point traversal logic is implemented.              |
| `--preserve-root`      | Ignored               | No root-preservation logic is implemented by the CLI layer. |
| `--preserve-root=all`  | Ignored               | Same as `--preserve-root`.                                  |
| `--no-preserve-root`   | Ignored               | Same as `--preserve-root`.                                  |

## Short-Option Clusters

If every letter in a short cluster is a documented compatibility flag, the entire cluster is ignored.

Examples:

- `-rf`
- `-rfxP`
- `-Riv`

If a cluster contains any unsupported letter, the whole token is treated as a literal operand.

Examples:

- `-W`
- `-rW`
- `-rxW`

## Leading-Dash Filenames

To trash a filename that starts with `-`, disambiguate it explicitly:

```bash
trash -- -rf
trash ./-rf
trash /tmp/-rf
```

Quoting alone is not enough, because the shell removes quotes before constructing `argv`.
