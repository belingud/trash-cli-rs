# Release Checklist

This checklist is the release process for `trash-cli-rs`.

## Required Inputs

- target version
- final changelog / release notes text

## Checklist

1. Review `CHANGELOG.md` and make sure `Unreleased` reflects the actual changes.
2. Run `just release-check`.
3. Verify a clean install path with the locked dependency graph:
   `just install-smoke`
4. Confirm the CLI contract docs still match reality:
   - `README.md`
   - `docs/rm-option-compatibility.md`
   - `docs/platform-differences.md`
5. Update `CHANGELOG.md`:
   - move relevant items out of `Unreleased`
   - add a dated version section
6. Prepare the release commit and tag:
   `just prepare-release <version>`
7. Review the generated diff and tag locally.
8. Publish:
   `just publish <version>`
9. Confirm GitHub Actions created the GitHub Release for the pushed tag.
10. Confirm the release contains packaged binaries for:
    - Linux x64
    - Linux arm64
    - Windows x64
    - macOS arm64
    - macOS Intel

## Minimum Acceptance Criteria

- `cargo fmt --all --check` passes
- `cargo test` passes
- `cargo install --path . --locked` succeeds in a temporary root
- documentation matches the released behavior

## Notes

- `just release-check` is the stable preflight entrypoint.
- Pushing a version tag now triggers the release job in `.github/workflows/ci.yml`, which creates a GitHub Release automatically after the CI matrix passes.
- The same workflow also uploads packaged binaries for each supported release target as GitHub Release assets.
- If the CLI contract changes, update docs before shipping.
- If behavior differs by platform, update `docs/platform-differences.md` before shipping.
