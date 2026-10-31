# Privacy and Data Handling

## Purpose

This document describes the privacy principles and data-handling practices
that New Bearings should follow as development progresses.

It is not a claim that every planned privacy control is already implemented.
Actual behavior must be verified against the application version being used.

## Privacy principles

The project should aim to:

- Collect only data necessary for an explained purpose.
- Keep learning records under meaningful user control.
- Avoid collecting sensitive information without a clear need.
- Explain when data is stored locally or transmitted elsewhere.
- Avoid embedding personal information in diagnostic logs.
- Make data deletion behavior understandable.
- Protect stored data against accidental exposure.
- Avoid making unsupported privacy or security guarantees.

## Categories of information

Depending on implemented features, the application may handle categories such
as the following. Listing a category here does not mean that the application
currently collects it.

### Learning content

This may include lesson material, exercises, and user-created notes.

### Progress information

This may include completed lessons, exercise attempts, review state, or other
records needed to support learning workflows.

### Preferences

This may include interface settings, accessibility options, and personalization
choices.

### Application diagnostics

Diagnostics may include error categories, software versions, or technical
details needed to investigate failures.

Diagnostic collection should be minimized and documented. Sensitive values
should not be included merely because they are convenient to log.

### Account or synchronization information

If account-based features or remote synchronization are introduced, they will
require explicit documentation of the data involved, the purpose of processing,
the relevant service, and the available user controls.

Do not assume that such features exist in the current build.

## Local storage

When information is stored locally, the application should use predictable
storage locations and handle permissions and errors carefully.

Local storage does not automatically mean that information is encrypted,
backed up, inaccessible to other software, or protected from a person with
access to the same operating-system account.

The implementation should document the actual storage mechanism and its
limitations.

## Network communication

Any feature that sends data outside the user's device should have a defined
purpose and a documented destination.

Avoid unexpected telemetry or hidden data transfers.

If remote services are introduced, explain what is sent, when it is sent, why
it is needed, and how users can control the relevant feature where applicable.

Network behavior should be testable and should fail in a way that does not
misrepresent an unsuccessful operation as successful.

## Personalization and sensitive information

Personalization should not require a medical diagnosis.

Avoid inferring sensitive traits from learning behavior unless a clearly
justified, lawful, and carefully reviewed feature requires it.

Prefer user-selected settings over opaque profiling.

If sensitive information is ever considered for collection, evaluate necessity,
retention, access controls, risks, and alternatives before implementing it.

## Logs and diagnostics

Logs should support troubleshooting without becoming an unnecessary record of
a person's private activity.

Avoid logging passwords, access tokens, private notes, full learning responses,
or other sensitive content without a clear and justified requirement.

Prefer structured error categories and redacted diagnostic details.

Ensure that user-facing error messages do not expose secrets or internal
filesystem details unnecessarily.

## Retention and deletion

Each stored data category should have an understandable retention policy.

Deletion should remove the relevant records from the active storage system
where the implementation supports that behavior.

The project should document any limitations involving backups, caches, logs,
or external services.

Do not claim that information has been permanently erased from every location
unless that claim can be verified.

## Backups and recovery

Important user records should not be exposed or destroyed by careless updates.

Where backup and restore functionality exists, document what it includes,
how it works, and its limitations.

Data migrations should be tested against representative previous formats.

Users should not be encouraged to rely on an experimental build as the sole
copy of irreplaceable information.

## Third-party components

Libraries and external services may have their own data-handling behavior.

Review relevant documentation and configuration before integrating a component
that introduces network access, telemetry, remote storage, or account services.

The privacy implications of dependencies should be considered alongside their
technical and licensing implications.

## User rights and applicable law

Privacy rights and obligations depend on the user's location, the service
model, the data involved, and applicable law.

The project should not promise a specific legal entitlement or compliance status
without evaluating the relevant requirements.

Requests concerning personal information should be handled through a documented
process when the project begins processing such information in a way that
requires one.

## Incident response

If a privacy incident is suspected, preserve relevant diagnostic information
without unnecessarily copying or exposing sensitive data.

Investigate what information may have been affected, the likely scope, and the
steps needed to contain and address the problem.

Follow applicable legal requirements and the project's security-reporting
process.

## Transparency

Documentation should reflect real application behavior.

Update this document when data collection, storage, synchronization, logging,
or deletion behavior changes.

Do not use this policy to promise controls that the application does not provide.

## Related documentation

- [Privacy Policy](../PRIVACY_POLICY.md)
- [Security](../SECURITY.md)
- [Security Policy](SECURITY_POLICY.md)
- [Architecture](ARCHITECTURE.md)
- [Testing](TESTING.md)
