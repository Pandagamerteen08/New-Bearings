# Changelog

This file records notable changes to New Bearings.

The project is in early development. This changelog should describe changes
that have actually occurred, not features that are merely proposed.

## How entries should be written

Entries should be understandable to users and contributors.

Where applicable, group changes under:

- Added: new functionality or documentation.
- Changed: modifications to existing behavior.
- Fixed: corrections to defects.
- Removed: functionality that is no longer available.
- Security: relevant security fixes or advisories.

Do not claim that a feature is complete merely because its design has been
discussed or its implementation has begun.

## Unreleased

### Added

- Established this changelog as the location for recording project changes.
- Documented the project's intended change-reporting structure.
- Onboarding now continues into the learning screens: the setup stage gained a
  `learning` stage with `start_learning`, and the stage is remembered between
  runs.
- A learning catalog in Rust (`src/learning`), with paths, lessons, three kinds of
  lesson block and multiple-choice knowledge checks.
- The first real learning content: the `rust-foundations` path, three lessons and
  six knowledge checks, in `content/programming/rust/foundations.json`.
- Content validation that reports every problem with its location, plus tests
  that assert the bundled content is valid.
- Answer judging in Rust. Correct answers are never serialized to the interface,
  and a test asserts the answer key cannot leak.
- A completion rule and progress records (`src/progress`): a lesson is complete
  only when all of its checks have been answered correctly.
- Versioned local persistence (`src/storage`) in the platform application data
  directory, written through a temporary file and a rename.
- Recovery for unusable saved data: the old file is kept aside and the learner is
  told, rather than losing it silently.
- Learning screens in the interface: dashboard with real progress, path view,
  lesson reader, answer feedback and a next-lesson action.

### Changed

- Documentation is being organized around the New Bearings project name.
- `serde_json` is now a runtime dependency, because content parsing and saved
  data both use it.
- The README describes the current implementation instead of the original design
  draft, and states plainly what does and does not work.
- The Rust checks in CI install Tauri's Linux system libraries, run
  `--all-targets`, and a separate job now builds the frontend.

### Fixed

- `cargo fmt --check` failed in CI on all three runners, which skipped
  compilation, tests and Clippy entirely.
- A wrong answer used to lock a knowledge check, which made a lesson impossible
  to finish after a single mistake. Checks now stay open until they are answered
  correctly.

## Release history

No stable public release history is established by this document.

Add a dated release entry when a release has actually been published.

## Release entry template

Use the following structure for future releases:

```markdown
### [Version] - YYYY-MM-DD

#### Added

- Describe a new, verified capability.

#### Changed

- Describe a change in existing behavior.

#### Fixed

- Describe a verified correction.

#### Removed

- Describe a removed capability and any relevant migration information.

#### Security

- Describe a security correction without exposing sensitive exploit details.
```

Omit empty categories when preparing a real release entry.

## Accuracy requirements

Every entry should be supported by a commit, merged change, test result,
release artifact, or another appropriate record.

Distinguish implementation from documentation. A planned feature is not an
implemented feature, and an implemented feature is not necessarily ready for
release.

Do not include private user information, credentials, access tokens, or other
secrets in changelog entries.

## Related documentation

- [README](README.md)
- [Development](docs/DEVELOPMENT.md)
- [Release Plan](docs/RELEASE_PLAN.md)
- [Roadmap](docs/ROADMAP.md)
- [Security Policy](docs/SECURITY_POLICY.md)
