# Content Format

## Purpose

This document defines principles for structuring learning content in
New Bearings.

The format should support lessons and exercises while remaining readable,
validatable, versionable, and suitable for future content packs.

The exact serialized schema should be treated as authoritative only when it
matches the schema files and parser implemented in the repository.

## Design goals

A useful content format should be:

- Explicit about its version.
- Understandable to content authors.
- Validatable before use.
- Independent of unnecessary platform-specific assumptions.
- Suitable for incremental expansion.
- Able to identify lessons and exercises consistently.
- Clear about required and optional fields.
- Safe to parse as untrusted input.
- Compatible with meaningful error messages.

## Content is data

Learning content should not be treated as trusted executable code.

Parsing a lesson should not automatically run arbitrary commands, load native
libraries, or grant access to private files.

If executable examples are supported, they require a separate execution model
and security boundary.

## Suggested conceptual structure

A lesson may need information such as:

- A stable identifier.
- A title.
- A summary or introduction.
- A subject or topic.
- A content-format version.
- A sequence of learning sections.
- Optional exercises.
- Optional hints.
- Optional explanations.
- Attribution and licensing information where applicable.

These are conceptual fields, not a claim that the current schema requires
exactly these names or fields.

The repository's actual schema should define the canonical representation.

## Identifiers

Identifiers should be stable and unambiguous within their documented scope.

Do not use a lesson title as the only permanent identifier because titles may
change or may not be unique.

If identifiers are case-sensitive, that behavior should be documented and
validated consistently.

## Titles and descriptions

Titles should identify the material clearly.

Descriptions should explain the purpose of a lesson or exercise without relying
on hidden context.

Avoid embedding sensitive personal information in content metadata.

## Versioning

Content formats may evolve as the application gains capabilities.

A format version should identify the rules needed to interpret a document.

When a version is unsupported, the application should explain the problem
rather than silently interpreting the document using unrelated rules.

Changes that break older content should be documented and tested.

## Validation

Validation should distinguish structural errors from semantic problems.

Structural errors include missing required fields, invalid types, or malformed
serialized data.

Semantic errors include references to nonexistent lessons, invalid answer
configurations, or values that conflict with documented rules.

Validation should report enough context to locate the problem without dumping
sensitive data into logs.

## Text and markup

Any supported markup should be explicitly documented.

If raw HTML, embedded resources, or rich markup are not supported, the parser
should not silently treat them as trusted interface instructions.

Links and external resources should be handled according to explicit policy.

Content authors should not be able to override application navigation or
access private application data merely by placing special text in a lesson.

## Exercises and answers

Exercise definitions should clearly separate:

- The learner-facing prompt.
- Accepted answer rules.
- Feedback for correct answers.
- Feedback for incorrect answers.
- Optional hints.
- Any scoring or progression behavior.

Not every exercise type will use every field.

Answer evaluation should be deterministic where practical and testable
independently of interface rendering.

Avoid storing sensitive information in answer feedback or debugging output.

## Code examples

Code examples should identify their programming language when the format
supports language metadata.

Examples should distinguish displayed code from code that may be executed.

An execution feature must not infer permission to run code simply because a
content item contains a code block.

Before supporting execution, define isolation, resource limits, file access,
network access, cancellation, and failure handling.

## Localization

Content should avoid unnecessarily mixing interface labels with lesson text.

If localization is supported, the format should specify how language identifiers,
fallback behavior, and translated variants are represented.

Localization support should not be claimed until the relevant behavior exists.

## External content

Content packs from outside the trusted application build should be validated
before use.

The application should define acceptable file sizes, nesting depth, resource
references, and other relevant limits based on the parser and supported content.

Do not follow arbitrary local file references from untrusted content.

## Content licensing

Content authors must have permission to distribute submitted material.

Record required attribution and license information where applicable.

Code snippets, illustrations, fonts, and other assets may have different license
requirements from the surrounding lesson text.

## Schema files

The repository may contain machine-readable schemas under a schema directory.

Keep schemas and parser behavior synchronized.

Tests should cover valid examples, invalid examples, unsupported versions,
boundary cases, and meaningful validation errors.

## Compatibility

When changing the format, consider existing lesson files and content packs.

A migration should not silently discard material or misrepresent a failed
conversion as successful.

Document whether old content remains readable, needs migration, or is no longer
supported.

## Related documentation

- [README](../README.md)
- [Architecture](ARCHITECTURE.md)
- [Development](DEVELOPMENT.md)
- [Testing](TESTING.md)
- [Roadmap](ROADMAP.md)
