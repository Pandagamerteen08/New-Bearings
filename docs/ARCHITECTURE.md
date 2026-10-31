# Architecture

## Purpose

This document describes the intended architectural direction of New Bearings.

The project is written primarily in Rust and is intended to grow into a
cross-platform learning application. The architecture should remain understandable
to contributors while allowing the application to evolve.

This document is a design guide, not proof that every described boundary or
subsystem is already implemented.

## Architectural principles

The application should favor:

- Clear separation of responsibilities.
- Small, testable units of behavior.
- Explicit data ownership.
- Predictable error handling.
- Minimal unnecessary dependencies.
- Platform-specific behavior behind clear boundaries.
- Accessible user interaction.
- User-controlled personalization.
- Transparent storage behavior.
- Incremental implementation over speculative complexity.

## Conceptual layers

### User interface

The interface presents lessons, exercises, navigation, settings, progress,
and other learning workflows.

It should collect user input and present results without embedding every
business rule directly in rendering code.

The interface framework and its accessibility capabilities must be evaluated
against actual implementation needs.

### Application coordination

Application-level logic coordinates workflows such as opening a lesson,
starting an exercise, recording an answer, or navigating between screens.

This layer should define the order of operations and handle expected failures.

### Learning domain

The learning domain represents concepts such as lessons, exercises, attempts,
review schedules, and learning progress.

Domain behavior should be testable without requiring a running graphical
interface whenever practical.

### Content handling

Content handling is responsible for interpreting learning materials and
checking that they meet the supported content format.

Untrusted content should be validated before it can affect application state.

### Persistence

Persistence handles saving and loading information.

It should separate storage details from learning rules so that storage
implementation can evolve without rewriting every workflow.

Data migrations, error recovery, and file compatibility require explicit
design and testing.

### Platform integration

Platform integration contains behavior that differs between operating systems,
such as paths, notifications, application integration, and platform-specific
capabilities.

Common application logic should not assume that a Windows-specific path or
service exists on every platform.

### Shared services

Shared services may include settings, diagnostics, reminders, progress
coordination, and other cross-cutting functionality.

A shared service should have a defined responsibility and should not become a
general-purpose location for unrelated logic.

## Existing source organization

The repository has used modules associated with areas such as:

- accessibility
- achievements
- app
- content packs
- errors
- learning
- lessons
- navigation
- platform
- progress
- reminders
- resources
- review
- rewards
- settings
- setup
- storage
- UI

The exact responsibilities and implementation status of these modules should
be verified against the current source code before making architectural claims.

A module name alone does not establish that its functionality is complete.

## Data flow

A typical learning workflow should follow a clear sequence:

1. Load or receive a content item.
2. Validate its structure and supported features.
3. Present the content to the learner.
4. Receive an answer or action.
5. Evaluate the response using the appropriate domain rules.
6. Present understandable feedback.
7. Record progress if the workflow and user settings permit it.
8. Handle storage failures without falsely reporting success.

The implementation may differ, but the responsibilities should remain explicit.

## Error handling

Expected failures should be represented using meaningful error types and handled
at an appropriate boundary.

Examples include:

- Invalid content.
- Missing files.
- Unsupported content versions.
- Storage failures.
- Invalid settings.
- Unavailable platform capabilities.
- Failed migrations.
- Unexpected internal errors.

Error messages should help users recover without exposing secrets, sensitive
paths, or unnecessary internal details.

Avoid silently converting failures into successful results.

## Persistence boundaries

Domain logic should not need to know every detail of a database or file format.

Where practical, define clear operations for loading and saving domain data.

Persistence changes should consider:

- Existing user data.
- Schema or format versions.
- Interrupted writes.
- Corrupt records.
- Backup and recovery.
- Migration failure.
- File permissions.
- Platform-specific storage locations.

The exact persistence strategy must be documented once it is finalized.

## Content boundaries

Learning content should be treated as data, not executable application code.

Content parsing should reject malformed structures and enforce documented limits
where appropriate.

If external content can contain markup, links, embedded media, or code examples,
each capability needs its own validation and security considerations.

## Platform strategy

Cross-platform behavior should be built around explicit capabilities rather
than assumptions that all operating systems behave identically.

Platform-specific implementations should have testable boundaries and sensible
fallbacks where possible.

A platform should not be advertised as supported merely because the code
compiles for its target.

## Language-specific learning environments

New Bearings is intended to support programming education across several
languages over time.

Potential learning environments include Python, C, C++, C#, Java, JavaScript,
TypeScript, Rust, and Go. Unity-oriented C# instruction is also a possible
specialized learning path.

These environments should share suitable application services while respecting
the differences between programming languages, build systems, runtimes, and
toolchains.

The application should not pretend that one execution model or compiler setup
works for every language.

Running learner-provided code introduces security risks. Execution architecture
must be designed and reviewed before untrusted code execution is offered.

## Testing strategy

Prefer unit tests for domain rules and parsing logic.

Use integration tests for workflows that cross module boundaries.

Use platform-specific testing for behavior that depends on the operating system.

Use interface-level tests where they can reliably verify meaningful user
behavior, while recognizing that automated UI tests cannot replace manual
accessibility evaluation.

## Architectural decisions

Record significant decisions in [DECISIONS.md](DECISIONS.md).

Useful decision records explain the context, considered alternatives, trade-offs,
decision, and conditions that could justify revisiting it.

## Evolution

Architecture should evolve in response to real requirements and implementation
experience.

Avoid introducing complex abstractions solely for hypothetical future needs.
At the same time, avoid tightly coupling unrelated responsibilities when a
simple boundary would prevent repeated changes.

## Related documentation

- [README](../README.md)
- [Development](DEVELOPMENT.md)
- [Content Format](CONTENT_FORMAT.md)
- [Testing](TESTING.md)
- [Privacy and Data Handling](PRIVACY_AND_DATA_HANDLING.md)
- [Roadmap](ROADMAP.md)
