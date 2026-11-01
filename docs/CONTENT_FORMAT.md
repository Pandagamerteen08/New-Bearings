# Content format

This document describes the content format the application actually reads. It
matches `src/lessons/lessons.rs`, and the tests in that module fail if the
bundled packs stop satisfying it. If the two ever disagree, the code and its
tests are right and this document is out of date.

Current format version: **2**.

## Where content lives

```text
content/
  programming/rust/foundations.json   Rust Foundations
  web/fundamentals.json               How the Web Works
```

Packs are compiled into the binary with `include_str!` and listed in
`EMBEDDED_PACKS` in `src/learning/learning.rs`. There is nothing to download and
no content directory on disk at runtime.

## Pack

A pack is one JSON object:

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `formatVersion` | number | yes | Must equal `SUPPORTED_FORMAT_VERSION` (currently `2`). |
| `paths` | array | yes | At least one learning path. |

```json
{
  "formatVersion": 2,
  "paths": []
}
```

## Path

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `id` | slug | yes | Unique across every pack. Used in progress records and commands. |
| `title` | text | yes | Displayed name of the path. |
| `summary` | text | yes | One or two sentences describing the path. |
| `subject` | text | yes | Displayed as the path's category, for example `Programming`. |
| `difficulty` | `"beginner"`, `"intermediate"` or `"advanced"` | yes | How demanding the path is. |
| `tags` | array of text | no | Short labels; also searched. |
| `lessons` | array | yes | At least one lesson, in teaching order. |

Total reading time is the sum of the lessons' `minutes`; it is computed, not
authored.

## Lesson

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `id` | slug | yes | Unique within its path. |
| `title` | text | yes | Displayed name of the lesson. |
| `summary` | text | yes | What the lesson covers. |
| `minutes` | number | no | Declared reading time, used in totals. |
| `objectives` | array of text | yes | At least one; displayed as "What you will learn". |
| `sections` | array | yes | At least one teaching block. |
| `checks` | array | yes | At least one knowledge check; completion depends on them. |

## Sections

Every section has a `kind` tag. Unknown kinds are rejected by the parser.

### `explanation`

```json
{ "kind": "explanation", "heading": "A program starts at main", "body": "Every Rust binary begins…" }
```

### `code`

```json
{ "kind": "code", "heading": "The smallest program", "language": "rust", "code": "fn main() {}" }
```

`heading` is optional. `language` is a short label, currently only used for
future syntax highlighting, and `code` is rendered verbatim in a monospace block.

### `note`

```json
{ "kind": "note", "body": "println! is a macro…" }
```

### `steps`

```json
{ "kind": "steps", "heading": "What happens when this runs", "steps": ["Rust calls main.", "The statement runs."] }
```

Rendered as a numbered list. At least one step is required.

### `table`

```json
{
  "kind": "table",
  "heading": "Three ways to name a value",
  "columns": ["Writing", "What it means"],
  "rows": [["let total = 0;", "A fixed binding."]]
}
```

At least two columns and at least one row. Every row must have exactly as many
cells as there are columns.

### `mistake`

```json
{ "kind": "mistake", "body": "A common mistake is to write println without the exclamation mark." }
```

Rendered as a labelled aside. Use it for a specific error learners make, not for
general advice.

## Knowledge checks

Every check has an `id`, a `prompt` and an `explanation`. The explanation is
shown only after an attempt, and it is required: it is the part that teaches.

Ids must be unique within their lesson.

### `choice`

```json
{
  "kind": "choice",
  "id": "entry-point",
  "prompt": "Which function does a Rust binary start in?",
  "options": ["start()", "main()", "run()", "init()"],
  "answer": 1,
  "explanation": "Rust calls main first."
}
```

At least two options; `answer` is the zero-based index of the correct one and
must be in range.

### `trueFalse`

```json
{
  "kind": "trueFalse",
  "id": "macro-mark",
  "prompt": "The exclamation mark marks a macro call.",
  "answer": true,
  "explanation": "A plain name with parentheses calls a function."
}
```

The interface renders the two labels, which live in the interface wording file.

### `shortAnswer`

```json
{
  "kind": "shortAnswer",
  "id": "ownership-rule",
  "prompt": "Which rule gives every value one owner? Type one word.",
  "accepted": ["ownership", "the ownership rule"],
  "explanation": "Ownership decides when a value is freed."
}
```

Grading normalizes both the submitted text and each accepted wording before
comparing:

1. trim the ends,
2. lower-case the text,
3. collapse runs of whitespace to a single space,
4. drop trailing full stops.

An empty submission is rejected with `empty_answer` and does not count as a wrong
answer. A non-empty submission that matches nothing counts as a mistake and is
added to the review queue.

Because grading is exact after normalization, write prompts that make the
expected answer shape clear ("Type one word", "Type the selector") and list the
reasonable spellings in `accepted`. Avoid answers that differ only by punctuation
that normalization removes.

## Identifiers

`id` values are used as keys in saved progress and as command arguments, so they
are validated as slugs:

- ASCII lowercase letters, digits and hyphens only;
- no leading or trailing hyphen;
- no doubled hyphens;
- non-empty.

Path ids must be unique across all packs. Lesson ids must be unique within a
path. Check ids must be unique within a lesson. Packs are validated separately,
so collisions between packs are checked after they are merged.

## Validation

Every problem is collected and reported with a JSON-style location, so one pass
over a broken pack reports everything wrong with it:

```text
pack[0].paths[0].lessons[1].checks[0].answer: the answer index 5 is outside the 2 available options
```

The rules:

| Area | Rule |
| --- | --- |
| Pack | `formatVersion` must match; at least one path. |
| Path | Slug id, unique; non-blank title, summary and subject; at least one lesson. |
| Tags | Each tag must be non-blank. |
| Lesson | Slug id, unique in the path; non-blank title and summary; at least one objective; at least one section; at least one check. |
| Section | Required text for the kind; a `steps` section needs at least one step; a `table` needs two columns, one row, and matching row widths. |
| Check | Slug id, unique in the lesson; non-blank prompt and explanation; `choice` needs two options and an in-range answer; `shortAnswer` needs at least one accepted wording. |
| Content generally | Text fields must not be only whitespace. |

Validation failures are build defects, not learner errors: the tests fail, and
the interface shows a plain "content could not be read" message.

## Adding a pack

1. Write the JSON under `content/`, following this document.
2. Add a line to `EMBEDDED_PACKS` in `src/learning/learning.rs`:

   ```rust
   const EMBEDDED_PACKS: &[&str] = &[
       include_str!("../../content/programming/rust/foundations.json"),
       include_str!("../../content/web/fundamentals.json"),
       include_str!("../../content/your/new-pack.json"),
   ];
   ```

3. Run `cargo test`. The content tests validate the pack, check id uniqueness,
   and verify that every exercise kind can be answered with its own answer.
4. Run the application and work through the new lessons; a pack can be valid and
   still teach badly.

## Format history

| Version | Change |
| --- | --- |
| 1 | Paths, lessons, `explanation`/`code`/`note` sections, multiple-choice checks only. |
| 2 | Added lesson `objectives`, `steps`/`table`/`mistake` sections, `trueFalse` and `shortAnswer` checks, path `tags`. |

Version 1 packs are **not** accepted by the current parser; content is compiled
in, so upgrading a pack simply means adding the new required fields. Saved
learner data is a separate format and *is* migrated automatically; see the
README.

## Related documentation

- [README](../README.md)
- [Architecture](ARCHITECTURE.md)
- [Testing](TESTING.md)
