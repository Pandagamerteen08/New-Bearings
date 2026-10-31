# Architecture Decision Records

## Purpose

This document records significant decisions affecting New Bearings.

Decision records preserve the reasoning behind choices so future contributors
can understand why a direction was selected and when it may be appropriate to
reconsider it.

This file is an index and format guide. It does not imply that every potential
decision has already been made.

## When to record a decision

Record decisions that have meaningful consequences for:

- Application architecture.
- Supported platforms.
- Persistence and data migration.
- Content formats.
- Accessibility.
- Security boundaries.
- Programming-language integrations.
- Major dependencies.
- Distribution and packaging.
- Compatibility commitments.

Small implementation details usually do not need a formal decision record.

## Decision status

Use one of the following statuses:

- Proposed: under discussion and not yet approved.
- Accepted: selected for implementation or continued use.
- Superseded: replaced by a later decision.
- Rejected: considered and deliberately not selected.
- Deprecated: no longer recommended, but potentially still present.

A decision's status should reflect the current project position.

## Recommended record format

For a substantial decision, record:

### Title

Use a short description of the decision.

### Status

State the current status and date when appropriate.

### Context

Explain the problem, constraints, and relevant requirements.

### Options considered

Describe credible alternatives and their trade-offs.

### Decision

State what was selected.

### Consequences

Explain benefits, limitations, risks, and maintenance implications.

### Revisit conditions

Identify evidence or changed requirements that could justify reconsideration.

## Decision index

Add links to individual decision records here when the project begins maintaining
them as separate files.

No individual decision record is asserted by this initial index.

## Current architectural principles

The following are guiding principles rather than claims that every detail has
been formally decided:

- Rust is the primary application language.
- The application should be designed for cross-platform development.
- Accessibility and user-controlled personalization are important goals.
- Learning logic should remain testable independently of interface rendering
  wherever practical.
- Content should be validated before use.
- Storage behavior should be explicit.
- Significant dependencies should be evaluated rather than added casually.
- Untrusted learner code must not receive unrestricted access to the host
  system.
- Planned functionality must be distinguished from implemented functionality.

If a principle becomes a binding technical choice with important trade-offs,
create a dedicated decision record.

## Example: choosing a persistence strategy

A future persistence decision should compare realistic options against the
application's requirements.

Relevant criteria may include:

- Reliability.
- Portability.
- Migration complexity.
- Data recovery.
- Query requirements.
- Dependency maintenance.
- Testability.
- Backup behavior.
- User control over stored data.

Do not select a database or file format solely because it is popular.

The final decision should explain the chosen approach and any limitations.

## Example: running learner code

A decision about code execution should identify the threat model before choosing
an implementation.

Questions include:

- Is execution local or remote?
- Which operating-system account runs the code?
- Can code access the network?
- Can code read or modify files?
- How are CPU and memory limits enforced?
- How are processes terminated?
- What happens when a runtime is unavailable?
- How are toolchains installed and updated?

A normal subprocess is not automatically a security sandbox.

## Revisiting decisions

A decision should be reconsidered when assumptions change materially, new
evidence appears, or the implementation creates problems not anticipated during
the original discussion.

Do not rewrite historical reasoning to make an earlier decision appear
inevitable. Record the new decision and link it to the superseded one.

## Related documentation

- [Architecture](ARCHITECTURE.md)
- [Development](DEVELOPMENT.md)
- [Security Policy](SECURITY_POLICY.md)
- [Roadmap](ROADMAP.md)
