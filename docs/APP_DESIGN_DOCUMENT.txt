RECALL
App Design Document (ADD)

Document status: Initial design baseline
Project name: Recall
Application type: Cross-platform desktop learning application
Initial release platforms: Windows, macOS, and Linux
Initial learning focus: Programming, coding, and scripting
Long-term vision: An accessible learning companion for a wide range of subjects
Primary development language: Rust, subject to initial framework validation
Distribution model: Free, open source, with optional paid services considered only in the future

---

1. Project Overview

Recall is a desktop application designed to make learning easier to understand, navigate, practice, and remember.

The initial release will focus on programming, coding, and scripting. Over time, Recall should expand to support other subjects through a flexible learning-content system.

Recall is intended for beginners, students, children, and people who find learning difficult for any reason. It should accommodate different learning preferences and needs without requiring users to disclose a medical diagnosis.

The application should make learning feel manageable. Users should be able to progress at their own pace, revisit difficult concepts, practice what they learn, and see their progress without feeling punished for forgetting something.

Core principle

Forgetting should not mean starting over.

Recall should help users return to previous concepts, understand them again, and continue learning.

2. Project Goals

Recall should:

- Provide an easy-to-navigate learning experience.
- Explain difficult concepts in understandable language.
- Offer practical exercises and small projects.
- Help users remember previously learned material.
- Adapt lessons and reviews using saved preferences and learning history.
- Provide meaningful progress tracking and non-monetary rewards.
- Support accessibility and personalization throughout the application.
- Keep users in control of their data and settings.
- Support Windows, macOS, and Linux from the beginning.
- Make additional learning subjects possible without redesigning the entire application.
- Remain free to use, with the source code published publicly on GitHub.

Success will be measured primarily by whether people actually use Recall and find it helpful.

3. Target Audience

Recall is designed for:

- People who are completely new to programming.
- Learners who need concepts explained in different ways.
- People who benefit from shorter lessons and frequent reviews.
- People who struggle with memory, attention, organization, or information overload.
- Children and students learning new skills.
- Experienced learners who want a structured place to practice and track progress.

The application must not assume that all users with similar learning needs learn in the same way.

Users must be able to personalize their experience without providing a diagnosis.

4. Supported Subjects and Languages

Initial subject

Programming education will be the first subject category.

The long-term language catalog may include:

- Rust
- Python
- JavaScript
- TypeScript
- C
- C++
- C#
- Java
- Go
- Additional programming and scripting languages as the project develops

HTML, CSS, and SCSS will not be standalone learning subjects in the initial scope.

The initial catalog should prioritize Rust and Python. JavaScript and TypeScript should follow as the learning-content system becomes established. Other languages can be introduced incrementally.

Recall should distinguish between a language being listed in its catalog and having a complete, tested learning curriculum. The application must never claim that a language is fully supported when its content is incomplete.

Learning resource requirements

Each supported language should have a curated collection of reliable resources, including:

- Official documentation.
- Beginner-friendly tutorials.
- Interactive exercises.
- Courses that teach through practical work.
- Small projects that learners can build.
- Reference materials for reviewing concepts.

Resources should be reviewed periodically so that broken links and outdated recommendations can be corrected.

5. First-Run Setup Wizard

Recall will include a detailed setup wizard the first time a user opens the application.

The wizard may use a large window with clearly organized sections. It should remain understandable rather than presenting every setting as a wall of options.

The setup should cover:

- Preferred learning subjects.
- Programming experience and familiarity.
- Learning goals.
- Preferred explanation depth.
- Lesson length and reading density.
- Text size and contrast.
- Relevant accessibility preferences.
- Review and reminder preferences.
- Other supported personalization settings.

Users should be able to skip optional questions and change their answers later.

Every setting configured during setup must also be available in the application's Settings area. Users must be able to enable, disable, or change supported features without repeating the entire wizard.

After setup, Recall should briefly explain where users can find their settings.

The wizard must not require users to provide a diagnosis or unnecessary personal information.

6. Lessons and Learning Experience

Recall should combine built-in learning content with links to external courses, tutorials, and practical projects.

A typical learning session may contain:

1. A short explanation of a concept.
2. A practical example.
3. A guided exercise.
4. An opportunity to try the concept independently.
5. Feedback or a review.
6. A summary of what was learned.
7. A recommendation for what to learn or review next.

Users should be able to choose between structured courses and more flexible, shorter learning sessions.

Lessons should avoid unnecessary complexity. When a learner struggles, Recall should be able to offer a simpler explanation, another example, or a review of prerequisite concepts.

Content should be written for its intended experience level and should not assume knowledge that has not been introduced.

7. Memory and Adaptive Learning

The first release will use rule-based personalization rather than an AI tutor.

Recall should use available learning history and user preferences to help determine:

- Which concepts may need another review.
- Which lessons the learner has completed.
- Which concepts the learner finds difficult.
- Which prerequisite topics may need attention.
- Which explanations and lesson formats the learner prefers.

The application should support spaced repetition, recaps, learning summaries, and a way to revisit previously learned material.

Progress must not be treated as permanently lost when a user struggles with a concept.

Recall should distinguish between completing a lesson and demonstrating understanding. A learner may finish a lesson but still benefit from additional practice.

Optional AI features may be considered in the future, but AI must not be required for the initial learning experience.

8. XP, Rewards, and Motivation

Recall will include a non-monetary experience-points system.

XP and rewards should encourage useful learning activities rather than pressure users to spend excessive time in the application.

Potential activities that may earn XP include:

- Completing lessons.
- Finishing exercises.
- Building small projects.
- Reviewing previous material.
- Reaching learning milestones.
- Demonstrating improvement over time.

The exact reward rules will be defined separately during feature planning.

The first release should not require purchases, paid boosts, or monetary rewards to progress. Rewards should support learning rather than replace it.

The design should avoid punishing users for missed days or forgotten material.

9. Accessibility and Personalization

Accessibility must be part of the core application design, not an optional feature added at the end.

Recall should provide configurable options for:

- Text size.
- Contrast and readable color schemes.
- Reading density.
- Explanation depth.
- Lesson length.
- Navigation and layout preferences.
- Review frequency.
- Optional reminders.
- Other practical accessibility needs identified during testing.

The interface should use clear labels, consistent navigation, readable typography, and understandable error messages.

Features should have sensible defaults. Advanced options should be available without overwhelming new users.

Reminder notifications must be opt-in.

10. Internet Connectivity

The initial release will require an internet connection for the application experience.

Individual features may have different connectivity requirements, and the application should communicate those requirements clearly.

Offline operation is not a first-release requirement. It may be reconsidered in a future version, potentially as an explicitly enabled option.

The architecture should avoid making future offline support unnecessarily difficult, but development must not expand into full offline support until that feature is approved.

11. Future Subject Expansion

Recall should be designed around a flexible learning-content system.

Future subjects should be introduced through organized content packs rather than requiring a complete application redesign.

A content pack may include:

- Subject metadata.
- Lessons and explanations.
- Exercises and assessments.
- Learning objectives.
- Prerequisite relationships.
- Review information.
- External learning resources.
- Required assets.

Content packs should use compact formats and avoid unnecessary duplicate files.

Recall should estimate and display the approximate download and installed storage requirements before a user installs a pack. Actual storage use may vary depending on the content and the operating system.

The initial application should establish the foundations for future subject expansion without requiring those future subjects to be developed immediately.

12. Application Architecture and Technology

Primary language

Rust is the leading choice for the application core because it offers strong performance, memory-safety guarantees, and cross-platform development capabilities.

However, the graphical interface framework must be evaluated before the architecture is finalized.

Candidate approaches include:

- A native Rust graphical interface.
- Tauri with a web-based interface.
- Other lightweight, actively maintained cross-platform options.

The choice should consider accessibility, maintainability, installation requirements, RAM use, disk space, and development difficulty.

The final choice should be validated with a small prototype on the supported platforms.

Data storage

SQLite is the preferred starting point for local structured data, including learning progress, preferences, and review history.

Learning content should be stored separately from personal progress where practical.

The data model must support versioning and safe database migrations so that future releases can change the schema without unnecessarily losing user data.

Modular design

The codebase should be divided into logical modules, such as:

- Application startup and lifecycle.
- User interface.
- Setup and settings.
- Learning content.
- Lesson presentation.
- Progress tracking.
- Review scheduling.
- XP and rewards.
- Storage and migrations.
- Notifications.
- Content-pack management.
- Platform-specific functionality.

These are logical responsibilities, not a requirement to create a separate DLL for every feature.

The initial application should avoid unnecessary runtime dependencies and excessive architectural complexity.

13. Privacy and Security

Recall should collect only the information necessary to provide its features.

Requirements include:

- No mandatory account for the initial release unless a later decision establishes a clear need.
- Clear disclosure of any information sent to external services.
- No hidden collection of learning activity.
- No unnecessary personal information in logs.
- Safe handling of downloaded content.
- Appropriate protection for user data.
- Clear backup and export options when implemented.
- No requirement to disclose a disability or diagnosis.

Future cloud synchronization or AI integrations must be evaluated separately, with explicit attention to consent, cost, security, and privacy.

14. Cross-Platform Support

Recall will target:

- Windows.
- macOS.
- Linux.

Cross-platform support is a first-release requirement, not merely a future aspiration.

The development process must include testing on all three operating systems.

Testing may use physical machines, virtual machines, remote environments, and automated CI runners where appropriate.

The developer's Windows computer alone cannot establish that the macOS version works correctly. Access to a Mac or an appropriate macOS build and test environment will be necessary before publishing a verified macOS release.

Linux testing should cover at least one clearly documented distribution initially, with broader compatibility added as practical.

Platform-specific installation instructions must be provided.

15. Installation and Distribution

Recall should eventually support multiple installation methods:

- A guided installer for Windows.
- A PowerShell installation method for Windows.
- Suitable installation packages for macOS.
- Suitable installation packages for Linux.
- A portable or archive-based distribution where practical.
- Source code and build instructions on GitHub.

Installation methods should be documented, and users should understand what each method changes on their computer.

PowerShell installation instructions must not encourage users to execute unverified remote scripts blindly. Any automated installer should be inspectable, versioned, and obtained from the official project source.

The application should support predictable updates and safe removal. User learning data should not be silently deleted during application updates or uninstallation.

16. Performance and Storage

Recall should be lightweight and responsive.

RAM usage and installed storage should be measured on actual builds rather than promised based on estimates alone.

The project should establish measurable performance targets after the graphical framework prototype is tested.

Optimization priorities include:

- Avoiding unnecessary background processes.
- Loading content when needed.
- Minimizing duplicate assets.
- Keeping dependencies under control.
- Avoiding unnecessary bundled runtimes.
- Providing storage estimates for content packs.
- Monitoring startup time, idle memory use, and active lesson memory use.

The application should not sacrifice usability or accessibility merely to achieve an arbitrary file-size target.

17. Development Workflow

Development will take place primarily in Visual Studio Code, with ChatGPT providing coding assistance.

The developer is learning programming while building Recall, so implementation instructions must be beginner-friendly and explicit.

Development guidance should provide:

- Exact file paths.
- Complete file contents when a file is created or replaced.
- Commands that can be copied and run.
- Clear explanations of what each step accomplishes.
- Troubleshooting steps for common errors.
- Verification steps after important changes.

The project should avoid unnecessary placeholders and unfinished features in code presented as complete.

Git and GitHub should be used to track changes, preserve working versions, and document progress.

Changes should be implemented incrementally, with working checkpoints rather than large, untested rewrites.

18. Testing and Quality Assurance

Testing should be built into development from the beginning.

The project should include appropriate tests for:

- Learning progress calculations.
- Review scheduling.
- XP and reward calculations.
- Settings persistence.
- Database migrations.
- Content-pack validation.
- Error handling.
- Cross-platform behavior.

The application should also undergo manual accessibility and usability testing.

Before a release, test installation, startup, settings changes, learning progress persistence, updates, and uninstallation on each supported operating system.

A successful Windows build alone is not sufficient for a cross-platform release.

19. Open Source and Community

The project source code will be published on GitHub.

The license will be selected after comparing suitable open-source licenses, including MIT and Apache-2.0.

The repository should include a clear project description, build instructions, contribution guidance, and documentation for third-party dependencies and their licenses.

Community feedback should help identify confusing interfaces, missing features, and learning barriers.

The project must not promise support or compatibility that has not been tested.

20. Initial Release Scope

The first release should focus on a stable, useful foundation.

Initial release requirements

- Cross-platform application foundation.
- Accessible main interface and navigation.
- First-run setup wizard.
- Settings that can be changed after setup.
- Initial programming learning content.
- Curated external learning resources.
- Lessons, examples, and exercises.
- Learning progress tracking.
- Basic rule-based personalization.
- Review and recap functionality.
- Non-monetary XP and rewards.
- Optional review reminders.
- Persistent storage for user progress.
- Clear error handling.
- Initial documentation and installation instructions.
- Tested builds for Windows, macOS, and Linux.

Not required for the first release

- An AI tutor.
- Full offline operation.
- Every planned programming language.
- Every future learning subject.
- Cloud synchronization.
- A complicated achievement economy.
- Advanced social or community features.
- A large ecosystem of third-party extensions.

These may be considered later, but they must not delay the core application without a clear justification.

21. Proposed Project Structure

The repository should begin with a clear, modular layout. The exact source directories may change after the GUI framework prototype is selected.

The proposed structure is:

Recall/
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── CHANGELOG.md
├── .gitignore
├── docs/
│   ├── APP_DESIGN_DOCUMENT.txt
│   ├── ARCHITECTURE.md
│   ├── DEVELOPMENT.md
│   ├── TESTING.md
│   └── PRIVACY.md
├── src/
│   ├── main.rs
│   ├── app/
│   ├── ui/
│   ├── setup/
│   ├── settings/
│   ├── learning/
│   ├── lessons/
│   ├── review/
│   ├── progress/
│   ├── rewards/
│   ├── storage/
│   ├── content_packs/
│   └── platform/
├── resources/
│   ├── icons/
│   ├── images/
│   └── themes/
├── content/
│   └── programming/
│       ├── rust/
│       └── python/
├── tests/
│   ├── unit/
│   ├── integration/
│   └── content/
├── scripts/
│   ├── setup-dev.ps1
│   ├── build.ps1
│   └── test.ps1
└── .github/
    └── workflows/

This is a starting layout, not a requirement to create empty folders for every future feature. The implementation should add directories as their responsibilities become real.

If Tauri is selected, the repository will also need its frontend source and configuration. If a native Rust GUI is selected, unnecessary web frontend directories should not be added.

22. Development Milestones

Milestone 1: Validate the technical foundation

Compare suitable Rust GUI frameworks and build a small prototype. Test accessibility, cross-platform feasibility, startup time, RAM use, and storage requirements.

Milestone 2: Establish the application structure

Set up the repository, build system, error handling, testing framework, and core application modules.

Milestone 3: Build the basic application

Implement the main window, navigation, settings, persistence, and first-run setup wizard.

Milestone 4: Implement learning

Add the initial content system, lessons, exercises, progress tracking, and curated learning resources.

Milestone 5: Implement retention and motivation

Add review scheduling, recaps, rule-based personalization, and the initial XP system.

Milestone 6: Validate cross-platform behavior

Test installation, operation, settings, data persistence, and updates on Windows, macOS, and Linux.

Milestone 7: Prepare the first release

Complete documentation, security and dependency reviews, accessibility checks, installation packages, and release testing.

Milestone 8: Expand carefully

Use real user feedback to improve the initial experience before adding more languages, subject packs, AI capabilities, or other major features.

23. Learning Resources for Development

These are starting points for learning the languages and building practical projects. The selection can be updated as development progresses.

Rust

- Official learning portal: https://www.rust-lang.org/learn
- The Rust Programming Language book: https://doc.rust-lang.org/book/
- Rustlings interactive exercises: https://rustlings.rust-lang.org/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/

Recommended starting path: Rustlings alongside the Rust book, followed by small command-line projects.

Python

- Official tutorial: https://docs.python.org/3/tutorial/
- Python beginner resources: https://www.python.org/about/gettingstarted/
- University of Helsinki Python Programming MOOC: https://programming-24.mooc.fi/

Recommended projects include a quiz application, a study tracker, and a small data-processing tool.

JavaScript

- MDN JavaScript Guide: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide
- The Modern JavaScript Tutorial: https://javascript.info/

Recommended projects include a quiz engine, a progress calculator, and interactive learning exercises.

TypeScript

- Official TypeScript Handbook: https://www.typescriptlang.org/docs/handbook/intro.html
- TypeScript exercises: https://typescript-exercises.github.io/

Recommended projects should focus on type-safe application logic and structured data.

C#

- Microsoft C# learning resources: https://learn.microsoft.com/en-us/dotnet/csharp/
- C# tutorials: https://learn.microsoft.com/en-us/dotnet/csharp/tour-of-csharp/tutorials/

Recommended projects include a command-line quiz, a task tracker, and a simple data-management application.

C++

- LearnCpp: https://www.learncpp.com/

Recommended projects include small command-line programs and progressively more complex data-structure exercises.

C

- Learn-C.org: https://www.learn-c.org/

Recommended projects include simple calculators, text-processing tools, and programs that introduce memory and data structures.

Java

- Official Java learning portal: https://dev.java/learn/

Recommended projects include a quiz program, a study tracker, and a basic object-oriented application.

Go

- A Tour of Go: https://go.dev/tour/
- Go tutorials: https://go.dev/doc/tutorial/

Recommended projects include a command-line utility, a small file-processing tool, and a simple HTTP service.

These languages will not all receive complete courses in the first release. The resources provide a foundation for future learning tracks.

24. Success Criteria

Recall will be considered successful when it:

- Works reliably on its supported operating systems.
- Is easy for beginners to navigate.
- Helps users learn concepts and return to them when needed.
- Provides useful lessons, exercises, and practical projects.
- Allows users to personalize their learning experience.
- Preserves learning progress reliably.
- Remains accessible without requiring users to disclose a diagnosis.
- Can expand to additional languages and subjects without major architectural rework.
- Attracts real users who continue using it because they find it valuable.

The project's long-term direction should be guided by actual learner feedback rather than feature count alone.

25. Guiding Rule

Build a small, reliable, accessible learning companion first.

Expand it only when the existing foundation is stable, understandable, and genuinely useful.

Recall should help people learn at their own pace, remember what matters, and feel capable of continuing even when learning is difficult.
