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
- Onboarding continues into the learning screens: the setup stage gained a
  `learning` stage with `start_learning`, and the stage is remembered between
  runs.
- A learning catalog in Rust (`src/learning`) with paths, lessons and validation,
  and the screen state (`dashboard`, `review`, `path`, `lesson`).
- Answer judging in Rust. Correct answers are never serialized to the interface,
  and a test asserts the answer key cannot leak.
- A completion rule and progress records (`src/progress`): a lesson is complete
  only when all of its checks have been answered correctly.
- Versioned local persistence (`src/storage`) in the platform application data
  directory, written through a temporary file and a rename.
- Recovery for unusable saved data: the old file is kept aside and the learner is
  told, rather than losing it silently.
- **Three kinds of knowledge check** — multiple choice, true/false and typed short
  answer — all graded in Rust. Short answers are compared after trimming,
  lower-casing, collapsing whitespace and dropping a trailing full stop; an empty
  answer is rejected instead of being scored as wrong.
- **Learning objectives** on every lesson, required by content validation and
  shown at the top of the lesson.
- **New section kinds**: numbered steps, comparison tables and a labelled common
  mistake, alongside explanation, code and note.
- **A second learning path**, *How the Web Works* (3 lessons, 9 checks), and the
  Rust path expanded to 4 lessons and 12 checks with a new ownership lesson.
- **Attempt and mistake tracking per check.** A wrong answer does not advance
  completion but is stored, which is what makes review and honest feedback
  possible.
- **Review screen** listing every lesson that contains a wrong answer, ordered by
  mistakes and then recency. Completed lessons stay in the queue.
- **Bookmarks** per lesson, toggled from the path list, the review list or the
  lesson, counted on the dashboard, and persisted.
- **Search**, implemented in Rust, across path titles, summaries, subjects and
  tags, and lesson titles, summaries and objectives. Empty queries match nothing.
- **Dashboard summary** with lessons completed, questions solved, minutes of
  completed lessons and saved lessons, plus a progress bar per path.
- **Resume follows the lesson you opened last**, falling back to the first
  unfinished lesson in content order.
- **Saved-data schema version 2**, with automatic migration from version 1: the
  correct answers in an old file become successful attempts, and completion times
  are preserved.
- New commands: `show_learning_dashboard`, `show_learning_review`,
  `toggle_lesson_bookmark`, `search_learning`. `submit_lesson_answer` now takes a
  typed answer object instead of an option index.
- Rewritten [README](README.md) describing what exists, how requests flow, where
  data lives and what is missing.
- New [docs/FILE_REFERENCE.md](docs/FILE_REFERENCE.md) describing every tracked
  file.
- A concrete, authoritative [docs/CONTENT_FORMAT.md](docs/CONTENT_FORMAT.md) for
  format version 2.
- A `custom-protocol` Cargo feature, so an optimised build embeds the interface
  instead of loading it from the development server. `tauri build --no-bundle`
  enables it and produces a standalone executable.
- Real installation instructions in [docs/INSTALLATION.md](docs/INSTALLATION.md),
  replacing a placeholder.
- Tests now cover grading for each exercise kind, the review queue, bookmarks,
  search, resume selection, migration, and the serialization contract, for a
  total of 115.

### Changed

- The lesson view now carries each check's solved state and attempt count, so
  reopening a lesson shows what was already answered instead of asking again.
- The saved progress record changed shape: checks hold attempts, mistakes,
  correctness and a timestamp; bookmarks and the last opened lesson are stored
  alongside them.
- `scripts/build.ps1` builds the frontend before `cargo build`, and reports where
  the executable lands.
- `scripts/run.ps1` starts the application through the Tauri CLI, because
  `cargo run` alone leaves the window without a development server.
- `scripts/test.ps1` now runs the same checks as CI, including Clippy and the
  frontend build.
- `scripts/setup-dev.ps1` checks Node and npm as well as the Rust toolchain.
- Documentation is organized around the New Bearings project name.
- `serde_json` is a runtime dependency, because content parsing and saved data
  both use it.
- The Rust checks in CI install Tauri's Linux system libraries, run
  `--all-targets`, and a separate job builds the frontend.
- The browser tab title is `New Bearings` instead of `frontend`.
- The test directories now explain where the real tests live instead of claiming
  there is nothing to test.

### Fixed

- `cargo fmt --check` failed in CI on all three runners, which skipped
  compilation, tests and Clippy entirely.
- A wrong answer used to lock a knowledge check, which made a lesson impossible
  to finish after a single mistake. Checks now stay open until they are answered
  correctly.
- Two lessons opened within the same second could make "continue learning" pick
  the wrong one, because resume compared second-resolution timestamps. It now
  stores the last opened lesson explicitly.
- An executable built with plain `cargo build` opened
  `http://localhost:5173` and showed `ERR_CONNECTION_REFUSED` when the dev
  server was not running, because Tauri only embeds the interface with its
  `custom-protocol` feature. The feature is now declared and
  `scripts/build.ps1` uses `tauri build --no-bundle`, producing a release
  executable that renders on its own. The installation guide explains the
  difference.

### Removed

- Nothing has been removed.

### Security

- No security-relevant changes in this period.

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
