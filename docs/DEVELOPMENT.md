# Development Guide

## Purpose

This guide explains the basic workflow for contributing to New Bearings.

The project is an early-stage Rust application. Commands and requirements may
change as its architecture, dependencies, and build process evolve.

## Prerequisites

A development environment generally requires:

- Git.
- A Rust toolchain appropriate for the repository.
- Cargo, installed with Rust.
- A supported operating system.
- A code editor or IDE.

Additional platform libraries may be required by the interface framework or
other dependencies.

Use the repository's actual toolchain configuration and dependency manifests
as the source of truth.

## Inspect the project

Before making changes, inspect the repository and its current state.

Useful commands include:

    git status
    git branch --show-current
    git log -5 --oneline
    cargo metadata --no-deps --format-version 1

Do not assume that the working tree is clean. Review existing changes before
editing files or switching branches.

## Check the Rust environment

Run:

    rustc --version
    cargo --version

If either command is unavailable, install or repair the Rust toolchain before
continuing.

Use the toolchain configuration supplied by the repository when one exists.
Do not upgrade the compiler solely because a newer version is available.

## Build and check

From the repository root, run:

    cargo check

This checks the package and its dependencies for compilation and type errors
without producing a normal release build.

To build the application, use:

    cargo build

For optimized output, use:

    cargo build --release

A successful build does not establish that all features work correctly or that
the application is ready for release.

## Formatting

Check Rust formatting with:

    cargo fmt --check

Apply formatting when appropriate with:

    cargo fmt

Review the resulting diff. Formatting should not be used as a reason to
overwrite or discard another contributor's unrelated changes.

## Tests

Run the available tests with:

    cargo test

Run targeted tests while developing a specific component when that is
supported by the repository.

Review test output and exit codes. A test command that could not start because
of an unavailable dependency or network connection has not demonstrated that
the code passes.

See [Testing](TESTING.md).

## Linting

When the project's dependencies and configuration support it, run:

    cargo clippy --all-targets --all-features -- -D warnings

Clippy configuration can vary. If this command fails because an optional
feature or target is unsupported, identify the cause before changing the
project's configuration.

Do not silence a useful warning without understanding the underlying issue.

## Recommended workflow

1. Review the issue or intended change.
2. Inspect relevant modules and tests.
3. Identify expected behavior and failure cases.
4. Make the smallest coherent implementation.
5. Add or update tests.
6. Format the code.
7. Run checks and tests.
8. Review the complete diff.
9. Update documentation if needed.
10. Commit the changes with a clear message.

## Reviewing changes

Use:

    git diff --check
    git diff
    git status --short

These commands help identify whitespace problems, inspect changes, and check
which files are modified.

Do not commit build output, credentials, private data, or unrelated generated
files.

## Dependencies

Before introducing a dependency, evaluate whether it is necessary and suitable.

Consider maintenance, licensing, security, platform support, compile time,
binary size, and the effect on application behavior.

Update dependency manifests and lockfiles through the appropriate tooling.
For Cargo-managed dependencies, do not edit Cargo.lock manually to simulate
a dependency resolution.

## Error handling

Use errors that provide useful context and preserve relevant failure information.

Avoid empty error handlers and silent fallback behavior that could make users
believe an operation succeeded when it failed.

User-facing messages should explain the problem in understandable language.
Diagnostic logs should avoid secrets and unnecessary personal information.

## Configuration

Do not hardcode private credentials into source code.

Use explicit configuration for behavior that needs to vary by environment.
Document defaults and validation rules where they exist.

Do not assume that a local configuration file belongs in version control.

## Cross-platform development

Avoid relying on Windows-specific path assumptions in shared application logic.

Use appropriate path-handling APIs and test behavior on each platform that the
project claims to support.

Compilation for a target is not sufficient proof that platform-specific
workflows operate correctly.

## Documentation changes

Update relevant documentation when changing:

- Public commands.
- Configuration.
- User-visible behavior.
- Data formats.
- Storage behavior.
- Security boundaries.
- Supported platforms.
- Testing requirements.
- Release procedures.

Do not describe future behavior as if it already exists.

## Git and GitHub

Before committing, inspect:

    git status
    git diff

Stage only the intended files.

Before pushing, verify the current branch and remote. Do not force-push or rewrite
shared history unless the operation is intentional and its consequences are
understood.

## Troubleshooting

### Cargo cannot download a dependency

Check the network connection, proxy settings, and registry availability.

A network or DNS failure does not automatically indicate a source-code defect.

### The compiler reports an error

Read the first meaningful diagnostic and follow its file and line references.
Later errors may be consequences of an earlier issue.

### Tests fail

Identify whether the failure is caused by application behavior, test
assumptions, environmental requirements, or unavailable services.

Do not remove a failing test merely to obtain a green build.

### A platform-specific build fails

Confirm that the required target and platform dependencies are installed.
Separate toolchain failures from errors in the project's own code.

## Related documentation

- [README](../README.md)
- [Architecture](ARCHITECTURE.md)
- [Testing](TESTING.md)
- [Content Format](CONTENT_FORMAT.md)
- [Security Policy](SECURITY_POLICY.md)
