# Contributing to Recall

Thanks for wanting to help. Recall is built for people who find learning difficult, and the project tries to be a place where nobody has to explain their learning difficulties, diagnosis, or background to take part.

## What is most useful right now

Recall is in design and technical-validation stage. There is no runnable build yet, so the most useful contributions are not code:

- **Feedback on the design.** Read the [App Design Document](docs/APP_DESIGN_DOCUMENT.txt) and point out anything confusing, unrealistic, or missing.
- **Reports about learning barriers.** If something in the design would make the application harder to use for a beginner, say so.
- **Resource corrections.** Broken links, outdated tutorials, or unsuitable recommendations in the learning-resource lists.
- **GUI framework experience.** First-hand notes on accessibility, packaging, memory use, or setup difficulty for the candidate approaches in the milestone 1 prototype.

## Reporting an issue

Please include:

- What you expected to happen.
- What actually happened.
- Your operating system and version, if the issue is platform-specific.
- The application version, once releases exist.

Never paste personal learning data, database files, or credentials into an issue.

## Code contributions

Code contributions become relevant once the GUI framework is selected and the first prototype exists. At that point this document will gain:

- Build and test instructions for each platform.
- Coding conventions and module boundaries.
- Requirements for tests accompanying new behaviour.

Until then, please open an issue to discuss a code change before writing it, so no one spends time on an approach the architecture is about to replace.

## Ground rules

- **Beginner-friendly communication.** The maintainer is learning to program while building Recall. Explain ideas in plain language and skip jargon where it is not needed.
- **Small, working changes.** Prefer incremental commits with a working checkpoint over large rewrites.
- **Do not weaken accessibility** to make an implementation easier.
- **Do not promise untested support.** Never claim a platform, language, or feature is supported when it has not been verified.
- **No diagnosis requirements.** No feature, question, form, or issue template may require a user to disclose a disability or diagnosis.

## Licensing of contributions

The project's open-source license has not been chosen yet (MIT and Apache-2.0 are the candidates). Please keep that in mind before contributing substantial code: the license needs to be settled first so contributions can be distributed clearly.
