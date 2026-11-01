# Integration tests

Integration coverage currently lives in `src/app/session.rs`. Those tests build a
real `Session` on a temporary directory and exercise the whole chain: command
method, content lookup, answer grading, progress, persistence and the snapshot
the interface would receive. They include restart persistence, a version 1 file
being migrated, a failed save being reported, and bookmarks surviving a reopen.

The interface itself is not covered by an automated test. It is verified by
running the application and driving the window through the WebView2 DevTools
protocol; the README describes how.

This directory is reserved for fixture-based integration tests that need files
the inline tests cannot express cleanly.
