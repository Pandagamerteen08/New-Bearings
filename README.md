# New Bearings

A Rust-first desktop learning companion, built with Tauri 2, Vue 3 and TypeScript.

Learning should adapt to the individual, rather than forcing every learner to
adapt to a single way of learning. New Bearings is an early but runnable
implementation of that idea: a small, honest learning application where the
rules, the content checks and the learner's progress live in Rust.

**Status: early development.** The application launches, teaches one complete
learning path, decides whether answers are correct, and remembers progress
across restarts. It does not yet have accounts, cloud sync, downloadable
content packs, or more than one subject.

---

## What works today

| Area | State |
| --- | --- |
| Application startup | Launches on Windows 11 through `tauri dev`. |
| Welcome and introduction | Rust owns the stage and the allowed transitions. |
| Starting to learn | `start_learning` moves the learner into the learning screens. |
| Learning dashboard | Lists installed paths, real progress counts, and the next unfinished lesson. |
| Path view | Shows the lessons of one path with their completion state. |
| Lesson reading | Explanation, code and note blocks rendered from content files. |
| Knowledge checks | Multiple choice, judged in Rust; the correct answer never reaches the interface. |
| Completion rule | A lesson is complete only when every check in it has been answered correctly. |
| Local progress | Saved to a versioned JSON file in the platform's application data directory. |
| Data recovery | Unreadable saved data is moved aside and reported, never silently deleted. |

Not implemented yet: accounts, synchronisation, review scheduling,
achievements, reminders, multiple subjects, and downloadable content packs. See
[docs/ROADMAP.md](docs/ROADMAP.md).

---

## Architecture

Rust owns the application. Vue renders it.

```
User interaction
  → Vue component
  → typed wrapper in frontend/src/backend/
  → Tauri command in src/main.rs
  → app::Session (authoritative state)
  → learning / progress / storage
  → snapshot returned to Vue
```

- **Rust owns behaviour.** Onboarding transitions, the learning catalog, the
  learner's position in it, answer judging, the completion rule and persistence
  are all Rust. `src/main.rs` only registers modules and forwards commands.
- **Vue owns presentation.** Components render the snapshot Rust returns and
  show the wording from content files. They do not decide what is allowed.
- **TypeScript is the bridge and the content.** `frontend/src/backend/` holds
  typed wrappers and the shape of each snapshot; `content/interface/` holds
  user-facing wording.
- **One lock, one state.** `app::Session` holds the setup stage, the learning
  screen, the progress and the storage handle behind a single mutex, so there is
  one authoritative value and no lock ordering to get wrong.

Answer keys are private fields in `src/lessons/lessons.rs` and are never part of
a serialized view; a test asserts that a lesson view contains no `answer` or
`explanation` key.

---

## Repository layout

```
src/
  main.rs                 entry point, module registration, Tauri commands
  app/
    app.rs                application identity
    session.rs            authoritative state and persistence coordination
  setup/setup.rs          onboarding stages and transition rules
  lessons/lessons.rs      content model, validation, answer-safe views
  learning/learning.rs    catalog, screens, answer judging, snapshots
  progress/progress.rs    completion rule and progress records
  storage/storage.rs      versioned, atomic local persistence
  ...                     the remaining module files are still empty stubs
content/
  interface/              editable interface wording (TypeScript)
  programming/rust/       learning content (JSON)
frontend/
  src/App.vue             welcome, introduction and the learning shell
  src/components/         LearningView.vue, LessonPane.vue
  src/backend/            typed wrappers around the Tauri commands
```

The other `src/*/` modules (`achievements`, `review`, `reminders`, …) exist as
one-line placeholders. They are not registered in `main.rs` and contain no
implementation yet.

---

## Development

### Prerequisites

- Rust (stable; the project uses edition 2024).
- Node.js and npm.
- The platform requirements for Tauri 2. On Windows that means the WebView2
  runtime and the MSVC build tools. On Debian/Ubuntu:

  ```bash
  sudo apt-get install libwebkit2gtk-4.1-dev build-essential pkg-config \
    libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev
  ```

### Commands

Run from the repository root:

```bash
# Run the desktop application (Vite and Rust are started for you)
./frontend/node_modules/.bin/tauri.cmd dev     # Windows
./frontend/node_modules/.bin/tauri dev         # macOS / Linux

# Rust checks
cargo fmt --check
cargo check --all-targets
cargo test
cargo clippy --all-targets -- -D warnings

# Frontend type-check and build
npm --prefix frontend run build
```

`tauri.conf.json` runs the build commands from the frontend directory, so use
the commands above instead of calling `cargo run` directly.

Release packaging is intentionally disabled (`bundle.active` is `false`); this
project does not produce an installer yet.

---

## Learning content

Content is JSON under `content/`, validated by Rust before use, and compiled into
the application with `include_str!`. A pack is registered in `EMBEDDED_PACKS` in
`src/learning/learning.rs`; `content/programming/rust/foundations.json` is the
only pack today.

Shape of a pack (`formatVersion` must match `SUPPORTED_FORMAT_VERSION`):

```json
{
  "formatVersion": 1,
  "paths": [
    {
      "id": "rust-foundations",
      "title": "Rust Foundations",
      "summary": "…",
      "subject": "Programming",
      "difficulty": "beginner",
      "lessons": [
        {
          "id": "first-program",
          "title": "Your first Rust program",
          "summary": "…",
          "minutes": 6,
          "sections": [
            { "kind": "explanation", "heading": "…", "body": "…" },
            { "kind": "code", "heading": "…", "language": "rust", "code": "…" },
            { "kind": "note", "body": "…" }
          ],
          "checks": [
            {
              "id": "entry-point",
              "prompt": "Which function does a Rust binary start in?",
              "options": ["start()", "main()", "run()", "init()"],
              "answer": 1,
              "explanation": "Rust calls main first."
            }
          ]
        }
      ]
    }
  ]
}
```

Validation rejects duplicate ids, empty text, missing sections or checks, and
answer indexes outside the offered options, reporting every problem with its
location such as `paths[0].lessons[1].checks[0].answer`. Content problems are
build defects: the tests fail and the interface shows a plain error instead of
pretending a lesson loaded.

---

## Where your data lives

Progress and the onboarding stage are stored in `app-state.json` in the
platform application data directory:

- Windows: `%APPDATA%\com.newbearings.desktop\app-state.json`
- macOS: `~/Library/Application Support/com.newbearings.desktop/app-state.json`
- Linux: `~/.local/share/com.newbearings.desktop/app-state.json`

Writes go to a temporary file that is then renamed, so an interrupted save
cannot leave a half-written file. A file that cannot be read is kept as
`app-state.recovered-<timestamp>.json` and the learner is told.

The application does not send anything over the network, does not require an
account, and contains no telemetry.

---

## Accessibility

The interface uses native buttons, radio inputs and fieldsets, keeps visible
focus outlines, announces answer results through `role="status"`, keeps every
control reachable by keyboard, and honours `prefers-reduced-motion`. Errors are
presented as text, never by colour alone.

---

## Testing

- `cargo test` covers the setup transitions, content validation, the completion
  rule, snapshot building, answer judging, storage round-trips, corrupt and
  version-mismatched data handling, and restart persistence. Run it for the
  current count.
- `npm --prefix frontend run build` type-checks every component and the content
  files, then builds the frontend.
- GitHub Actions runs the Rust checks on Windows, Linux and macOS, and builds the
  frontend. See the Actions tab for the current status rather than assuming it.

---

## Known limitations

- Only one learning path, in one subject, is included.
- Answers are recorded, but review scheduling and spaced repetition are not
  implemented.
- The learning screens have no search, filtering or settings yet.
- `LICENSE` contains placeholder text, while `Cargo.toml` declares MIT. The
  licence has not been finalised.
- Packaging, signing and auto-update are not configured.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md), [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md)
and [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md). Keep the Rust-first boundary:
if a change makes a decision, that decision belongs in Rust; the interface
renders the result.

## Licence

Not yet decided. See "Known limitations".
