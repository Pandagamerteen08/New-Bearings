# Unit tests

The unit tests live beside the code they test, in `#[cfg(test)]` modules inside
each Rust module under `src/`. Run them with `cargo test`.

That keeps a test next to the rule it protects: content validation in
`src/lessons/lessons.rs`, the completion rule in `src/progress/progress.rs`,
migration and recovery in `src/storage/storage.rs`, and the command surface in
`src/app/session.rs`.

This directory is reserved for fixture-based tests that need their own files
rather than inline values. It is empty today.
