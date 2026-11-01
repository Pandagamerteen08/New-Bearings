# File reference

Every file tracked in the repository, what it is for, and how it relates to the
rest of the project. This is the detailed companion to the
[README](../README.md); it is organised by directory rather than by importance.

Role labels:

| Label | Meaning |
| --- | --- |
| **Runtime** | Compiled into the application or served to its window. |
| **Dev** | Used while developing or building. |
| **Test** | Only used by tests or used to describe them. |
| **Config** | Configuration read by a tool. |
| **Docs** | Human-facing documentation. |
| **Asset** | Image, icon or other binary resource. |
| **Generated** | Produced by a tool and checked in for reproducibility. |

---

## Repository root

| Path | Role | Purpose and contents |
| --- | --- | --- |
| [`README.md`](../README.md) | Docs | Entry point: what the application does, status of each feature, architecture diagram, request flow, content overview, data location, development commands, testing, limitations and links to the rest of the documentation. |
| [`CHANGELOG.md`](../CHANGELOG.md) | Docs | Record of changes that actually happened, grouped into Added/Changed/Fixed/Removed/Security, plus the template and accuracy rules for future entries. |
| [`Cargo.toml`](https://github.com/Pandagamerteen08/New-Bearings/blob/main/Cargo.toml) | Config | Rust package metadata (`new-bearings` 0.1.0, edition 2024, MIT declared, not published) and the `[[bin]]` target `new-bearings` at `src/main.rs`. Dependencies: `serde` (derive), `serde_json`, `tauri` 2. Build dependency: `tauri-build` 2. |
| `Cargo.lock` | Generated | Exact dependency graph resolved by Cargo, including Tauri and its transitive crates. Commit changes to it whenever dependencies change so builds are reproducible. Do not edit by hand. |
| [`build.rs`](https://github.com/Pandagamerteen08/New-Bearings/blob/main/build.rs) | Dev / Runtime | Build script. Calls `tauri_build::build()`, which reads `tauri.conf.json`, validates the configuration and icons, generates the Tauri context used by `tauri::generate_context!`, and emits the ACL/capability data consumed at runtime. |
| [`tauri.conf.json`](../tauri.conf.json) | Config | Tauri application configuration. `identifier` (`com.newbearings.desktop`) determines the app-data directory. `build` runs `npm run dev -- --host 127.0.0.1` before `tauri dev`, loads `http://localhost:5173`, runs `npm run build` before a release build, and reads `frontend/dist`. One 1100×760 window (minimum 720×560). `bundle.active` is `false`, so no installer is produced; `bundle.icon` lists the generated icon set. |
| [`.gitignore`](https://github.com/Pandagamerteen08/New-Bearings/blob/main/.gitignore) | Config | Ignores build output and artefacts: `target/`, `build/`, `installers/`, Rust backup files, debug symbols, platform packages (`.exe`, `.msi`, `.dmg`, `.deb`, `.AppImage`, …), `node_modules/`, generated schemas under `src-tauri/gen`, and editor scratch files. |
| [`.editorconfig`](https://github.com/Pandagamerteen08/New-Bearings/blob/main/.editorconfig) | Config | Editor defaults: UTF-8, LF line endings, final newline, 4-space indentation, trim trailing whitespace (except Markdown), 2-space YAML. |
| [`.vscode/settings.json`](https://github.com/Pandagamerteen08/New-Bearings/blob/main/.vscode/settings.json) | Config | Two workspace preferences: do not compact folders in the explorer, and let rust-analyzer run build scripts (needed because `build.rs` generates Tauri code). |
| [`LICENSE`](https://github.com/Pandagamerteen08/New-Bearings/blob/main/LICENSE) | Docs | **Placeholder**: contains `# Hello`, while `Cargo.toml` declares MIT. The licence has deliberately not been chosen here. |
| [`ACCESSIBILITY.md`](../ACCESSIBILITY.md) | Docs | Accessibility intentions and responsibilities: keyboard access, focus, contrast, reduced motion, clear language, and the expectation that accessibility is reviewed continuously rather than declared finished. |
| [`CONTRIBUTING.md`](../CONTRIBUTING.md) | Docs | How to contribute: scope, the Rust-first boundary, expectations for tests and documentation, and what not to claim. |
| [`CODE_OF_CONDUCT.md`](../CODE_OF_CONDUCT.md) | Docs | Community expectations and behaviour. |
| [`SECURITY.md`](https://github.com/Pandagamerteen08/New-Bearings/blob/main/SECURITY.md) | Docs | How to report a vulnerability privately, and what not to publish. |
| [`PRIVACY_POLICY.md`](../PRIVACY_POLICY.md) | Docs | Privacy principles for the project; explicitly not a finalised policy for a distributed product. |
| [`TERMS_OF_SERVICE.md`](https://github.com/Pandagamerteen08/New-Bearings/blob/main/TERMS_OF_SERVICE.md) | Docs | Preliminary terms; states that it has not been legally reviewed. |
| [`DISCLAIMER.md`](https://github.com/Pandagamerteen08/New-Bearings/blob/main/DISCLAIMER.md) | Docs | Early-stage status disclaimer: features and interfaces may change, and nothing here is suitable for important tasks. |

## `.github/`

| Path | Role | Purpose and contents |
| --- | --- | --- |
| [`.github/workflows/ci.yml`](../.github/workflows/ci.yml) | Config / Dev | Continuous integration. Triggers on every push, pull request and manual dispatch, with read-only repository permissions. **Rust checks** job runs on `windows-latest`, `ubuntu-latest` and `macos-latest` with `fail-fast: false`: checkout, install Tauri's Linux system libraries on Linux only (`libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libxdo-dev`, etc., without which even `cargo check` fails), install stable Rust with rustfmt and clippy, `cargo fmt --check`, `cargo check --all-targets`, `cargo test`, `cargo clippy --all-targets -- -D warnings`. The compile step pipes through `tee` with `pipefail` and republishes matching error lines as workflow annotations, so a platform-only failure is readable from the run summary. **Frontend build** job runs on Ubuntu: Node 24 with npm cache, `npm ci`, `npm run build`. |

## `src/` — Rust application

The Rust side is the decision layer. Modules are registered explicitly in
`main.rs` with `#[path]` attributes, so the directory layout and the module tree
stay readable despite the project using a flat `src/` without `mod.rs` files.

### Core modules

#### [`src/main.rs`](../src/main.rs) — Runtime

Entry point and the entire command surface.

- Declares the six active modules (`app`, `learning`, `lessons`, `progress`,
  `setup`, `storage`).
- Defines the fourteen Tauri commands: `get_app_info`, `get_setup_state`,
  `begin_setup`, `return_to_welcome`, `start_learning`, `get_learning_state`,
  `open_learning_path`, `open_learning_lesson`, `show_learning_dashboard`,
  `show_learning_review`, `go_back_in_learning`, `submit_lesson_answer`,
  `toggle_lesson_bookmark`, `search_learning`.
- `main` generates the Tauri context, and in `setup` resolves the platform
  application-data directory, creates `AppStorage` and registers a single
  `Session` as managed state. It then registers the handlers and runs the app.
- A startup failure prints the application name, version and error, then exits
  with status 1.
- Every command is intentionally thin: it forwards to a `Session` method and
  returns that method's typed result or error. It contains no rules.

Limitation: command names are part of the frontend contract; renaming one
requires updating `frontend/src/backend/*.ts`.

#### [`src/app/app.rs`](../src/app/app.rs) — Runtime

Application identity and re-exports.

- `APP_NAME` ("New Bearings") and `APP_VERSION` (from `CARGO_PKG_VERSION`).
- Declares `session` and re-exports `AnswerOutcome` and `Session`, so `main.rs`
  only needs `app::`.
- Two tests pin the name and assert the version is not empty.

#### [`src/app/session.rs`](../src/app/session.rs) — Runtime / Test

The authoritative application state. Everything that changes state goes
through this file.

- `AnswerOutcome` — the response to a submitted answer: whether it was correct,
  the authored explanation, whether it finished the lesson, and the resulting
  snapshot.
- `Inner` — setup stage, learning screen, progress, storage handle, a
  startup recovery notice and a last-save warning.
- `Inner::warning` combines those two notices; `Inner::persist` writes
  `AppData` and, on failure, logs the technical reason to stderr and exposes a
  plain learner-facing warning on the next snapshot.
- `Session::open` loads storage and builds the initial state, never failing: data
  that cannot be used becomes a fresh state plus a notice.
- Setup methods: `setup_snapshot`, `begin_setup`, `return_to_welcome`,
  `start_learning`.
- Learning methods: `learning_snapshot`, `open_path`, `open_lesson` (also records
  the resume pointer and persists), `show_review`, `show_dashboard`, `go_back`.
- `submit_answer` grades first, then records the attempt (correct *or* wrong) and
  persists, so mistakes survive a restart and feed the review queue.
- `toggle_bookmark` validates the path and lesson before changing anything.
- `search` is read-only and needs no lock.
- `catalog()` converts a content-loading failure into
  `LearningError::ContentInvalid` after printing each problem to stderr, because
  invalid bundled content is a build defect.
- Locks recover from poisoning by taking the inner value: every field is replaced
  as a whole, so a panic cannot leave a half-updated state.
- Tests use a `TempDirectory` helper and cover the full command chain, restart
  persistence, a legacy file, a failing save, and bookmarks.

Limitation: one process, one learner; there is no cross-process file locking,
which is acceptable for a desktop application that owns its data directory.

#### [`src/setup/setup.rs`](../src/setup/setup.rs) — Runtime / Test

Onboarding state machine.

- `SetupStage` — `welcome`, `introduction`, `learning` (snake_case on the wire,
  `Welcome` by default).
- `SetupSnapshot` — the stage plus `canBegin`, `canReturnToWelcome` and
  `canStartLearning`, derived from the same rules the transitions enforce.
- `SetupState` — `begin`, `return_to_welcome`, `start_learning`; each returns a
  typed `SetupError` when the transition is not legal (already started, already
  at welcome, already learning, not at the introduction). `from_stage` restores a
  saved stage.
- `SetupError` serializes to a lowercase code so wording stays in content files,
  and implements `Display`/`Error` for developer messages.
- Tests cover every legal and illegal transition, the flags, and serialization.

#### [`src/lessons/lessons.rs`](../src/lessons/lessons.rs) — Runtime / Test

The content model, its validation, the answer-safe projections, and grading.

- `SUPPORTED_FORMAT_VERSION` (2), `Difficulty` (`beginner`/`intermediate`/
  `advanced`).
- `Section` — tagged enum with `explanation`, `code`, `note`, `steps`, `table`
  and `mistake`.
- `CheckKind`, `SubmittedAnswer` (tagged `choice`/`trueFalse`/`shortAnswer`) and
  `AnswerProblem` (`kind_mismatch`, `option_out_of_range`, `empty_answer`).
- `Check` — the three authored variants. Correct answers, accepted wordings and
  explanations live here and the type deliberately does not implement
  `Serialize`.
- `Check::grade` is the only place an answer is compared:
  choice indexes must be in range, true/false compares booleans, short answers are
  normalized and matched against the accepted list, and an empty answer is an
  error rather than a wrong answer.
- `normalize_answer` trims, lower-cases, collapses whitespace and drops trailing
  full stops.
- `CheckView` — what the interface receives: id, kind, prompt, options (empty for
  non-choice checks), `solved` and `attempts`.
- `Lesson` — id, title, summary, optional minutes, objectives, sections, checks;
  `view(record)` projects it with the learner's per-check state.
- `Path` — id, title, summary, subject, difficulty, tags, lessons; `lesson(id)`
  and `total_minutes()`.
- `ContentProblem` — location plus message; validation collects all problems.
- `parse_pack` — parses a pack, checks the format version and non-empty path list,
  then validates each path, returning every problem or the paths.
- Validation helpers check slugs, required text, objectives, section shapes
  (including table row widths and step counts), check shapes
  (options/answer range, accepted wordings), and duplicate ids within a path or
  lesson.
- Tests: bundled content is valid, objectives present, ids unique, answers cannot
  leak into a view, table/steps/short-answer/choice validation failures report
  the right location, malformed JSON is reported, and grading behaves for all
  three kinds.

Limitation: `language` on a code section is a label only; no highlighting or
execution. Sections have no ids, so there are no per-section anchors yet.

#### [`src/learning/learning.rs`](../src/learning/learning.rs) — Runtime / Test

The catalog, the learner's position in it, and everything a snapshot contains.

- `EMBEDDED_PACKS` — the compiled-in content files. Adding a pack is one line.
- `Catalog` — loaded once into a `OnceLock`; exposes `paths`, `path`, `lesson`
  and `search`. Loading validates each pack, prefixes problems with the pack
  index, and rejects path ids duplicated across packs.
- `search` matches case-insensitively over path title/summary/subject/tags and
  lesson title/summary/objectives, excludes everything for an empty query, caps
  results at 20 and shortens long matches to an excerpt.
- `MatchedField` and `SearchHit` — a result names whether it is a path or a
  lesson, gives ids to open, and the matching text.
- `Screen` — tagged enum: `dashboard`, `review`, `path`, `lesson`.
- `LearningError` — `content_invalid`, `path_not_found`, `lesson_not_found`,
  `check_not_found`, `option_out_of_range`, `answer_mismatch`, `empty_answer`;
  serializes to its code and converts from `AnswerProblem`.
- Views: `ResumePoint`, `PathView`, `LessonSummaryView` (check count, solved
  count, attempts, mistakes, completion time, bookmark), `ProgressSummary`,
  `ReviewItem`, and `LearningSnapshot` (screen, back flag, paths, open lesson,
  resume, summary, review queue, storage warning).
- `LearningSnapshot::build` derives all of it from the catalog and progress:
  per-path lesson lists, the open lesson with per-check state, the resume point,
  totals and the review queue.
- Resume prefers the last opened lesson if it exists and is unfinished, then the
  first unfinished lesson in content order.
- The review queue contains lessons with at least one mistake, sorted by mistakes
  descending, then most recent attempt, then title.
- `evaluate_answer` looks up the lesson and check and grades the submitted
  answer, returning the explanation and the full list of required checks.
- `LearningSession` — the screen state machine: `open_path` and `open_lesson`
  validate existence first; `go_back` walks lesson → path → dashboard, and review
  or path → dashboard.
- Test-only helpers `correct_answer` and `wrong_answer` build submissions for any
  check kind and are reused by the session tests.
- Tests cover catalog loading and multi-subject coverage, every screen
  transition, grading for all kinds, snapshot contents, resume selection,
  review, bookmarks, search, and the serialization contract.

Limitation: the catalog is a process-wide `OnceLock`, which is correct because
content is compiled in; it would need changing if content ever became editable at
runtime.

#### [`src/progress/progress.rs`](../src/progress/progress.rs) — Runtime / Test

What the learner has answered, and the rules that turn answers into progress.

- `LessonKey` — path id plus lesson id; serializes as `{ pathId, lessonId }`,
  exposes getters, and has a stable `path/lesson` string form for map keys.
- `AnswerEffect` — `Recorded` or `LessonCompleted`.
- `CheckRecord` — attempts, mistakes, correct, last attempt time.
- `LessonRecord` — per-check records, completion time, and helpers for solved
  count, attempt count, mistakes and last attempt.
- `ProgressState` — lessons, bookmarks and the last opened lesson.
- `record_attempt` increments attempts, records a mistake when wrong, marks the
  check correct when right, and completes the lesson exactly once when every
  required check is correct. A wrong answer can never complete a lesson.
- `mark_opened` / `last_opened` store the resume pointer; a pointer rather than a
  timestamp, because two lessons can be opened in the same second.
- Bookmarks: `is_bookmarked`, `toggle_bookmark`, `bookmark_count`.
- `from_legacy` converts the version 1 shape (a set of correct check ids and an
  optional completion time) into the current records.
- Tests cover the completion rule, mistakes surviving a later success, review
  counters, opening, bookmarks, serialization round-trips, legacy migration and
  parsing records that omit optional fields.

#### [`src/storage/storage.rs`](../src/storage/storage.rs) — Runtime / Test

Versioned local persistence with recovery.

- `SCHEMA_VERSION` (2) and `STATE_FILE_NAME` (`app-state.json`).
- `AppData` — version, setup stage, progress.
- `LoadOutcome` — data plus an optional learner-facing notice.
- `AppStorage::load` — missing or empty file means a fresh start; a version 2 file
  is parsed directly; a version 1 file is migrated and rewritten in the new
  shape; any other version, or unparseable content, is moved aside as
  `app-state.recovered-<timestamp>.json` with an explanation.
- `AppStorage::save` — creates the directory, serializes pretty JSON to
  `app-state.json.tmp`, flushes it, then renames it over the real file. Failures
  remove the temporary file and return a message.
- Tests cover round-trips, missing directories, no leftover temporary file,
  corrupt data being kept, migration preserving completions and rewriting the
  file, newer formats being refused safely, empty files, and missing optional
  fields.

Limitation: JSON is rewritten in full on each save. That is deliberate for this
data size and keeps the format inspectable; it is not a database.

### Placeholder modules

These twelve files are one-line doc comments. They are not registered in
`main.rs`, compile to nothing, and contain no behaviour. They are kept as named
spaces for future work, and the README lists them as placeholders.

| Path | Intended area | Current content |
| --- | --- | --- |
| `src/accessibility/accessibility.rs` | Accessibility services | One doc comment. |
| `src/achievements/achievements.rs` | Achievements | One doc comment. |
| `src/content_packs/content_packs.rs` | Loading external content packs | One doc comment. |
| `src/errors/errors.rs` | Shared error types | One doc comment. |
| `src/navigation/navigation.rs` | Navigation model | One doc comment. |
| `src/platform/platform.rs` | Platform integration | One doc comment. |
| `src/resources/resources.rs` | Bundled resources | One doc comment. |
| `src/review/review.rs` | Review scheduling | One doc comment; the current review queue lives in `learning`/`progress`. |
| `src/reminders/reminders.rs` | Practice reminders | One doc comment. |
| `src/rewards/rewards.rs` | Rewards | One doc comment. |
| `src/settings/settings.rs` | Settings | One doc comment. |
| `src/ui/ui.rs` | UI services | One doc comment. |

## `content/` — learning content and interface wording

| Path | Role | Purpose and contents |
| --- | --- | --- |
| [`content/programming/rust/foundations.json`](../content/programming/rust/foundations.json) | Runtime (compiled in) | The *Rust Foundations* path: 4 lessons (`first-program`, `variables`, `functions`, `ownership`) and 12 checks, using every section kind and all three check kinds. |
| [`content/web/fundamentals.json`](../content/web/fundamentals.json) | Runtime (compiled in) | The *How the Web Works* path: 3 lessons (`requests-and-responses`, `html-structure`, `css-styling`) and 9 checks, covering HTTP, HTML structure and CSS selectors. |
| `content/programming/rust/.gitkeep` | Asset | Keeps the directory tracked; it holds the Rust pack. |
| `content/programming/javascript/.gitkeep` | Asset | Reserves the JavaScript subject directory; empty. |
| `content/programming/python/.gitkeep` | Asset | Reserves the Python subject directory; empty. |
| `content/programming/typescript/.gitkeep` | Asset | Reserves the TypeScript subject directory; empty. |
| [`content/interface/learning.ts`](../content/interface/learning.ts) | Runtime | Every string the learning screens display: navigation, dashboard, statistics, search, path and lesson labels, exercise labels and placeholders, completion, review, bookmarks, difficulty names and error messages. Typed as `LearningContent`; several entries are functions so numbers can be pluralised in one place. |
| [`content/interface/setup.ts`](../content/interface/setup.ts) | Runtime | Wording for onboarding errors and the "view technical details" label. |
| [`content/interface/welcome.ts`](../content/interface/welcome.ts) | Runtime | Wording for the welcome screen, the introduction, the footer and the backend-status line. |

## `frontend/` — Vue 3 + TypeScript interface

| Path | Role | Purpose and contents |
| --- | --- | --- |
| [`frontend/src/App.vue`](../frontend/src/App.vue) | Runtime | Application shell. Loads the setup snapshot and the application info on mount, renders the welcome or introduction screen while onboarding is unfinished, and the learning shell afterwards. Owns the backend-status line, the footer, the global styles (dark theme, focus rings, reduced motion) and the setup error banner with collapsible technical details. Instantiates `LearningView` and keeps the snapshot it emits. |
| [`frontend/src/components/LearningView.vue`](../frontend/src/components/LearningView.vue) | Runtime | The learning screens. Renders navigation tabs (Learning paths / Review with a count), the dashboard (summary statistics, resume card, review callout, submitted search with results and clear action, path cards with progress bars), the path view (lesson rows with completion, in-progress and mistake state, bookmark toggles) and the review view (mistake counts, completed hint, open and bookmark actions). Holds busy/error state, maps backend error codes to wording, and emits updated snapshots upward. |
| [`frontend/src/components/LessonPane.vue`](../frontend/src/components/LessonPane.vue) | Runtime | The lesson itself. Renders objectives, all six section kinds (including numbered steps, scrollable tables and a labelled common-mistake aside), and the checks: radio options for choice, labelled true/false options, and a labelled text input for short answers. Keeps a selection per interaction kind, restores checks already solved in earlier sessions, blocks resubmission of solved checks, shows correct/incorrect feedback with the explanation, allows another attempt after a mistake, toggles the bookmark, and shows the completion card with next-lesson or back actions. |
| [`frontend/src/backend/learning.ts`](../frontend/src/backend/learning.ts) | Runtime | The typed bridge to the learning commands. Declares the TypeScript mirror of every Rust view (`Section`, `CheckKind`, `CheckView`, `LessonView`, `LessonSummary`, `PathView`, `Screen`, `ResumePoint`, `ProgressSummary`, `ReviewItem`, `LearningSnapshot`, `AnswerOutcome`, `SubmittedAnswer`, `SearchHit`), wraps ten `invoke` calls, and recognises backend error codes. |
| [`frontend/src/backend/setup.ts`](../frontend/src/backend/setup.ts) | Runtime | The typed bridge to the onboarding commands: `getSetupState`, `beginSetup`, `returnToWelcome`, `startLearning`, plus the stage and error-code types. |
| [`frontend/src/main.ts`](../frontend/src/main.ts) | Runtime | Creates the Vue application from `App.vue` and mounts it on `#app`. |
| [`frontend/index.html`](../frontend/index.html) | Runtime | The Vite entry document: UTF-8, viewport, favicon, title `New Bearings`, `<div id="app">` and the module script that loads `src/main.ts`. |
| [`frontend/vite.config.ts`](../frontend/vite.config.ts) | Config | Vite configuration: the Vue plugin and nothing else. Port 5173 is Vite's default and is what `tauri.conf.json` expects. |
| [`frontend/package.json`](../frontend/package.json) | Config | Frontend package definition. Scripts: `dev` (Vite), `build` (`vue-tsc -b && vite build`, so type errors fail the build), `preview`. Dependencies: `@tauri-apps/api` and `vue`. Dev dependencies: `@tauri-apps/cli`, `@types/node`, the Vue plugin, Vue's base tsconfig, TypeScript, Vite and `vue-tsc`. |
| `frontend/package-lock.json` | Generated | npm's exact dependency tree. Use `npm ci` (as CI does) to install exactly what is recorded. |
| [`frontend/tsconfig.json`](../frontend/tsconfig.json) | Config | Solution file with no sources of its own; references the app and node configurations. |
| [`frontend/tsconfig.app.json`](../frontend/tsconfig.app.json) | Config | Type-checks `src/**/*.ts|tsx|vue` for the browser: ES2020, `moduleResolution: bundler`, `noEmit`, strict unused checks, no fallthrough. Note that `content/interface/*.ts` live outside `include` but are still type-checked because the components import them. |
| [`frontend/tsconfig.node.json`](../frontend/tsconfig.node.json) | Config | Type-checks `vite.config.ts` in a Node environment. |
| [`frontend/src/style.css`](../frontend/src/style.css) | Asset (unused) | 296 lines of the Vite/Vue starter stylesheet. It is not imported by `main.ts` or any component; the application's styles live in `App.vue`, `LearningView.vue` and `LessonPane.vue`. Kept, not used. |
| `frontend/src/assets/hero.png` | Asset (unused) | Starter image; no component references it. |
| `frontend/src/assets/vite.svg` | Asset (unused) | Starter logo; no component references it. |
| `frontend/src/assets/vue.svg` | Asset (unused) | Starter logo; no component references it. |
| [`frontend/public/favicon.svg`](../frontend/public/favicon.svg) | Asset | Browser/window icon referenced by `index.html`. |
| `frontend/public/icons.svg` | Asset (unused) | Starter icon sprite; not referenced. |
| [`frontend/README.md`](../frontend/README.md) | Docs | The Vue + TypeScript + Vite template README that came with the starter. It describes the template, not New Bearings; the root README is authoritative. |
| `frontend/.gitignore` | Config | Ignores `node_modules`, `dist`, logs, local overrides and editor folders inside the frontend, while keeping `.vscode/extensions.json`. |
| `frontend/.vscode/extensions.json` | Config | Recommends the Vue.volar extension. |

## `docs/`

| Path | Role | Purpose and contents |
| --- | --- | --- |
| [`docs/FILE_REFERENCE.md`](FILE_REFERENCE.md) | Docs | This document. |
| [`docs/ARCHITECTURE.md`](ARCHITECTURE.md) | Docs | Architectural direction: Rust-first boundaries, maintainability and the intended evolution. Broader than the current implementation; the README describes what exists today. |
| [`docs/CONTENT_FORMAT.md`](CONTENT_FORMAT.md) | Docs | The authoritative content format for version 2: pack, path, lesson, section and check schemas, identifier rules, grading rules, every validation rule, how to add a pack, and the format history. |
| [`docs/DEVELOPMENT.md`](DEVELOPMENT.md) | Docs | Contributor workflow and expectations. |
| [`docs/INSTALLATION.md`](INSTALLATION.md) | Docs | Prerequisites per platform, getting the source, running in development, building a standalone executable, data location, removal and troubleshooting. |
| [`docs/TESTING.md`](TESTING.md) | Docs | Testing strategy and the kinds of evidence the project expects. |
| [`docs/DECISIONS.md`](DECISIONS.md) | Docs | Decision records with reasoning. |
| [`docs/ROADMAP.md`](ROADMAP.md) | Docs | Intended direction and priorities; explicitly not a promise. |
| [`docs/RELEASE_PLAN.md`](RELEASE_PLAN.md) | Docs | How a release would be prepared; no release has been made. |
| [`docs/SECURITY_POLICY.md`](SECURITY_POLICY.md) | Docs | Security handling, complementing `SECURITY.md`. |
| [`docs/PRIVACY_AND_DATA_HANDLING.md`](PRIVACY_AND_DATA_HANDLING.md) | Docs | Privacy principles and data-handling intentions, distinguishing plans from implemented behaviour. |

## `scripts/`

PowerShell helpers for Windows. They call the same commands as CI; they are
conveniences, not a separate build system.

| Path | Role | Purpose and contents |
| --- | --- | --- |
| [`scripts/setup-dev.ps1`](../scripts/setup-dev.ps1) | Dev | Checks for `rustc`, `cargo`, `rustup`, `git`, `node` and `npm`, reports whether `frontend/node_modules` exists, prints links for anything missing, and exits non-zero if a tool is absent. |
| [`scripts/run.ps1`](../scripts/run.ps1) | Dev | Starts the application through `frontend/node_modules/.bin/tauri.cmd dev`, failing with an instruction if the CLI is not installed. `cargo run` alone is deliberately not used. |
| [`scripts/build.ps1`](../scripts/build.ps1) | Dev | Builds the frontend (`npm --prefix frontend run build`), then `cargo build`, and reports the executable path and the fact that no installer is produced. |
| [`scripts/test.ps1`](../scripts/test.ps1) | Dev / Test | Runs `cargo fmt --check`, `cargo check --all-targets`, `cargo test`, `cargo clippy --all-targets -- -D warnings` and `npm --prefix frontend run build`, stopping at the first failure. |

## `icons/`, `resources/`, `gen/schemas/`, `schemas/`

| Path | Role | Purpose and contents |
| --- | --- | --- |
| `icons/32x32.png`, `icons/128x128.png`, `icons/128x128@2x.png` | Asset (generated) | Window and application icons. `tauri-build` needs a PNG on Linux and macOS; without one, compilation fails there. |
| `icons/icon.ico` | Asset (generated) | Windows icon. |
| `icons/icon.icns` | Asset (generated) | macOS icon. |
| `icons/icon.png` | Asset (generated) | 512×512 source for the other sizes. |
| [`resources/icons/app-icon.svg`](../resources/icons/app-icon.svg) | Asset | The source artwork for the icon set: a rounded gradient square with the letter N. Regenerate the icons from it with `tauri icon resources/icons/app-icon.svg`. |
| `resources/icons/icon.ico` | Asset | An earlier Windows icon kept with the source artwork. |
| `resources/branding/.gitkeep`, `resources/fonts/.gitkeep`, `resources/images/.gitkeep`, `resources/themes/.gitkeep` | Asset | Reserved resource directories; all empty. |
| `gen/schemas/acl-manifests.json` | Generated | Tauri's permission (ACL) manifest snapshot, produced by the build. |
| `gen/schemas/capabilities.json` | Generated | The application's capability set, currently `{}`: the app grants no extra plugin permissions. |
| `gen/schemas/desktop-schema.json`, `gen/schemas/windows-schema.json` | Generated | Schemas describing the configuration surface for tooling. Large files; do not edit. |
| [`schemas/README.md`](../schemas/README.md) | Docs | Notes that versioned configuration and content schemas belong here once they exist; the content schema is currently documented rather than machine-readable. |

## `tests/`

| Path | Role | Purpose and contents |
| --- | --- | --- |
| [`tests/unit/README.md`](../tests/unit/README.md) | Test | Explains that unit tests live in `#[cfg(test)]` modules beside the code, and that this directory is reserved for fixture-based tests. |
| [`tests/content/README.md`](../tests/content/README.md) | Test | Points at the content validation tests and describes the fixture cases worth adding later. |
| [`tests/integration/README.md`](../tests/integration/README.md) | Test | Describes the integration coverage in `src/app/session.rs` and how the interface is verified at runtime. |

## Generated, ignored or external files

These are not tracked, but they explain what a working checkout contains:

| Path | Produced by | Notes |
| --- | --- | --- |
| `target/` | `cargo build` / `cargo test` | Rust build output, including `target/debug/new-bearings.exe`. Ignored. |
| `frontend/dist/` | `npm --prefix frontend run build` | The built interface, embedded by the Rust binary. Ignored. |
| `frontend/node_modules/` | `npm install` | Frontend dependencies and the Tauri CLI. Ignored. |
| `frontend/node_modules/.tmp/` | `vue-tsc -b` | TypeScript build information. |
| `%APPDATA%\com.newbearings.desktop\app-state.json` | The running application | Learner data, outside the repository. A corrupt file is kept alongside it as `app-state.recovered-<timestamp>.json`. |

## Keeping this document current

When a file is added, removed or renamed, update its row here and, if it changes
the architecture or the workflow, the README as well. The changelog records the
change. A file reference that describes a file which no longer exists is worse
than no reference at all.
