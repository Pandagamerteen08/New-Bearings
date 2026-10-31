# Contributing to New Bearings

Thank you for considering a contribution.

New Bearings is an early-stage learning application. Contributions may include
code, documentation, testing, accessibility feedback, content-format proposals,
bug reports, and careful review of design decisions.

The goal is to improve the project without introducing unnecessary complexity
or making unsupported claims about its capabilities.

## Before starting

Read the following documents when relevant:

- [README](README.md)
- [Development Guide](docs/DEVELOPMENT.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Testing](docs/TESTING.md)
- [Content Format](docs/CONTENT_FORMAT.md)
- [Decision Records](docs/DECISIONS.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Security](SECURITY.md)

Check the current repository state before beginning a substantial change.

An idea described in the roadmap is not necessarily approved for immediate
implementation. For large features, discuss the intended behavior and scope
before investing significant effort.

## Types of contributions

### Bug reports

A useful bug report explains:

- What you were trying to do.
- What happened.
- What you expected to happen.
- Whether the problem is repeatable.
- The operating system and application version, if known.
- Relevant error output with secrets removed.

Keep reports focused. Separate unrelated problems into separate reports.

### Documentation

Documentation contributions should be accurate, readable, and consistent with
the implementation.

Clearly distinguish current behavior from proposed behavior.

When documenting a command, configuration option, or file format, verify it
against the actual project where possible.

### Code

Code changes should solve a defined problem and fit the project's architecture.

Prefer focused changes over unrelated refactoring. Avoid adding dependencies
when the same requirement can be met clearly and safely without them.

Do not introduce placeholder implementations that appear functional but silently
discard data, bypass validation, or claim success without completing an action.

### Accessibility

Accessibility reports and improvements are welcome.

Describe the obstacle, its impact, and the behavior that would make the
workflow more usable. Do not assume a single solution will suit every user.

### Learning content

Content contributions should be accurate, clearly structured, and appropriate
for the intended learner.

Respect copyright, licenses, attribution requirements, and the rights of
content authors.

## Development workflow

1. Review the current code and documentation.
2. Identify the problem and intended outcome.
3. Check for existing related work.
4. Make a focused change.
5. Add or update tests when appropriate.
6. Run relevant formatting, linting, and test commands.
7. Review the complete diff.
8. Update documentation if behavior or project expectations changed.
9. Submit the contribution with a clear explanation.

Do not assume that a command succeeded simply because it started running.
Review its exit status and output.

## Rust conventions

Follow the Rust edition and toolchain configuration used by the repository.

Use `cargo fmt --check` to check formatting when the project supports it.

Use `cargo check` to identify compilation and type-checking problems.

Use `cargo test` to run the available test suite.

Use `cargo clippy --all-targets --all-features -- -D warnings` when the
repository's current configuration supports this command without unrelated
failures.

If a command fails because of an unavailable network, platform dependency, or
toolchain issue, report the actual failure instead of claiming that the code
failed or passed without evidence.

## Testing expectations

Tests should exercise behavior, not merely reproduce implementation details.

When fixing a bug, consider a regression test.

For changes affecting storage, privacy, accessibility, content parsing, or
cross-platform behavior, consider relevant edge cases.

Document any test that could not be run and explain why.

See [Testing](docs/TESTING.md).

## Commit messages

Write concise commit messages that describe the change.

Examples:

- `Document accessibility principles`
- `Add content validation tests`
- `Clarify local storage behavior`
- `Fix progress calculation`

Avoid commit messages that conceal the nature or scope of a change.

## Pull requests

A pull request should explain:

- The problem or goal.
- The approach taken.
- The files or systems affected.
- Tests performed.
- Known limitations.
- Any documentation updates.
- Any migration or compatibility implications.

Include screenshots when they help explain an interface change and do not expose
private information.

Do not claim tests passed unless they actually ran successfully.

## Dependencies

Before adding a dependency, consider:

- Whether it is necessary.
- Its maintenance status.
- Its license and redistribution terms.
- Its security history.
- Its effect on compilation and distribution.
- Whether it introduces network access or additional data collection.
- Whether it behaves consistently across supported platforms.

Record meaningful architectural choices in docs/DECISIONS.md.

## Security

Never commit passwords, tokens, private keys, personal records, or production
credentials.

Do not publicly disclose an unpatched vulnerability before coordinating an
appropriate response.

Follow SECURITY.md and docs/SECURITY_POLICY.md for security reporting guidance.

## Scope management

Keep each contribution reasonably focused.

A large feature may need to be split into design, implementation, tests, and
documentation changes.

Avoid changing unrelated files simply to make a patch look more comprehensive.

## Review standards

Reviewers should explain requested changes and distinguish correctness problems
from personal preferences.

Contributors should respond constructively and may ask for clarification when
feedback is ambiguous.

Approval does not replace required testing or maintainer release decisions.

## Licensing and originality

Only contribute material that you have the right to submit.

Identify third-party code, copied content, generated assets, and external
dependencies when their origin or licensing matters.

Submitting a contribution does not authorize the project to ignore the license
under which that contribution is provided.

## Maintainer discretion

Maintainers may request revisions, defer changes, or decline contributions
that do not fit the project's goals, quality expectations, available resources,
or legal obligations.

A declined contribution should not be treated as a judgment of the contributor's
personal worth or programming ability.

## Thank you

Thoughtful reports, clear documentation, reproducible tests, and constructive
feedback can be valuable even when they do not involve new code.

## Related documentation

- [README](README.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Development](docs/DEVELOPMENT.md)
- [Testing](docs/TESTING.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Security](SECURITY.md)
