# Platform Differences

This document records the platform-specific behavior that matters for `trash-cli-rs`.

## Current Verification Status

| Platform | Upstream backend exists | Verified in this project CI | Notes |
| --- | --- | --- | --- |
| macOS | Yes | Yes | Actively exercised in CI and local development. |
| Linux | Yes | Yes | Actively exercised in CI. |
| Windows | Yes | CI configured | Windows is now included in the GitHub Actions matrix via `windows-2025`; treat it as verified once the first remote run is green. |

## macOS

`trash-cli-rs` currently relies on the `trash` crate's macOS backend.
In `trash 5.2.5`, the backend exposes two deletion methods:

- `Finder`
- `NSFileManager`

The default is `Finder`, and `trash-cli-rs` does not override that default today.

Practical consequences:

- Files are moved through Finder semantics.
- The operation may trigger the normal Finder trash behavior, including UI-related side effects from Finder.
- The upstream backend documents that `NSFileManager` can be faster and quieter, but may lose Finder's "Put Back" behavior on some systems.

Source used:

- `trash-5.2.5/src/macos/mod.rs`

## Linux

On Linux, the upstream backend follows the Freedesktop Trash Specification 1.0.

Practical consequences:

- Trash placement is mount-point aware.
- Behavior depends on the presence and accessibility of the expected trash folders.
- The upstream crate documents that it assumes desktop Linux systems follow the Freedesktop convention.

Source used:

- `trash-5.2.5/src/lib.rs`
- `trash-5.2.5/src/freedesktop.rs`

## Windows

The upstream backend uses the Windows Shell / Recycle Bin APIs through `IFileOperation` with undo enabled.

Practical consequences:

- Deletions are intended to go to the Recycle Bin rather than being unlinked directly.
- Windows is now included in the CI matrix, but the claim should still be backed by an actual green GitHub Actions run before release messaging upgrades from "configured" to "verified".

Source used:

- `trash-5.2.5/src/windows.rs`

## Compatibility Promise

The CLI compatibility contract in [rm-option-compatibility.md](rm-option-compatibility.md) is intended to remain platform-independent.
Platform differences should affect trash backend behavior, not argument parsing rules.

If a platform requires a CLI-level exception in the future, it should be documented explicitly before release.
