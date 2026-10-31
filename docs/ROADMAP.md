# Roadmap

## Purpose

This roadmap describes the intended direction of New Bearings.

It is a planning document, not a promise that every item will be delivered by
a particular date. Priorities may change as implementation, testing, feedback,
and available resources evolve.

## Project vision

New Bearings aims to become a flexible learning companion that helps people
study, practice, and retain material in ways that suit their needs.

The initial emphasis is on programming education, with a longer-term intention
to support additional subjects.

The guiding principle is that learning should adapt to the individual rather
than forcing every learner into a single method.

## Stage 1: Project foundation

Potential priorities include:

- Keep the Rust project buildable.
- Establish understandable source organization.
- Improve error handling.
- Maintain useful development instructions.
- Add and maintain automated tests.
- Define architectural boundaries.
- Establish documentation and contribution conventions.
- Review dependency and license obligations.

Progress should be measured by verified repository state, not by the length of
the planning document.

## Stage 2: Core learning workflows

Potential priorities include:

- Clear lesson navigation.
- Structured learning material.
- Exercise presentation.
- Answer evaluation.
- Useful feedback.
- Basic progress tracking.
- Predictable recovery from errors.
- A coherent application setup flow.

Each workflow should be tested before being described as complete.

## Stage 3: Personalization

Potential priorities include:

- User-controlled learning preferences.
- Adjustable presentation options.
- Optional pacing preferences.
- Review behavior that can be understood and adjusted.
- Useful explanations of how preferences affect the experience.
- Ability to revise settings without losing unrelated progress.

Personalization should not require a diagnosis or assume that all people with
similar characteristics have identical needs.

## Stage 4: Accessibility

Potential priorities include:

- Keyboard navigation.
- Visible focus states.
- Readable text and layout.
- Clear feedback and error recovery.
- Screen-reader compatibility where supported.
- Sensible motion behavior.
- Improved scaling and resizing.
- Accessibility-focused testing and feedback.

The actual priority and scope of each item should be based on implementation
needs and evidence.

## Stage 5: Progress and review

Potential priorities include:

- Reliable progress records.
- Review scheduling where appropriate.
- Clear explanations of review recommendations.
- User control over review preferences.
- Safe persistence and recovery.
- Tests for progression and scheduling rules.

Progress indicators should not imply more certainty about learning mastery than
the underlying evidence supports.

## Stage 6: Content ecosystem

Potential priorities include:

- A documented content format.
- Content validation.
- Example lessons.
- Content packs.
- Clear content licensing guidance.
- Localization support if justified.
- Safe handling of external content.
- Compatibility and migration rules.

The format should evolve carefully so that content does not become needlessly
difficult to author or maintain.

## Stage 7: Programming environments

Potential programming subjects include:

- Python.
- C.
- C++.
- C#.
- Java.
- JavaScript.
- TypeScript.
- Rust.
- Go.

Specialized paths, including Unity-oriented C#, may be considered as the
application develops.

Listing a language does not mean its compiler, interpreter, debugger, runtime,
or execution environment is currently integrated.

Each language requires a realistic plan for tooling, diagnostics, exercises,
and safe execution where execution is supported.

## Stage 8: Safe code execution

Running learner-provided code may eventually be useful, but it introduces
significant security risks.

Before implementing execution, the project needs a documented threat model
and a design for isolation, resource limits, file access, network access,
process cleanup, and failure handling.

Running a process with ordinary application privileges is not equivalent to
a secure sandbox.

This work should not be rushed merely to expand the feature list.

## Stage 9: Persistence and portability

Potential priorities include:

- Documented storage behavior.
- Data format versioning.
- Tested migrations.
- Backup and recovery mechanisms where appropriate.
- Clear deletion behavior.
- Cross-platform storage paths.
- Safe handling of corrupt or incomplete data.

Any migration should be tested against realistic older data.

## Stage 10: Cross-platform support

The long-term direction includes support beyond Windows.

Platform support should be established through actual builds and workflow
testing on each claimed operating system.

Potential targets and their readiness should be documented individually.

## Stage 11: Distribution

Potential priorities include:

- Clear development and installation instructions.
- Versioned release artifacts.
- Dependency and license review.
- Release notes.
- Integrity verification where feasible.
- A defined support and vulnerability-reporting process.

Packaging should follow the needs of the selected interface framework and target
platforms rather than assuming that one installer format works everywhere.

## Prioritization principles

When comparing proposed features, consider:

1. User benefit.
2. Correctness and reliability.
3. Accessibility impact.
4. Security and privacy implications.
5. Maintenance cost.
6. Compatibility.
7. Testing requirements.
8. Dependency burden.
9. Available development time.

A feature with a compelling demonstration is not automatically more important
than reliability, accessibility, or data integrity.

## What is not promised

This roadmap does not promise specific delivery dates, a particular release
schedule, or completion of every listed capability.

Features may be revised, deferred, or removed if evidence shows that another
approach better serves the project.

## How to suggest a priority

Explain the problem, who it affects, the expected benefit, and any relevant
constraints.

Concrete examples and reproducible issues are especially helpful.

## Keeping the roadmap current

Update the roadmap when priorities change materially.

Move completed items only when their acceptance criteria have been met. If an
item is deferred or dropped, record that change where it helps preserve useful
project history.

## Related documentation

- [README](../README.md)
- [Architecture](ARCHITECTURE.md)
- [Decision Records](DECISIONS.md)
- [Release Plan](RELEASE_PLAN.md)
- [Testing](TESTING.md)
