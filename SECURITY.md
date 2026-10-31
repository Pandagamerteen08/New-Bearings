# Security

## Reporting a vulnerability

Please report suspected security vulnerabilities responsibly.

Do not publish passwords, access tokens, private user records, or sensitive
exploit details in a public issue.

Use a private security-reporting channel if one is available through the
project's repository or published contact information.

If no private channel is available, avoid assuming that a public issue is
confidential. Seek an appropriate way to contact a maintainer without publicly
exposing sensitive details.

## What to include

A useful report may include:

- Affected version or commit.
- Relevant operating system and environment.
- A clear description of the suspected vulnerability.
- Steps to reproduce the issue.
- The observed impact.
- A minimal proof of concept, when safe and necessary.
- Possible mitigations, if known.

Remove credentials and unrelated personal information before sharing logs or
test files.

## What not to do

Please do not:

- Access or modify another person's private data.
- Disrupt services or systems to prove a point.
- Publish exploit details prematurely when coordinated disclosure is possible.
- Include real credentials in a report.
- Claim a vulnerability is confirmed when the evidence is uncertain.

Testing should be limited to systems and data you are authorized to examine.

## What happens next

Maintainers should review reports, assess potential impact, investigate
reproducibility, and determine an appropriate remediation plan.

The time needed to respond depends on severity, available evidence, and
maintainer capacity. This project does not promise a fixed response time.

If a vulnerability is confirmed, maintainers may prepare a correction, release
an update, provide a mitigation, or publish a security advisory as appropriate.

## Scope

Relevant issues may involve the application, its content parser, dependencies,
data storage, logging, platform integration, or any implemented process
execution and network features.

A feature that is merely planned should not be assumed to exist.

## Safe development

Contributors should:

- Keep dependencies reasonably current.
- Review dependency security advisories.
- Avoid committing secrets.
- Validate untrusted input.
- Handle filesystem paths carefully.
- Avoid exposing private data in logs.
- Test error and recovery behavior.
- Treat learner-provided code as untrusted.
- Avoid claiming that a process is sandboxed without a verified isolation
  boundary.

## Supported versions

A formal supported-version policy has not been established by this document.

When releases become available, the project should clarify which versions
receive security fixes and how users can obtain updates.

## Related documentation

- [Security Policy](docs/SECURITY_POLICY.md)
- [Privacy Policy](PRIVACY_POLICY.md)
- [Privacy and Data Handling](docs/PRIVACY_AND_DATA_HANDLING.md)
- [Contributing](CONTRIBUTING.md)
- [Release Plan](docs/RELEASE_PLAN.md)
