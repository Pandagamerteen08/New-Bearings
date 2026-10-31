# Release Plan

## Purpose

This document describes the process New Bearings should follow when preparing
and publishing releases.

The project is in early development. The stages below describe a proposed
release process and should not be interpreted as evidence that a stable release
already exists.

## Release principles

A release should be:

- Identifiable by a clear version.
- Built from a known source revision.
- Tested against its intended platforms.
- Accompanied by accurate release notes.
- Distributed with relevant license and notice information.
- Clear about known limitations.
- Reproducible to the extent supported by the build environment.
- Supported by a reasonable way to report defects.

## Development stages

### Stage 1: Foundation

Establish a working project structure, build process, core error handling,
and initial tests.

Confirm that contributors can build and check the project using documented
instructions.

### Stage 2: Functional development

Implement and test the application's essential workflows.

Keep feature status accurate and avoid describing unfinished functionality as
production-ready.

### Stage 3: Pre-release validation

Before publishing a preview build, review:

- Build results.
- Test results.
- Known defects.
- Data storage and migration behavior.
- Privacy and security documentation.
- Accessibility concerns.
- License obligations.
- Platform-specific limitations.

### Stage 4: Release candidate

A release candidate should be feature-stable enough for focused validation.

Record issues that block release and establish how their resolution will be
verified.

### Stage 5: Stable release

A stable release should have a defined scope, verified artifacts, meaningful
release notes, and a support policy appropriate to the project's resources.

Do not call a build stable solely because it compiles successfully.

## Versioning

Choose a versioning scheme that communicates compatibility and release maturity.

If semantic versioning is adopted, document what constitutes a breaking change,
a backward-compatible feature, and a bug fix.

Use Cargo's package version as the source of truth for the Rust package unless
the project establishes a different documented arrangement.

Avoid maintaining conflicting version numbers across manifests and release
metadata.

## Release checklist

Before a release:

1. Confirm the intended source revision.
2. Review the working tree.
3. Run formatting checks.
4. Run compilation checks.
5. Run the relevant test suite.
6. Run configured lint checks.
7. Build the intended release artifacts.
8. Test the artifacts on the claimed platforms.
9. Review known issues.
10. Update the changelog.
11. Review documentation links.
12. Review license and third-party notices.
13. Confirm that no secrets or private test data are included.
14. Prepare release notes.
15. Publish only the artifacts that have actually been verified.

Not every project stage will support every checklist item. Any omitted step
should be understood rather than silently assumed complete.

## Continuous integration

CI can automate repeatable checks, such as compilation, formatting, and tests.

A successful CI run provides evidence only for the jobs and environments that
actually ran.

A failure caused by network access, DNS, unavailable package registries, or a
temporary service outage should be distinguished from a reproducible code
failure.

Platform coverage should be described accurately.

## Artifacts

Release artifacts should have clear names and version identifiers.

Where feasible, publish checksums or other integrity information and explain
how users can verify downloads.

Do not publish build artifacts containing local databases, credentials, private
configuration, or unrelated development files.

## Dependencies and notices

Review dependency licenses and relevant redistribution requirements before
shipping binaries.

Keep third-party notices accurate and synchronized with the components actually
distributed.

Do not assume that every dependency uses the same license or attribution rules.

## Security review

Review dependency changes, unsafe code, file access, process execution, network
behavior, and data migration risks as appropriate to the release.

Handle known vulnerabilities through the project's security process.

Do not disclose sensitive exploit details unnecessarily in public release notes
before users can reasonably obtain a fix.

## Rollback and withdrawal

If a release has a serious defect, assess whether to publish a corrected
release, withdraw the artifact, or provide a clear warning.

Document the affected versions and recommended user action.

If a release changes stored data, consider whether rollback is safe. Reverting
application binaries does not necessarily reverse a data migration.

## Release notes

Release notes should describe:

- The version and date.
- Major user-visible changes.
- Important fixes.
- Known limitations.
- Compatibility or migration notes.
- Security-relevant changes when disclosure is appropriate.
- Where to report problems.

Avoid claiming that every platform or workflow has been tested when it has not.

## Support expectations

Support duration and update frequency should be set according to actual
maintainer capacity.

Do not imply that every historical version will receive updates indefinitely.

## Related documentation

- [README](../README.md)
- [Changelog](../CHANGELOG.md)
- [Development](DEVELOPMENT.md)
- [Testing](TESTING.md)
- [Security Policy](SECURITY_POLICY.md)
- [Roadmap](ROADMAP.md)
