# Security Policy

## Purpose

This document describes how security issues in New Bearings should be handled.

It complements the repository's top-level SECURITY.md file.

## Scope

Security concerns may involve:

- Unsafe file handling.
- Data exposure.
- Dependency vulnerabilities.
- Malformed content parsing.
- Unexpected network communication.
- Authentication or authorization if those features are introduced.
- Unsafe process execution.
- Insufficient isolation of learner-provided code.
- Data corruption or insecure migration.
- Sensitive information appearing in logs.
- Release artifacts containing secrets.

A defect is not automatically a security vulnerability, but reports should be
reviewed according to their potential impact.

## Reporting

Use the private reporting method described in SECURITY.md when one is available.

If a dedicated private channel has not yet been established, do not assume that
a public issue is confidential. Avoid publishing sensitive exploit details
until an appropriate disclosure process has been arranged.

Include the affected version or commit, relevant environment details, a clear
description of the behavior, and minimal reproduction steps.

Do not include real passwords, access tokens, private user records, or unrelated
personal information.

## Initial assessment

A maintainer should attempt to establish:

- Whether the report concerns this project.
- Which versions or components may be affected.
- The conditions needed to reproduce the issue.
- Potential confidentiality, integrity, and availability impacts.
- Whether an existing mitigation is available.
- Whether additional coordination is needed.

Assessment should distinguish confirmed facts from assumptions.

## Severity

Severity should reflect realistic impact, exploitability, required privileges,
user interaction, and the scope of affected systems.

A theoretical weakness with no credible impact should not automatically receive
the same priority as a reproducible flaw exposing private data.

The project may use a recognized severity framework if its needs justify one.

## Response and remediation

Where practical, maintainers should:

1. Acknowledge the report.
2. Investigate and reproduce the issue.
3. Identify affected code and versions.
4. Develop and test a correction or mitigation.
5. Coordinate disclosure when appropriate.
6. Publish relevant updates or advisories.
7. Document any remaining limitations.

Response times depend on severity, reproducibility, maintainer availability,
and the resources required to investigate the issue.

No fixed response-time guarantee is made by this initial policy.

## Coordinated disclosure

Reporters and maintainers should make reasonable efforts to avoid unnecessary
harm while a vulnerability is being investigated.

The appropriate disclosure date depends on the severity, availability of a
fix, release process, and risk to users.

Do not publish exploit code or sensitive details prematurely when doing so would
materially increase risk and a coordinated alternative is available.

## Dependencies

Security review should consider direct and transitive dependencies.

Use available tooling and advisories to identify known vulnerabilities, while
recognizing that automated reports can be incomplete or produce false positives.

Evaluate whether a finding affects the versions, features, and distribution
model actually used by the project.

## Learner-provided code

Any feature that executes code supplied by a learner requires particular care.

The design must address operating-system permissions, process isolation,
resource limits, filesystem access, network access, termination, and cleanup.

A child process is not inherently isolated from the user's account or files.

Do not advertise code execution as securely sandboxed without evidence supporting
that claim.

## Logs and private information

Security diagnostics should preserve useful evidence without unnecessarily
exposing sensitive content.

Redact credentials and private data before sharing logs in a public issue.

## Releases

Release preparation should consider relevant security fixes, dependency
changes, and accidental inclusion of secrets.

Security fixes should be documented without disclosing details that would
unnecessarily facilitate exploitation.

## Policy changes

This policy may evolve as the project establishes a formal security contact,
release process, and supported-version policy.

Any public reporting instructions must match the channels that actually exist.

## Related documentation

- [Security](../SECURITY.md)
- [Privacy Policy](../PRIVACY_POLICY.md)
- [Privacy and Data Handling](PRIVACY_AND_DATA_HANDLING.md)
- [Release Plan](RELEASE_PLAN.md)
- [Testing](TESTING.md)
