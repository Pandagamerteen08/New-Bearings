# Content tests

Content is validated by `parse_pack` in `src/lessons/lessons.rs`, and the tests
in that module already assert that every bundled pack is valid, that ids are
unique, and that the expected section and check kinds are used.

Two kinds of check belong here once they are needed:

- a broken pack fixture that proves a specific validation error is reported with
  the right location;
- a proposed pack that must keep parsing as the format evolves.

Small negative cases are currently written inline in the module's test block, so
this directory is empty.
