# Recall

> **Forgetting should not mean starting over.**

Recall is a free, open-source, cross-platform desktop learning companion. It is built for people who want to learn to program — especially beginners, students, children, and anyone who finds learning difficult — and it is designed so that struggling or forgetting never feels like starting from zero.

Learning is hard for reasons that have nothing to do with intelligence: memory, attention, organization, information overload, and not knowing where to begin. Recall tries to make learning feel manageable. Lessons stay short, explanations can be re-read in simpler words, difficult concepts come back for review, and progress is not treated as lost when something is forgotten.

---

## Project status

**Early design and technical validation. There is no runnable build yet.**

Recall is currently a design document and a repository skeleton. The graphical interface framework has not been selected, so no source directories contain application code. This README describes the intended application; it is not a description of features you can install today.

The authoritative design document is [docs/APP_DESIGN_DOCUMENT.txt](docs/APP_DESIGN_DOCUMENT.md).

---

## Why Recall exists

Most learning tools assume you either already know how to learn, or that you will pay, or that you will not fall behind. Recall is designed around the opposite assumptions:

- **A forgotten concept is a scheduling problem, not a failure.**
- **Explanation depth should adapt to the person, not the topic.**
- **Accessibility is part of the design, not a settings screen bolted on at the end.**
- **Nobody should have to disclose a diagnosis to get a learning experience that fits them.**

Recall asks about preferences, never about medical conditions. Every question it asks can be skipped, and every answer can be changed later.

---

## Planned features (initial release)

| Area | What it will do |
| --- | --- |
| **First-run setup** | A guided wizard covering subjects, experience level, goals, explanation depth, lesson length, text size, contrast, review preferences, and reminders. Every setting is also available in Settings later. |
| **Lessons** | Short explanations, worked examples, guided exercises, independent practice, feedback, a summary, and a recommendation for what to do next. |
| **Curated resources** | Links to official documentation, beginner-friendly tutorials, interactive exercises, and small projects, reviewed periodically for dead links. |
| **Progress tracking** | The app separates *completing* a lesson from *demonstrating* understanding, so finishing something does not close the door on it. |
| **Review and recap** | Spaced repetition, recaps, learning summaries, and an easy way back to anything already covered. |
| **Rule-based personalisation** | No AI tutor in the first release. Recommendations come from saved preferences and learning history, using rules that can be inspected and explained. |
| **XP and rewards** | Non-monetary experience points for useful learning activity. No purchases, no streaks that punish you for missing a day. |
| **Local-first data** | Progress, preferences, and review history live in a local SQLite database, with versioning and safe migrations so updates do not cost you your history. |
| **Cross-platform** | Windows, macOS, and Linux from the first release, each tested before it is claimed to work. |

### Explicitly out of scope for the first release

An AI tutor, full offline operation, complete courses for every listed language, every future subject, cloud sync, an elaborate achievement economy, and social features. These may arrive later; they must not delay a stable, useful foundation.

---

## Subjects and languages

**Initial focus:** programming, coding, and scripting.

The long-term catalogue is expected to include Rust, Python, JavaScript, TypeScript, C, C++, C#, Java, and Go, with Rust and Python prioritised first.

Recall keeps two ideas separate on purpose:

- a language **listed** in the catalogue, and
- a language with a **complete, tested curriculum**.

A language will never be presented as fully supported when its learning content is incomplete. HTML, CSS, and SCSS are not standalone subjects in the initial scope.

---

## Platforms

Recall targets **Windows, macOS, and Linux** from the beginning:

- **Windows** — guided installer, plus a documented PowerShell installation method that does not ask anyone to blindly execute a remote script.
- **macOS** — a suitable installation package, verified on a real macOS build environment.
- **Linux** — packages for at least one clearly documented distribution, with broader compatibility added as practical.

A successful build on one operating system is not evidence that the others work. Platform-specific installation notes will be published alongside each release.

---

## Design and technology direction

Recall is intended to be **written in Rust**, because it offers performance, memory safety, and genuine cross-platform support.

The **GUI framework is deliberately undecided**. Candidate approaches are a native Rust interface, or Tauri with a web-based interface. The choice will be made by building a small prototype and comparing accessibility, maintainability, installation requirements, RAM use, disk space, and development difficulty — not by picking the most fashionable option.

Other direction the design commits to:

- **SQLite** for local structured data (progress, preferences, review history).
- **Learning content stored separately** from personal progress, so content can grow without touching user data.
- **Content packs** for future subjects, using compact formats, with approximate download and installed size shown before installation.
- **Modular responsibilities** — startup, UI, setup, settings, learning, lessons, review, progress, rewards, storage, content packs, notifications, platform — without turning each one into a separate library for its own sake.
- **An internet connection is required** in the first release. Features may differ in connectivity needs, and the app should say so plainly. Full offline support is a possible future version, and the architecture should not make it impossible — but it is not being built now.
- **No mandatory account.**

---

## Repository layout

This is a starting layout, adjusted once the GUI framework is chosen. Empty folders are not committed for features that do not exist yet.

```
Recall/
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── CHANGELOG.md
├── docs/                  design, architecture, development, testing, privacy
├── src/                   application source (Rust)
├── resources/             icons, images, themes
├── content/               learning content packs
├── tests/                 unit, integration, content validation
├── scripts/               developer setup, build, and test scripts
└── .github/workflows/     continuous integration
```

If Tauri is selected, the repository will gain its frontend source and configuration. If a native Rust GUI is selected, it will not.

---

## Building Recall

There is nothing to build yet. This section will contain real, copy-and-paste commands as soon as the first milestone produces a runnable prototype.

A practical toolchain for working on the design in the meantime:

1. Install Rust via [rustup](https://rustup.rs/).
2. Install [Visual Studio Code](https://code.visualstudio.com/) with the `rust-analyzer` extension.
3. Clone this repository and read the design document in `docs/`.

The project is being developed by someone learning to program while building it, so documentation is written to be explicit: exact paths, complete files, copyable commands, and a verification step after each change.

---

## Roadmap

1. **Validate the technical foundation** — compare GUI frameworks, prototype, measure accessibility, startup time, RAM, and disk use.
2. **Establish the application structure** — build system, error handling, testing, core modules.
3. **Build the basic application** — main window, navigation, settings, persistence, setup wizard.
4. **Implement learning** — content system, lessons, exercises, progress, curated resources.
5. **Implement retention and motivation** — review scheduling, recaps, rule-based personalisation, XP.
6. **Validate cross-platform behaviour** — installation, operation, settings, persistence, updates on all three systems.
7. **Prepare the first release** — documentation, security and dependency review, accessibility checks, installers, release testing.
8. **Expand carefully** — follow real user feedback before adding languages, subjects, AI, or anything else large.

**Guiding rule:** build a small, reliable, accessible learning companion first. Expand it only when the existing foundation is stable, understandable, and genuinely useful.

---

## Privacy

Recall should collect only what it needs to work.

- No mandatory account.
- No hidden collection of learning activity.
- Anything sent to an external service is disclosed.
- No unnecessary personal information in logs.
- No requirement to disclose a disability or diagnosis.

Progress data belongs to the person using the application. Future cloud sync or AI features would be evaluated separately, with explicit attention to consent, cost, security, and privacy.

---

## Contributing

Feedback about confusing interfaces, missing features, and learning barriers is the most valuable contribution at this stage — more than extra features. See [CONTRIBUTING.md](CONTRIBUTING.md).

The project aims to be a welcoming place for beginners. Nobody needs to explain their learning difficulties or diagnosis to participate.

---

## License

Recall will be open source. The license has **not been chosen yet** — MIT and Apache-2.0 are the candidates under consideration, and the decision will be recorded in a `LICENSE` file before the first release. Until then, treat the contents of this repository as all rights reserved.

---

## Documentation

- [App Design Document](docs/APP_DESIGN_DOCUMENT.txt) — the full design baseline
- [Contributing](CONTRIBUTING.md)
- [Changelog](CHANGELOG.md)

Architecture, development, testing, and privacy documents will be added as the corresponding milestones are reached.
