# Testing Strategy

## Purpose

This document describes how New Bearings should be tested as it develops.

Testing should provide evidence that the application behaves as intended.
No single test suite can establish that the entire application is correct,
secure, accessible, or suitable for every user.

## Testing principles

Tests should be:

- Repeatable.
- Relevant to defined behavior.
- Understandable to maintainers.
- Focused on meaningful failure cases.
- Maintained alongside implementation changes.
- Independent of unnecessary external services where practical.
- Honest about environmental limitations.

## Basic Rust checks

From the repository root, use the checks supported by the current project:

    cargo fmt --check
    cargo check
    cargo test

Where configured and supported, run:

    cargo clippy --all-targets --all-features -- -D warnings

These commands serve different purposes. Formatting, compilation, linting, and
tests should not be treated as interchangeable.

## Unit tests

Unit tests are appropriate for small, deterministic rules.

Potential areas include:

- Answer evaluation.
- Progress calculations.
- Content validation.
- Settings validation.
- Navigation rules.
- Review scheduling.
- Error classification.
- Serialization and deserialization.
- Achievement conditions, if implemented.

Test normal behavior, invalid input, boundary values, and failure cases.

## Integration tests

Integration tests should exercise interactions across meaningful boundaries.

Examples may include:

- Loading and validating a lesson.
- Recording an exercise attempt.
- Saving and reloading progress.
- Applying a settings change.
- Handling a failed storage operation.
- Migrating an older data format.

Only write tests for behavior that exists or is being implemented. Proposed
examples should not be mistaken for implemented features.

## Content tests

Content validation should be tested with:

- Valid content.
- Missing required fields.
- Invalid field types.
- Malformed serialized data.
- Unsupported format versions.
- Invalid references.
- Empty or unusually large values.
- Unexpected optional fields.
- Incorrect answer configurations.

Tests should verify that errors are useful and do not expose sensitive details.

## Storage tests

Where persistence exists, test:

- Successful writes and reads.
- Missing files or records.
- Invalid stored data.
- Interrupted or failed operations where reproducible.
- Migration behavior.
- Preservation of unrelated records.
- Handling of duplicate or conflicting identifiers.
- Recovery behavior where supported.

Avoid using real personal learning records in automated test fixtures.

## Error-handling tests

Verify that failures are surfaced rather than silently ignored.

Tests should confirm that the application does not report success when an
operation failed.

Check that user-facing errors are understandable and diagnostic details are
appropriately limited.

## Accessibility testing

Accessibility testing should include manual evaluation where appropriate.

Potential checks include:

- Keyboard-only operation.
- Focus order and focus visibility.
- Text readability and scaling.
- Contrast.
- Screen-reader behavior.
- Resizing.
- Error identification.
- Clear instructions.
- Motion-related settings.
- Predictable navigation.

Automated tests may help identify specific defects, but they cannot replace
testing with actual assistive technologies and representative workflows.

See [Accessibility](../ACCESSIBILITY.md).

## Cross-platform testing

For every platform the project claims to support, test relevant workflows on
that platform.

Compilation alone does not prove that installation, storage, notifications,
input handling, or interface behavior work correctly.

Record environmental limitations rather than claiming unsupported coverage.

## Regression testing

When fixing a reproducible defect, add a test that would detect the same
failure in the future when practical.

A regression test should verify the intended behavior, not merely the internal
implementation detail that happened to cause the original defect.

## Security testing

Depending on implemented features, consider:

- Malformed content.
- Unexpected file paths.
- Invalid configuration.
- Data leakage in logs.
- Dependency advisories.
- Process execution boundaries.
- Permission failures.
- Corrupt stored data.
- Unexpected network failures.

Security testing should be guided by an explicit threat model rather than by
the assumption that a successful build implies safety.

## Test isolation

Tests should avoid depending on a developer's personal settings or data.

Use temporary directories and controlled fixtures where appropriate.

Clean up resources created by tests and avoid modifying real application data.

Tests that require external services should explain their requirements and
should not silently skip critical validation without reporting that limitation.

## Continuous integration

CI should run repeatable checks on configured targets.

Review the actual jobs, operating systems, toolchains, and features covered by
each successful run.

A temporary registry outage, DNS failure, or network problem should be recorded
as an environmental failure when that is what the evidence shows.

A green CI badge should not be treated as proof of complete platform or
accessibility coverage.

## Manual test records

For significant workflows, record:

- The application version or commit.
- The operating system.
- The test steps.
- The expected behavior.
- The observed behavior.
- Any known limitations.

Avoid placing private user information in test reports.

## Test data

Use fictional or synthetic information in fixtures.

Do not commit real credentials, private notes, or personal learning records.

Test data should be small enough to understand and broad enough to cover
important edge cases.

## Failed tests

Do not remove or weaken a test merely because it fails.

Determine whether the failure reflects a genuine defect, an outdated test,
an environment problem, or an incorrect assumption.

Document the reason for any intentional exclusion or skip.

## Release readiness

Before release, review relevant automated tests, manual checks, known defects,
and platform-specific results.

The required test scope should match the release's intended audience and risk.

## Related documentation

- [README](../README.md)
- [Development](DEVELOPMENT.md)
- [Architecture](ARCHITECTURE.md)
- [Accessibility](../ACCESSIBILITY.md)
- [Release Plan](RELEASE_PLAN.md)
- [Security Policy](SECURITY_POLICY.md)
