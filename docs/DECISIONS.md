# Architecture Decision Log

This file records important project decisions and open questions.

## Accepted direction

### Rust

Rust is the preferred language for the application core.

### Cross-platform target

Windows, macOS, and Linux are intended first-release platforms.

### Initial scope

Programming education is the initial subject. Additional subjects are future expansion.

### No AI tutor in the first release

The initial release will use rule-based personalization.

### Internet requirement

The initial application requires internet access for its overall experience. Full offline operation is not a first-release requirement.

### Local structured storage

SQLite is the preferred option for structured local user data.

### Privacy

Hidden telemetry and undisclosed data sharing are not part of the product direction.

## Open decisions

- GUI framework.
- Final application architecture.
- Database schema.
- Content-pack schema.
- Reminder implementation.
- Installer and updater design.
- Final performance targets.
- Backup and export behavior.

Decisions should be validated through prototypes and tests where practical.