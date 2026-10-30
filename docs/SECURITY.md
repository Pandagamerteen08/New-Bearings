# Security Guidelines

## General principles

- Never commit secrets, passwords, API keys, or access tokens.
- Validate all imported and downloaded content.
- Avoid executing downloaded content.
- Use maintained dependencies and review their licenses.
- Handle file paths carefully.
- Avoid exposing personal information in error messages and logs.
- Preserve user data during upgrades.
- Treat network responses as untrusted input.
- Provide understandable error messages without exposing sensitive internals.

## Dependencies

Before introducing a dependency, evaluate its maintenance, license, security history, platform compatibility, and build impact.

Keep dependencies focused on actual requirements.

## Content packs

Content packs should be validated before installation.

Apply limits to manifest sizes, file sizes, nesting, and other resources as the format is defined.

## Reporting security issues

Do not publish working exploit details or private user data in ordinary issue reports. Establish an appropriate private reporting process before public release.
