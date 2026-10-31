# New Bearings

A personalized, accessible learning companion built with Rust.

> Learning should adapt to the individual, not force the individual to adapt to a single way of learning.

[![Repository](https://img.shields.io/badge/repository-GitHub-181717?logo=github)](https://github.com/Pandagamerteen08/New-Bearings)
[![Language](https://img.shields.io/badge/language-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![Development](https://img.shields.io/badge/status-Early%20Development-yellow)](#1-project-status)
[![Platform](https://img.shields.io/badge/initial%20platform-Windows-blue?logo=windows)](#14-platform-support)
[![License](https://img.shields.io/badge/license-Under%20Review-lightgrey)](#27-licensing-and-third-party-software)

**New Bearings is a working project name.** The name may change before the application reaches a stable public release.

New Bearings is an early-stage software project exploring a more flexible approach to learning and retaining knowledge. Its long-term vision is to provide a learning environment that can adapt to different preferences, abilities, subjects, and study circumstances without requiring every learner to follow the same rigid process.

The initial emphasis is programming education. The broader vision extends to other subjects, practical skills, technical concepts, and knowledge that benefits from repeated practice.

The project is being developed in Rust, with Windows as the initial development platform and cross-platform support as a long-term objective.

New Bearings is intended to be a learning companion, not a replacement for teachers, educational institutions, textbooks, professional instruction, or a learner's own judgment.

The project is in active early development. This README describes the product vision, development principles, intended architecture, planned capabilities, repository conventions, and contribution expectations. It does not mean every capability described here has already been implemented.

---

## Table of Contents

<<<<<<< HEAD
- [1. Project Status](#1-project-status)
- [2. Quick Overview](#2-quick-overview)
- [3. Why New Bearings Exists](#3-why-new-bearings-exists)
- [4. Project Philosophy](#4-project-philosophy)
- [5. Core Principles](#5-core-principles)
- [6. Goals and Non-Goals](#6-goals-and-non-goals)
- [7. Intended Audience](#7-intended-audience)
- [8. Learning Should Adapt to the Learner](#8-learning-should-adapt-to-the-learner)
- [9. Personalization and Learning Preferences](#9-personalization-and-learning-preferences)
- [10. Accessibility and Inclusive Design](#10-accessibility-and-inclusive-design)
- [11. Supported Subjects and Learning Environments](#11-supported-subjects-and-learning-environments)
- [12. Programming Language Support](#12-programming-language-support)
- [13. Planned Feature Catalogue](#13-planned-feature-catalogue)
- [14. Platform Support](#14-platform-support)
- [15. Technology Stack](#15-technology-stack)
- [16. Architecture Overview](#16-architecture-overview)
- [17. Module Responsibilities](#17-module-responsibilities)
- [18. Learning Content and Lesson Design](#18-learning-content-and-lesson-design)
- [19. Practice, Recall, and Retention](#19-practice-recall-and-retention)
- [20. Progress Tracking](#20-progress-tracking)
- [21. Achievements and Motivation](#21-achievements-and-motivation)
- [22. Reminders and Study Routines](#22-reminders-and-study-routines)
- [23. Navigation and User Experience](#23-navigation-and-user-experience)
- [24. Settings and Configuration](#24-settings-and-configuration)
- [25. Storage and User Data](#25-storage-and-user-data)
- [26. Privacy and Data Handling](#26-privacy-and-data-handling)
- [27. Licensing and Third-Party Software](#27-licensing-and-third-party-software)
- [28. Security and Responsible Development](#28-security-and-responsible-development)
- [29. Error Handling and Reliability](#29-error-handling-and-reliability)
- [30. Performance and Resource Use](#30-performance-and-resource-use)
- [31. Installation and Getting Started](#31-installation-and-getting-started)
- [32. Development Environment](#32-development-environment)
- [33. Building the Application](#33-building-the-application)
- [34. Running Tests](#34-running-tests)
- [35. Testing Strategy](#35-testing-strategy)
- [36. Debugging and Troubleshooting](#36-debugging-and-troubleshooting)
- [37. Repository Structure](#37-repository-structure)
- [38. Documentation Guide](#38-documentation-guide)
- [39. Content Packs and Extensibility](#39-content-packs-and-extensibility)
- [40. Versioning and Compatibility](#40-versioning-and-compatibility)
- [41. Roadmap and Development Priorities](#41-roadmap-and-development-priorities)
- [42. Contributing](#42-contributing)
- [43. Code Style and Engineering Standards](#43-code-style-and-engineering-standards)
- [44. Git and Repository Workflow](#44-git-and-repository-workflow)
- [45. Continuous Integration](#45-continuous-integration)
- [46. Community Standards](#46-community-standards)
- [47. Reporting Bugs and Requesting Features](#47-reporting-bugs-and-requesting-features)
- [48. Security Issue Reporting](#48-security-issue-reporting)
- [49. Frequently Asked Questions](#49-frequently-asked-questions)
- [50. Glossary](#50-glossary)
- [51. Project Limitations](#51-project-limitations)
- [52. Release Readiness](#52-release-readiness)
- [53. Long-Term Vision](#53-long-term-vision)
- [54. Final Project Principles](#54-final-project-principles)
=======
**Early design and technical validation. There is no runnable build yet.**

Recall is currently a design document and a repository skeleton. The graphical interface framework has not been selected, so no source directories contain application code. This README describes the intended application; it is not a description of features you can install today.

The authoritative design document is [docs/APP_DESIGN_DOCUMENT.md](docs/APP_DESIGN_DOCUMENT.md).
>>>>>>> 4107a34c1e701bedd32b303a98cf8d078b6f8b00

---

## 1. Project Status

New Bearings is an early-stage software project.

The current priority is to establish a maintainable application foundation and develop the project incrementally.

The existence of a source file does not automatically mean the corresponding feature is complete.

The existence of a module does not automatically mean its intended behavior is fully implemented.

A documented goal does not automatically mean that the application can perform that goal today.

A successful compilation does not automatically mean that every feature behaves correctly.

A successful test run does not automatically establish compatibility with every platform or environment.

For these reasons, project status should be reported precisely.

### 1.1 Current development position

- The application is being developed in Rust.
- Windows is the initial development platform.
- Cross-platform compatibility is a long-term objective.
- Programming education is the initial subject focus.
- The broader product vision includes other learning subjects.
- The application is intended to support different learning preferences.
- Accessibility is a design priority.
- Personalization should remain optional and user-controlled.
- The project is not intended to diagnose medical or learning conditions.
- The project is not a substitute for professional educational support.
- The codebase contains distinct application modules.
- The repository includes development and project documentation.
- The repository includes a continuous integration workflow.
- The project is intended to remain maintainable as it grows.
- The public application name remains provisional.
- A stable public release has not been established by this README.

### 1.2 Feature status terminology

Use the following terminology when discussing project capabilities.

| Status | Meaning |
| --- | --- |
| Implemented | The feature exists and its expected behavior has been verified to an appropriate standard. |
| Partially implemented | Some behavior exists, but important functionality or verification remains incomplete. |
| In progress | Development is actively underway. |
| Planned | The feature is part of the intended direction but has not been completed. |
| Proposed | The feature is being considered and is not yet an approved commitment. |
| Blocked | A dependency, unresolved decision, or technical problem prevents progress. |
| Unsupported | The project intentionally does not provide the capability in the relevant context. |
| Deprecated | The capability remains present temporarily but is scheduled for removal or replacement. |

These terms should be used consistently across documentation, issue discussions, release notes, and feature tracking.

### 1.3 What this README promises

This document explains the project's direction and development expectations.

It does not promise that every proposed feature is available.

It does not promise that every listed programming language already has a complete learning environment.

It does not promise that the application has passed a formal accessibility audit.

It does not promise that every platform has been tested.

It does not promise that user data has undergone a formal security audit.

It does not establish a legal guarantee of compatibility with every third-party library.

It does not replace the technical documentation for a specific implementation.

The authoritative state of a capability must be determined from the current code, its tests, and verified behavior.

### 1.4 Why accuracy matters

Educational software influences how people spend their time and how they judge their own progress.

Incorrect claims about learning results can mislead users.

Incorrect claims about privacy can create misplaced confidence.

Incorrect claims about accessibility can exclude the people the software intends to help.

Incorrect installation instructions can waste development time.

Incorrect compatibility claims can lead users to depend on unsupported configurations.

New Bearings should favor accurate, qualified explanations over impressive but unverified promises.

**Related documentation:** [Architecture](docs/ARCHITECTURE.md), [Roadmap](docs/ROADMAP.md), [Testing](docs/TESTING.md), [Release Plan](docs/RELEASE_PLAN.md).

[Back to top](#new-bearings)

---

## 2. Quick Overview

New Bearings is intended to help people learn, practice, and retain knowledge through a flexible digital learning environment.

Its initial focus is programming.

Its long-term direction is broader than a single programming language or educational subject.

The application should help learners understand material rather than merely complete a sequence of activities.

### 2.1 At a glance

| Category | Direction |
| --- | --- |
| Project name | New Bearings, provisional |
| Primary implementation language | Rust |
| Initial development platform | Windows |
| Long-term platform direction | Cross-platform |
| Initial learning focus | Programming |
| Broader learning scope | Multiple subjects and practical skills |
| Personalization | Optional and user-controlled |
| Accessibility | Core design consideration |
| User data | Clear handling and documented boundaries |
| Development style | Incremental, tested, maintainable |
| Release status | Early development |
| Public release maturity | Not established |

### 2.2 Intended experience

A learner should be able to approach a subject without needing to know exactly how to organize the learning process.

The application should eventually help the learner identify a useful starting point.

It should present material in understandable units.

It should offer opportunities to practice.

It should help distinguish familiarity from demonstrated understanding.

It should support reviewing previously studied material.

It should make progress understandable.

It should let learners adjust appropriate aspects of the experience.

It should avoid treating every learner as though they have identical needs.

### 2.3 What the application is not

New Bearings is not intended to be a generic chatbot presented as a complete education system.

It is not intended to be a replacement for programming documentation.

It is not intended to make unsupported claims about intelligence or learning ability.

It is not intended to require users to disclose diagnoses.

It is not intended to force a single study method on every learner.

It is not intended to turn every aspect of learning into a competition.

It is not intended to make users feel that a missed day invalidates previous work.

It is not intended to claim mastery solely because a lesson has been opened.

It is not intended to promise employment, certification, or professional competence.

It is intended to become a practical learning companion whose value comes from useful learning workflows, clear explanations, thoughtful practice, and reliable progress information.

**Related documentation:** [Roadmap](docs/ROADMAP.md), [Content Format](docs/CONTENT_FORMAT.md), [Development Guide](docs/DEVELOPMENT.md).

[Back to top](#new-bearings)

---

## 3. Why New Bearings Exists

Learning is not always a straightforward process.

A person may understand a concept when reading an explanation but struggle to use it independently.

Another learner may understand the concept through examples but find a long written explanation difficult to follow.

Someone may learn effectively in short sessions.

Someone else may prefer longer, uninterrupted periods of study.

A learner may need repeated exposure to a topic before it becomes familiar.

A person returning to a subject after several weeks may need a quick review rather than an entire course repeated from the beginning.

A beginner may need terminology explained before a lesson introduces more advanced ideas.

A more experienced learner may need challenging exercises rather than another introductory explanation.

A useful learning environment should recognize that these differences exist.

### 3.1 The problem with rigid learning paths

A fixed learning path can be useful because it provides structure.

However, a rigid path can also become frustrating when it cannot accommodate different circumstances.

A learner may be required to repeat material they already understand.

A learner may be moved forward before understanding a prerequisite.

A lesson may be too long for the time available.

A particular explanation may not make sense to a particular person.

An exercise may test a skill that has not been adequately introduced.

A user may need to review an older topic while continuing to learn new material.

A single pacing model may not work equally well for every learner.

New Bearings should retain the benefits of structure without making unnecessary rigidity a defining characteristic.

### 3.2 The value of learner control

Personalization should not mean that the application silently decides everything.

The learner should be able to influence the experience.

Users should be able to adjust available presentation options.

Users should be able to choose suitable practice approaches where supported.

Users should be able to review material voluntarily.

Users should be able to change preferences without restarting the entire application.

Users should receive understandable explanations when a setting affects behavior.

Users should not be required to accept a recommendation without understanding what it changes.

Where the application makes an adaptive recommendation, the recommendation should be understandable and, where practical, adjustable.

### 3.3 The importance of retention

Learning is not complete merely because information was seen once.

A learner may recognize a familiar example but be unable to reproduce the underlying idea.

A learner may understand a lesson today and forget important details later.

A learner may remember syntax but misunderstand when to use it.

A learner may solve an exercise by following instructions without being able to solve a similar problem independently.

The application should therefore explore ways to support retrieval practice, repetition, application, and review.

It should also explain the limits of its progress measurements.

### 3.4 A practical approach

New Bearings should be developed as a real application rather than as a collection of disconnected ideas.

Each capability should have a clear purpose.

Each capability should have an identifiable place in the architecture.

Each capability should be tested at an appropriate level.

Each user-facing feature should communicate its limitations.

The project should grow through coherent, verified improvements.

**Related documentation:** [Architecture](docs/ARCHITECTURE.md), [Roadmap](docs/ROADMAP.md), [Content Format](docs/CONTENT_FORMAT.md).

[Back to top](#new-bearings)

---

## 4. Project Philosophy

The central philosophy of New Bearings is that learning should adapt to the individual, not force the individual to adapt to a single way of learning.

This principle should influence product decisions, user-interface decisions, content design, personalization, and technical architecture.

It should not be treated as a slogan that excuses weak implementation.

A system cannot be considered adaptable merely because it includes a settings page.

A system cannot be considered accessible merely because it uses readable colors.

A system cannot be considered effective merely because it records completed lessons.

These qualities require deliberate design and appropriate verification.

### 4.1 Learning is individual

People bring different experiences, knowledge, schedules, goals, and preferences to learning.

These differences should be acknowledged without assuming that every individual in a particular group has the same needs.

A user's preferred learning approach may change over time.

A preference may depend on the subject.

A person may prefer visual examples for one topic and written explanations for another.

A person may want more structure while learning unfamiliar material.

The same person may want greater independence when practicing a familiar skill.

New Bearings should make room for these differences.

### 4.2 Adaptation should remain understandable

Adaptation should not become an unexplained black box.

When the application changes a learning recommendation, the user should receive a useful explanation when that explanation would help.

For example, an application might recommend reviewing a topic because recent practice suggests that the learner had difficulty recalling it.

That recommendation should not be presented as a diagnosis.

The system should distinguish observations from interpretations.

The system should not imply certainty when the available evidence is limited.

The learner should be able to understand that a recommendation is guidance rather than an unquestionable judgment.

### 4.3 Progress is not personal worth

A learning application should never imply that a person's value depends on the number of lessons completed.

Slow progress is not evidence of personal failure.

Taking a break is not evidence of laziness.

Changing a goal is not automatically a lack of commitment.

Needing to review material is a normal part of learning.

A learner may have responsibilities outside the application.

A learner may encounter technical problems that interrupt a session.

The application should avoid turning these circumstances into moral judgments.

### 4.4 Reliability is part of the learning experience

A learner should not lose progress because an application closes unexpectedly.

A learner should not be forced to repeat work because a settings file was handled incorrectly.

A learner should not receive misleading feedback because an exercise was scored incorrectly.

A learner should not be shown an inaccurate completion status because progress was saved inconsistently.

Technical reliability directly affects trust in the learning experience.

### 4.5 Simplicity should not eliminate capability

A beginner-friendly interface does not require a simplistic underlying system.

The application can provide a straightforward experience while maintaining a modular implementation.

Advanced capabilities can be introduced gradually.

Technical details can remain available when needed without overwhelming the default interface.

The project should favor understandable workflows supported by maintainable engineering.

**Related documentation:** [Architecture](docs/ARCHITECTURE.md), [Accessibility](ACCESSIBILITY.md), [Roadmap](docs/ROADMAP.md).

[Back to top](#new-bearings)

---

## 5. Core Principles

The following principles guide the intended product and its development.

### 5.1 Learner autonomy

- The learner should retain meaningful control over available preferences.
- The application should explain important behavior.
- Recommendations should not be treated as commands.
- Personalization should not require unnecessary personal disclosure.
- Users should be able to revise appropriate settings.
- The application should avoid assuming that a particular method works for everyone.
- A user should not need to justify a preference to the application.
- Default settings should remain reasonable for users who do not want extensive configuration.
- The application should avoid making ordinary learning tasks unnecessarily complicated.
- User control should be balanced with clear guidance.

### 5.2 Accessibility by design

- Accessibility should influence the initial design.
- Keyboard operation should be considered throughout the interface.
- Important information should not rely on color alone.
- Text should remain readable at supported display sizes.
- Error messages should communicate useful information.
- Controls should have understandable labels.
- Motion and animation should not be necessary to understand core information.
- Accessibility settings should be documented.
- Preferences should not be interpreted as proof of a diagnosis.
- Accessibility claims should be supported by testing.

### 5.3 Educational honesty

- Completion should not automatically mean mastery.
- Scores should not imply more certainty than the assessment supports.
- Recommendations should be based on available evidence.
- The application should explain uncertainty where appropriate.
- Content should distinguish facts from opinions.
- Technical examples should be checked for correctness.
- Learning outcomes should not be guaranteed.
- The application should not invent credentials or endorsements.
- Unsupported scientific claims should not be used to market features.
- Progress information should be presented in context.

### 5.4 Privacy-conscious development

- Collect only data needed for defined functionality.
- Avoid unnecessary collection of sensitive information.
- Explain data storage behavior.
- Document any network-dependent features.
- Do not silently introduce analytics.
- Do not silently transmit learning history.
- Protect local files against accidental corruption.
- Avoid exposing private data in logs.
- Make data export and deletion behavior understandable where implemented.
- Do not claim privacy guarantees that have not been verified.

### 5.5 Maintainable engineering

- Prefer clear module responsibilities.
- Avoid unnecessary dependencies.
- Keep error handling explicit.
- Test important behavior.
- Preserve existing functionality during changes.
- Avoid unrelated rewrites.
- Document important architectural decisions.
- Treat compatibility as an engineering responsibility.
- Keep user-facing behavior separate from low-level implementation where practical.
- Prefer reliable solutions over impressive but fragile shortcuts.

### 5.6 Incremental delivery

- Establish a working foundation before adding complexity.
- Introduce features in manageable increments.
- Verify changes before expanding their scope.
- Record known limitations.
- Update documentation when behavior changes.
- Avoid treating placeholder code as finished functionality.
- Avoid claiming success without evidence.
- Preserve a clear record of changes.
- Fix serious regressions before adding unrelated features.
- Keep the project understandable as it grows.

**Related documentation:** [Development](docs/DEVELOPMENT.md), [Testing](docs/TESTING.md), [Privacy and Data Handling](docs/PRIVACY_AND_DATA_HANDLING.md).

[Back to top](#new-bearings)

---

## 6. Goals and Non-Goals

A clear distinction between goals and non-goals helps keep the project focused.

### 6.1 Primary goals

New Bearings aims to provide a structured environment for learning and reviewing material.

It aims to support programming education for beginners and developing programmers.

It aims to expand beyond programming when the core application can support additional subjects responsibly.

It aims to accommodate different learning preferences.

It aims to make accessibility part of the design process.

It aims to help users understand their progress.

It aims to support continued practice rather than relying entirely on passive reading.

It aims to preserve user trust through transparent behavior.

It aims to provide a maintainable codebase.

It aims to support future platform expansion.

It aims to make development approachable for contributors.

It aims to keep the product's promises aligned with verified behavior.

### 6.2 Educational goals

The application should help learners encounter new concepts.

It should provide explanations at an appropriate level.

It should connect new concepts with prerequisites.

It should offer practice opportunities.

It should help users identify topics that deserve review.

It should encourage application of knowledge.

It should support different levels of familiarity.

It should make important terminology understandable.

It should help users revisit older material.

It should distinguish exposure from demonstrated understanding.

### 6.3 Technical goals

The project should maintain clear boundaries between modules.

It should keep data handling understandable.

It should use Rust's type system where it helps express valid states.

It should avoid unnecessary duplication.

It should make important behavior testable.

It should provide useful errors.

It should protect user data from avoidable corruption.

It should keep optional features from becoming unnecessary requirements for basic use.

It should document supported build and test procedures.

It should develop cross-platform support deliberately.

### 6.4 Non-goals

New Bearings is not intended to diagnose learning disabilities.

It is not intended to diagnose medical or mental health conditions.

It is not intended to prescribe treatment.

It is not intended to replace a qualified educational professional.

It is not intended to guarantee a particular learning outcome.

It is not intended to guarantee employment.

It is not intended to award an accredited qualification unless a separate, verified program is established.

It is not intended to claim that every learner benefits from the same approach.

It is not intended to collect personal data merely because that data might be interesting.

It is not intended to depend on an external service for every ordinary learning activity without a clear reason.

It is not intended to add features solely to increase the feature count.

### 6.5 Scope management

A broad vision does not require every feature to be built immediately.

Features should be prioritized according to user value, reliability, accessibility, maintainability, and technical dependencies.

The application should establish dependable foundations before taking on complicated adaptive systems.

The project should prefer a small set of useful features that work correctly over a large collection of incomplete controls.

**Related documentation:** [Roadmap](docs/ROADMAP.md), [Release Plan](docs/RELEASE_PLAN.md).

[Back to top](#new-bearings)

---

## 7. Intended Audience

New Bearings is intended for people who want a more flexible way to learn and review information.

The initial focus is programming, particularly for people who are still developing their technical foundations.

The application should remain useful to people with different experience levels.

### 7.1 Beginners

Beginners may need basic terminology explained.

They may benefit from examples that demonstrate one concept at a time.

They may need instructions for setting up a development environment.

They may need help understanding compiler messages.

They may need practice before moving to a more complex topic.

They may benefit from explanations of why a solution works rather than only seeing the solution.

The application should not assume that unfamiliarity with a concept indicates a lack of ability.

### 7.2 Developing programmers

Developing programmers may already understand basic syntax.

They may need practice combining multiple concepts.

They may need to recognize errors.

They may need to understand code organization.

They may need to compare alternative implementations.

They may benefit from exercises that require applying concepts in new situations.

They may need help identifying gaps in prerequisite knowledge.

The application should not force experienced learners through unnecessary introductory material when appropriate alternatives exist.

### 7.3 Returning learners

Some people return to programming after a long break.

They may remember broad ideas but forget syntax or details.

They may need a refresher rather than a complete beginner course.

They may need to review the concepts that are relevant to their current project.

They may benefit from a quick assessment of familiarity.

The application should aim to make returning to a subject less intimidating.

### 7.4 Learners with different preferences

Some learners prefer written explanations.

Some prefer worked examples.

Some prefer interactive practice.

Some prefer a mixture of approaches.

Some prefer short sessions.

Some prefer a longer study period.

Some need fewer visual distractions.

Some benefit from more detailed instructions.

These are possible preferences, not fixed categories of people.

The application should let users choose available options without assigning them a permanent learning identity.

### 7.5 Contributors and educators

The project may also interest people who want to contribute lessons, review technical explanations, improve accessibility, or help maintain the application.

Content contributors should be able to understand the expected format.

Developers should be able to locate the relevant module.

Reviewers should be able to identify the intended behavior.

Educators should be able to assess whether the learning content is accurate and appropriately scoped.

Contributors should not need to accept every future product direction to improve a specific part of the project.

**Related documentation:** [Content Format](docs/CONTENT_FORMAT.md), [Contributing](CONTRIBUTING.md), [Accessibility](ACCESSIBILITY.md).

[Back to top](#new-bearings)

---

## 8. Learning Should Adapt to the Learner

Adaptability is a central product objective.

It should be implemented through specific, understandable behavior rather than vague claims.

### 8.1 Multiple ways to approach material

Where supported, a topic may offer more than one way to study it.

Possible presentation methods include:

- A concise written explanation.
- A more detailed written explanation.
- A worked example.
- A step-by-step walkthrough.
- A practice question.
- A code-reading exercise.
- A debugging exercise.
- A short knowledge check.
- A review activity.
- A practical application.

These options should be introduced according to content quality and implementation capability.

A list of possible formats is not evidence that every format is currently available.

### 8.2 Appropriate pacing

The application should explore ways to make pacing more flexible.

A learner may want to move quickly through familiar concepts.

A learner may want to spend more time on unfamiliar concepts.

A learner may need a break between activities.

A learner may prefer to study in short sessions.

A learner may want to revisit a topic before attempting the next lesson.

Pacing controls should remain understandable.

The application should avoid treating speed as the only measure of ability.

### 8.3 Difficulty adjustment

Difficulty should reflect the demands of the activity, not merely the amount of text displayed.

An introductory exercise might ask the learner to identify a variable.

A more advanced exercise might ask the learner to use variables in a complete program.

A later exercise might require the learner to select an appropriate data structure.

Difficulty may also depend on how much guidance an exercise provides.

The project should document what a difficulty label means when such labels are implemented.

It should avoid pretending that a single number can fully describe a person's ability.

### 8.4 Recommendations

Recommendations may eventually help users decide what to study next.

A recommendation might be based on a prerequisite relationship.

It might be based on a recent practice result.

It might suggest reviewing material that has not been revisited recently.

It might suggest a different exercise format.

Recommendations should remain transparent enough to be useful.

The application should not claim that a recommendation is objectively optimal when the available information is limited.

### 8.5 Avoiding rigid learner labels

The application should not classify users into permanent categories such as visual learner or auditory learner and then restrict the experience accordingly.

People can use multiple approaches.

Preferences can change by task.

A person's preference does not establish that one presentation method is universally more effective for them.

New Bearings should prioritize user choice and observed usefulness over simplistic learner labels.

**Related documentation:** [Accessibility](ACCESSIBILITY.md), [Roadmap](docs/ROADMAP.md), [Content Format](docs/CONTENT_FORMAT.md).

[Back to top](#new-bearings)

---

## 9. Personalization and Learning Preferences

Personalization should help users configure the learning experience around their needs.

It should not become a source of pressure, unnecessary complexity, or unsupported psychological claims.

### 9.1 Optional configuration

Personalization should be optional.

Users should be able to begin with reasonable defaults.

Users should be able to change preferences later.

Users should be able to decline questions that are not necessary to use the application.

Users should not need to provide a medical diagnosis.

Users should not need to explain why a particular setting helps them.

Users should be able to understand what each available preference changes.

### 9.2 Possible preferences

Depending on implementation, configurable preferences may include:

- Preferred explanation length.
- Amount of instructional detail.
- Use of worked examples.
- Use of practice questions.
- Default session length.
- Reminder preferences.
- Interface text size.
- Theme selection.
- Motion-related preferences.
- Keyboard navigation behavior.
- Difficulty preferences where supported.
- Review frequency where supported.
- Whether optional progress indicators are displayed.
- Whether optional motivational elements are displayed.
- Whether particular learning formats are preferred.

These are design possibilities, not a guarantee that every setting exists in the current application.

### 9.3 Preferences are not diagnoses

A preference should be treated as information about how a user wants to interact with the application.

It should not be interpreted as proof of a medical condition.

A user who prefers short explanations does not necessarily have an attention-related condition.

A user who prefers larger text does not necessarily have a particular disability.

A user who prefers repeated examples does not necessarily have a learning disorder.

The application should not infer sensitive traits from ordinary configuration choices.

### 9.4 Defaults and reversibility

Default settings should allow the application to be useful without extensive setup.

A user should not need to answer a lengthy questionnaire before seeing the basic learning experience.

Where settings can be changed, the application should provide a clear path to change them.

Where a setting affects stored data, the consequences should be explained.

Where a setting cannot be changed without affecting other behavior, that limitation should be documented.

Settings should not be silently reset without a reasonable explanation.

### 9.5 Personalization and evidence

Adaptive behavior should be evaluated according to what it actually does.

A feature should not be described as intelligent merely because it changes a setting.

A recommendation should not be described as personalized if it is identical for every user and ignores relevant preferences.

A learning estimate should not be described as a measurement of intelligence.

The project should be explicit about whether a feature uses user-selected preferences, recorded activity, a fixed rule, or another method.

**Related documentation:** [Privacy and Data Handling](docs/PRIVACY_AND_DATA_HANDLING.md), [Accessibility](ACCESSIBILITY.md), [Settings](docs/ARCHITECTURE.md).

[Back to top](#new-bearings)

---

## 10. Accessibility and Inclusive Design

Accessibility should be part of the application's engineering process from the beginning.

It should not be postponed until the interface is considered complete.

The objective is to reduce avoidable barriers and give users practical ways to interact with learning material.

Accessibility work must be verified. Intentions alone do not establish that a feature is accessible.

### 10.1 Visual accessibility

The interface should aim for readable text.

Text and background combinations should be chosen with contrast in mind.

Important information should not depend exclusively on color.

Interactive controls should be visually distinguishable.

Focus indicators should remain visible during keyboard navigation.

Text should support reasonable scaling where the interface framework permits it.

Long explanations should be structured into readable sections.

Error messages should remain visible long enough to understand.

The interface should avoid relying on tiny decorative text to communicate essential information.

### 10.2 Keyboard accessibility

Core workflows should aim to support keyboard operation.

Users should be able to identify which control has focus.

The tab order should follow a sensible sequence.

Keyboard shortcuts should be documented.

Shortcuts should avoid unnecessary conflicts.

Controls should not require a mouse when an equivalent keyboard interaction is practical.

Keyboard navigation should not create traps that prevent a user from leaving a control or dialog.

Dialog behavior should be tested.

Focus should return to a sensible location after a dialog closes.

### 10.3 Cognitive accessibility

The interface should avoid unnecessary complexity.

Instructions should use clear language.

Technical terms should be explained when the audience may not know them.

Long tasks should be divided into manageable steps where appropriate.

Important actions should have predictable outcomes.

The interface should avoid surprising changes that interrupt a learning activity.

Error recovery should be explained.

The application should not rely on users remembering hidden instructions.

Related controls should be grouped logically.

The interface should distinguish primary actions from secondary actions.

### 10.4 Motion and sensory considerations

Motion should serve a clear purpose.

Essential information should not depend on animation.

Where practical, motion preferences should be respected.

Flashing effects should not be used casually.

Sound should not be required to understand an important event.

Optional sound should be controllable.

Visual feedback should remain understandable when animation is reduced or disabled.

### 10.5 Accessibility and educational content

Accessibility also applies to lessons.

Instructions should be readable and logically structured.

Code examples should have clear explanations.

Exercise requirements should be explicit.

Images used in learning content should include meaningful descriptions when appropriate.

Color-coded examples should have another way to distinguish important information.

The content format should allow lessons to remain understandable without relying on presentation alone.

### 10.6 Accessibility testing

Accessibility checks should cover actual workflows.

Tests should examine keyboard navigation.

Tests should examine focus behavior.

Tests should examine text scaling where supported.

Tests should examine error messages.

Tests should examine whether essential information is conveyed without color alone.

Tests should examine dialogs and other interactive elements.

Manual checks should complement automated checks.

A successful compilation does not prove accessibility.

A single automated accessibility check does not prove accessibility.

New Bearings should describe the tests performed and the limitations of those tests.

### 10.7 No diagnosis required

Users should not need to disclose a disability to use accessibility features.

Where practical, accessibility options should be available directly through settings.

The application should not require medical documentation.

The application should not infer a diagnosis from the use of an accessibility setting.

The application should not claim to provide clinical accommodations or medical treatment.

**Related documentation:** [Accessibility Policy](ACCESSIBILITY.md), [Testing](docs/TESTING.md), [Privacy and Data Handling](docs/PRIVACY_AND_DATA_HANDLING.md).

[Back to top](#new-bearings)

---

## 11. Supported Subjects and Learning Environments

Programming is the initial subject focus for New Bearings.

The architecture should avoid unnecessarily restricting the application to programming forever.

A reusable learning system can provide shared infrastructure while allowing different subjects to use appropriate content and exercises.

### 11.1 Initial subject: programming

Programming education offers a useful starting point because it combines explanations, terminology, examples, exercises, debugging, and practical application.

It also creates opportunities to distinguish between reading a solution and producing one independently.

Programming lessons may introduce a concept, demonstrate it, and provide practice.

They may explain how the concept relates to earlier material.

They may include common mistakes.

They may provide expected outputs.

They may ask learners to predict what a program will do.

They may ask learners to explain why a solution works.

The quality and correctness of each activity matter more than the number of lessons available.

### 11.2 Future subject expansion

Other subjects may be considered when the learning model can support their needs.

Possible future areas include mathematics.

Possible future areas include computer science theory.

Possible future areas include general technical skills.

Possible future areas include language learning.

Possible future areas include other knowledge-based subjects.

These examples describe potential scope rather than confirmed commitments.

Each subject may need different assessment methods.

Each subject may need different content structures.

Each subject may need different feedback rules.

The application should not assume that a programming exercise model can be copied unchanged into every discipline.

### 11.3 Shared infrastructure

Different subjects may share services such as navigation, settings, progress storage, and content loading.

The learning logic for a particular subject should remain appropriate to that subject.

The application should avoid forcing subject-specific behavior into unrelated core modules.

A shared content model should remain expressive enough for different lesson types.

The application should use explicit capabilities rather than pretending every content item supports every interaction.

### 11.4 Subject-specific quality

A lesson should be reviewed for subject accuracy.

A programming lesson should be checked for correct technical behavior.

A mathematics lesson should be checked for correct calculations and reasoning.

A language lesson should be checked for accurate usage and suitable context.

An explanation should not be treated as reliable merely because it is written confidently.

The project should document the review process as content contribution expands.

**Related documentation:** [Content Format](docs/CONTENT_FORMAT.md), [Architecture](docs/ARCHITECTURE.md), [Roadmap](docs/ROADMAP.md).

[Back to top](#new-bearings)

---

## 12. Programming Language Support

The initial focus is learning programming, with the longer-term possibility of providing language-specific learning environments.

Potential languages include Rust, Python, C, C++, C#, Java, JavaScript, TypeScript, and Go.

Unity-oriented C# learning may also be relevant to future content.

Listing a language here does not mean the application already includes a complete course, compiler integration, code runner, debugger, or development environment for that language.

### 12.1 Language-appropriate learning

Different programming languages have different syntax and execution models.

A lesson about ownership in Rust should not be presented as though it were identical to a lesson about object lifetime in another language.

A lesson about memory management in C should address the responsibilities and risks associated with manual memory management.

A lesson about Python should reflect Python's actual runtime behavior.

A lesson about C# should distinguish language features from framework-specific features.

A lesson about JavaScript should explain the relevant execution context.

The application should preserve these distinctions.

### 12.2 Shared concepts

Some concepts can be taught across multiple languages.

Examples include variables, types, conditions, loops, functions, data structures, debugging, and code organization.

However, similar concepts may have important differences across languages.

Shared lessons should explain those differences rather than implying that syntax and semantics are interchangeable.

### 12.3 Language-specific environments

A language environment may eventually provide:

- Language-specific lessons.
- Appropriate code examples.
- Syntax-focused exercises.
- Conceptual exercises.
- Debugging practice.
- Language-specific terminology.
- Prerequisite relationships.
- Language-specific content packs.
- Optional tooling integrations where implemented.

The precise capabilities should be documented for each environment.

### 12.4 Tooling is separate from instruction

Teaching a programming language does not automatically require the application to execute user code.

Some lessons can be completed through reading, prediction, or conceptual exercises.

Other activities may benefit from a compiler or interpreter.

Executing user-supplied code introduces additional security and resource-management concerns.

Any execution capability should be designed and verified separately.

The project should not imply that code is executed safely merely because it is written in a supported language.

### 12.5 Compiler and interpreter integration

A future integration may depend on an external compiler, interpreter, or development tool.

The application should explain when a dependency is required.

It should identify supported configurations.

It should handle missing dependencies clearly.

It should not silently install development tools without appropriate user consent.

It should not assume that a compiler is available because a language appears in the lesson catalogue.

It should not claim that an external tool is bundled unless the distribution actually includes it.

**Related documentation:** [Content Format](docs/CONTENT_FORMAT.md), [Architecture](docs/ARCHITECTURE.md), [Installation](docs/INSTALLATION.md).

[Back to top](#new-bearings)

---

## 13. Planned Feature Catalogue

This section describes the intended direction of the application.

Each item must be checked against the current implementation before it is described elsewhere as available.

### 13.1 Learning content

Potential capabilities include a structured lesson catalogue.

Potential capabilities include lesson descriptions and prerequisites.

Potential capabilities include topic grouping.

Potential capabilities include beginner-oriented explanations.

Potential capabilities include worked examples.

Potential capabilities include practical exercises.

Potential capabilities include knowledge checks.

Potential capabilities include links between related topics.

Potential capabilities include reviewable content.

Potential capabilities include content packs.

The implementation status of each capability must be verified independently.

### 13.2 Learning preferences

Potential capabilities include configurable explanation detail.

Potential capabilities include optional presentation preferences.

Potential capabilities include configurable session preferences.

Potential capabilities include accessible interface settings.

Potential capabilities include flexible review preferences.

Potential capabilities include optional progress displays.

Potential capabilities include optional reminders.

The application should avoid implying that all these controls are currently present.

### 13.3 Practice and review

Potential capabilities include repeated practice.

Potential capabilities include recall-based exercises.

Potential capabilities include review recommendations.

Potential capabilities include exercises that build on prerequisites.

Potential capabilities include identifying topics that may need further practice.

Potential capabilities include varied question formats.

Potential capabilities include feedback that explains an answer.

The quality of each assessment should be evaluated independently.

### 13.4 Progress and motivation

Potential capabilities include recording lesson completion.

Potential capabilities include tracking practice activity.

Potential capabilities include topic-level progress.

Potential capabilities include achievements.

Potential capabilities include optional goals.

Potential capabilities include study reminders.

Potential capabilities include progress summaries.

The application should distinguish activity statistics from actual mastery.

### 13.5 User experience

Potential capabilities include guided setup.

Potential capabilities include clearer navigation.

Potential capabilities include configurable settings.

Potential capabilities include contextual help.

Potential capabilities include more useful error messages.

Potential capabilities include improved accessibility.

Potential capabilities include clearer recovery workflows.

The interface should not expose controls that have no functional behavior.

### 13.6 Storage and reliability

Potential capabilities include persistent settings.

Potential capabilities include saved learning progress.

Potential capabilities include content validation.

Potential capabilities include safer data updates.

Potential capabilities include recovery from interrupted writes.

Potential capabilities include data export where appropriate.

Potential capabilities include schema evolution and migration.

These capabilities require implementation and testing before they can be promised.

### 13.7 Future integrations

Additional integrations may be considered if they provide clear value.

An integration should have a defined purpose.

An integration should have documented dependencies.

An integration should explain whether it uses the network.

An integration should explain whether it requires external software.

An integration should have a clear failure mode.

An integration should not become mandatory for unrelated core functionality without a strong reason.

### 13.8 Feature acceptance

A feature should not be considered complete simply because a control appears in the interface.

A feature should have a defined expected behavior.

Its normal path should be tested.

Its important error paths should be considered.

Its data handling should be understood.

Its accessibility implications should be reviewed.

Its documentation should match the implementation.

Its dependencies should be documented.

Its limitations should be stated.

**Related documentation:** [Roadmap](docs/ROADMAP.md), [Architecture](docs/ARCHITECTURE.md), [Testing](docs/TESTING.md).

[Back to top](#new-bearings)

---

## 14. Platform Support

Windows is the initial development platform.

Cross-platform support is a long-term objective.

Cross-platform support should be treated as a compatibility goal that requires actual testing, not as a property guaranteed merely because the application is written in Rust.

### 14.1 Windows

The current development environment is Windows.

Windows-specific development instructions should be documented where necessary.

Path handling should account for Windows path conventions.

File access should produce useful errors when permissions or paths prevent an operation.

The application should avoid assuming that every user has the same drive layout.

The application should not depend on a developer's personal absolute paths.

Build instructions should explain the required toolchain.

Windows behavior should be tested on supported configurations before a stable release is declared.

### 14.2 macOS

macOS is a potential future target.

Support requires appropriate builds and testing.

Filesystem behavior must be reviewed.

Platform-specific permissions may affect application behavior.

Packaging and signing requirements may affect distribution.

The project should not claim a working macOS release until an appropriate artifact and its relevant workflows have been verified.

### 14.3 Linux

Linux is a potential future target.

Linux distributions differ in packaging systems and system libraries.

Build dependencies should be documented.

Filesystem permissions should be considered.

Installation guidance should identify the actual supported distribution formats.

A successful build on one Linux distribution should not automatically be treated as verification of every distribution.

### 14.4 Platform abstraction

Platform-specific behavior should be isolated when that improves maintainability.

The core learning logic should avoid unnecessary dependencies on one operating system.

File paths should use suitable platform-aware APIs.

The application should avoid hard-coded platform assumptions where a standard library abstraction exists.

Platform-specific integrations should remain explicit.

### 14.5 Compatibility reporting

A platform should be listed as supported only when the project has a reasonable basis for that claim.

A platform that has not been tested should be marked accordingly.

A successful compilation is useful evidence, but it does not prove that the complete application works correctly.

Release notes should describe relevant platform limitations.

**Related documentation:** [Installation](docs/INSTALLATION.md), [Development](docs/DEVELOPMENT.md), [Release Plan](docs/RELEASE_PLAN.md).

[Back to top](#new-bearings)

---

## 15. Technology Stack

Rust is the main implementation language.

The project uses Rust's package and build tooling to manage compilation and dependencies.

The application is intended to remain modular and maintainable.

### 15.1 Rust

Rust provides strong static typing and memory-safety features.

Its ownership model can help prevent classes of memory errors.

Its type system can make invalid states harder to represent when used thoughtfully.

Its package ecosystem provides a standard approach to dependency management.

Its build tooling supports repeatable development workflows.

These advantages do not eliminate every possible bug.

Rust applications still require careful error handling.

Rust applications still require testing.

Rust applications still require attention to data integrity and security.

Unsafe code, if introduced, requires additional scrutiny.

### 15.2 Cargo

Cargo is Rust's package manager and build tool.

The project's `Cargo.toml` describes package metadata and dependencies.

Cargo may generate or update `Cargo.lock` as part of normal dependency resolution.

`Cargo.lock` should not be manually edited to perform a package rename.

Package metadata should be changed deliberately.

Generated dependency information should be updated through appropriate Cargo commands.

Build instructions should use commands that match the actual repository configuration.

### 15.3 Dependency management

Dependencies should serve a clear purpose.

Before adding a dependency, consider whether the functionality is already available in the standard library or an existing project dependency.

Review maintenance activity and compatibility where relevant.

Review license information before distribution.

Avoid introducing large frameworks for minor conveniences.

Avoid relying on unmaintained dependencies without understanding the consequences.

Keep dependency updates reviewable.

Test important behavior after dependency changes.

### 15.4 User interface technology

The final interface framework should be documented according to the actual implementation.

The project should not claim support for a particular GUI framework merely because it is being considered.

Interface architecture should be chosen with usability, accessibility, maintainability, and platform requirements in mind.

If a framework is introduced, its dependency and licensing implications should be reviewed.

### 15.5 External tools

Optional external tools should have clearly documented purposes.

An external compiler should not be confused with the application itself.

A system utility should not be assumed to exist on every supported platform.

A future integration should not silently become a required dependency for unrelated features.

The application should explain when a tool is missing and what the user can do next.

### 15.6 Technology decisions

Technical decisions should be based on requirements rather than novelty.

The project should avoid changing frameworks merely because another framework is popular.

A significant technology change should have a documented reason.

The cost of migration should be considered.

Existing functionality should be preserved where practical.

**Related documentation:** [Architecture](docs/ARCHITECTURE.md), [Development](docs/DEVELOPMENT.md), [Decisions](docs/DECISIONS.md).

[Back to top](#new-bearings)

---

## 16. Architecture Overview

New Bearings is organized into modules with distinct responsibilities.

The purpose of modular organization is to make the code easier to understand, test, and extend.

The exact responsibility of each module should be determined from the current source code and the architecture documentation.

A module name alone does not establish its complete implementation.

### 16.1 Architectural objectives

The architecture should separate concerns.

The user interface should not need to contain every learning rule.

Storage code should not need to determine how every lesson is taught.

Content definitions should not be responsible for unrelated operating-system behavior.

Platform-specific operations should be isolated where practical.

Error handling should remain understandable across module boundaries.

Settings should be handled consistently.

The application should avoid circular dependencies between unrelated systems.

Shared functionality should be introduced when it reduces meaningful duplication.

### 16.2 Core application flow

A conceptual learning workflow may follow this sequence:

1. The application starts.
2. The application loads its required configuration.
3. Required services are initialized.
4. The user reaches an appropriate starting screen.
5. The user navigates to a learning activity.
6. The application retrieves the relevant content.
7. The learner interacts with the activity.
8. The application evaluates the supported response.
9. Relevant progress is updated.
10. The application communicates the result.
11. The learner may continue or review.
12. Appropriate state is saved.

This is a conceptual flow, not a claim that every step is already implemented.

The actual flow should be documented when its behavior is verified.

### 16.3 Module boundaries

Each module should have a clear reason to exist.

A module should expose a limited, understandable interface.

Unrelated modules should not depend on unnecessary internal details.

Shared types should be placed where they can be reused without creating avoidable coupling.

Cross-module changes should be reviewed for side effects.

A refactor should not silently change user-facing behavior.

### 16.4 Data flow

Data should have a clear origin.

Data should have a clear owner.

Data should have a defined lifetime.

Data should be validated at appropriate boundaries.

Data should not be copied unnecessarily when a suitable alternative exists.

Data persisted to disk should be handled with appropriate error checking.

Data imported from content files should not automatically be treated as trusted.

### 16.5 Error flow

Errors should preserve useful context.

A low-level error should be translated into a meaningful explanation when presented to the user.

The application should not hide failures merely to keep the interface looking successful.

Recoverable errors should be distinguished from failures that prevent continued operation.

Where practical, the user should receive a next step.

Technical details should remain available for debugging where appropriate.

### 16.6 Architectural documentation

Important design choices should be documented.

The architecture document should explain module responsibilities.

It should identify important shared types and boundaries.

It should explain how content enters the learning system.

It should explain how progress is stored.

It should explain how settings are loaded and saved.

It should explain important error-handling conventions.

It should be updated when the architecture changes.

**Related documentation:** [Architecture](docs/ARCHITECTURE.md), [Decisions](docs/DECISIONS.md), [Development](docs/DEVELOPMENT.md).

[Back to top](#new-bearings)

---

## 17. Module Responsibilities

The repository contains several application modules.

The following descriptions identify their broad intended areas. The actual code remains the authority for what each module currently implements.

### 17.1 Accessibility

The accessibility module is associated with accessibility-related functionality.

Its intended responsibility is to support an interface that can accommodate different interaction preferences and accessibility needs.

It should not contain unrelated lesson content.

Accessibility behavior should be tested through actual user workflows.

The module should coordinate with settings or interface components through clear interfaces where appropriate.

Its documentation should distinguish implemented controls from planned improvements.

### 17.2 Achievements

The achievements module is associated with achievement-related functionality.

Its intended role is to represent optional milestones or accomplishments.

Achievements should not be treated as proof of mastery by themselves.

Achievement conditions should be clear.

The application should avoid misleading users about what an achievement represents.

### 17.3 App

The app module is associated with the overall application.

It may coordinate startup and high-level application behavior.

Its responsibilities should remain clear.

It should not become a catch-all location for every new feature.

Startup behavior should be testable where practical.

### 17.4 Content packs

The content-packs module is associated with organizing or loading learning content packs.

Content loading should validate data before use.

Invalid content should produce useful errors.

Content definitions should remain separate from unrelated user-interface details where practical.

Content compatibility should be documented.

### 17.5 Errors

The errors module is associated with application error types or error handling.

Errors should communicate enough information to identify the failure.

User-facing messages should be understandable.

Technical context should be retained where appropriate.

Error types should not be used to silently convert failures into success.

### 17.6 Learning

The learning module is associated with learning-related behavior.

Its exact responsibilities should be documented as implementation develops.

Learning rules should be testable.

The module should avoid unnecessary dependence on presentation details.

Changes to learning behavior should be evaluated for their effect on existing activities.

### 17.7 Lessons

The lessons module is associated with lesson-related structures or workflows.

Lessons should have clear content and behavior.

Lesson completion should be defined explicitly.

Lesson data should be validated.

The lesson system should avoid treating opening a lesson as proof that its material has been mastered.

### 17.8 Navigation

The navigation module is associated with moving between application areas.

Navigation should have predictable behavior.

Invalid destinations should be handled safely.

Keyboard navigation should be considered where the interface supports it.

Navigation should not unnecessarily duplicate learning logic.

### 17.9 Platform

The platform module is associated with platform-specific behavior.

Platform operations should use appropriate abstractions.

Unsupported platform behavior should be explicit.

Platform-specific errors should be translated into useful messages where practical.

The module should not make cross-platform compatibility claims without verification.

### 17.10 Progress

The progress module is associated with recording or representing learning progress.

Progress should be defined carefully.

A completed activity should not automatically imply full understanding.

Progress updates should be consistent.

Progress information should be protected from accidental loss.

### 17.11 Reminders

The reminders module is associated with reminder-related functionality.

Reminder behavior should respect user preferences.

Reminders should be optional where practical.

The application should not make misleading claims about reminders that depend on unsupported operating-system behavior.

Reminder scheduling should be tested before being documented as reliable.

### 17.12 Resources

The resources module is associated with application resources.

Resource paths should be handled consistently.

Missing required resources should produce useful errors.

Optional resources should not unnecessarily prevent unrelated core workflows.

Resource packaging should be verified for release builds.

### 17.13 Review

The review module is associated with reviewing previously studied material.

Review behavior should have clear rules.

Review recommendations should be explainable where practical.

Review should not assume that every learner benefits from an identical schedule.

Review status should be distinguished from overall mastery.

### 17.14 Rewards

The rewards module is associated with reward-related functionality.

Rewards should support the learning experience rather than dominate it.

Reward conditions should be transparent.

Rewards should not punish users for circumstances outside their control.

The application should not present a reward count as a scientific measure of ability.

### 17.15 Settings

The settings module is associated with user configuration.

Settings should have sensible defaults.

Invalid settings should be handled safely.

Settings persistence should be tested.

The application should explain important settings.

Settings should not unexpectedly overwrite unrelated user data.

### 17.16 Setup

The setup module is associated with setup-related behavior.

Setup should avoid unnecessary barriers.

Required configuration should be distinguished from optional preferences.

Setup errors should be actionable.

The user should not be forced through irrelevant configuration steps.

### 17.17 Storage

The storage module is associated with persistence.

Storage operations should handle failures explicitly.

Writes should be designed to reduce avoidable corruption.

Stored data should be validated before use.

Schema changes should be managed deliberately.

### 17.18 UI

The UI module is associated with the user interface.

Interface code should communicate clearly with application services.

The UI should avoid owning unrelated business logic.

Visible controls should have real behavior.

Interface changes should be reviewed for accessibility.

### 17.19 Tests

The repository also includes tests.

Tests should verify behavior rather than merely repeat implementation details.

Test names should explain what is being checked.

Important edge cases should be included where practical.

A passing test suite should not be described as broader verification than it actually provides.

**Related documentation:** [Architecture](docs/ARCHITECTURE.md), [Testing](docs/TESTING.md), [Development](docs/DEVELOPMENT.md).

[Back to top](#new-bearings)

---

## 18. Learning Content and Lesson Design

Learning content is one of the most important parts of New Bearings.

A well-organized application cannot compensate for inaccurate or confusing lessons.

Content should be accurate, appropriately scoped, and understandable to its intended audience.

### 18.1 Lesson objectives

A lesson should have a clear purpose.

The objective should describe what the learner is expected to understand or practice.

Objectives should avoid vague wording when a more specific description is possible.

A lesson should not attempt to teach too many unrelated concepts at once.

Prerequisites should be identified where they matter.

A lesson should explain unfamiliar terminology.

A lesson should avoid assuming knowledge that has not been introduced or established.

### 18.2 Explanations

Explanations should introduce the main idea.

They should define important terms.

They should explain why the concept matters.

They should show how the concept works.

They should distinguish similar but different concepts.

They should identify common misunderstandings where useful.

They should avoid unnecessary jargon for beginners.

They should introduce technical terminology when it helps the learner progress.

### 18.3 Examples

Examples should illustrate the concept being taught.

Examples should be correct.

Code examples should use syntax appropriate to the relevant language version.

Examples should avoid unexplained dependencies where possible.

Examples should explain important lines or decisions.

Examples should distinguish required behavior from optional style choices.

Examples should not teach unsafe practices without appropriate explanation.

### 18.4 Exercises

Exercises should have clear instructions.

The expected task should be understandable.

The difficulty should be appropriate to the stated learning objective.

Exercises should avoid requiring knowledge that has not been introduced unless that requirement is intentional and documented.

Feedback should explain mistakes when possible.

An exercise should not mark an answer incorrect merely because it differs stylistically from one preferred solution when the answer is otherwise valid.

### 18.5 Content review

Content should be reviewed for factual accuracy.

Programming content should be tested where practical.

Examples should be checked for compatibility.

Typographical errors should be corrected.

Prerequisite relationships should be checked.

Feedback should be reviewed for clarity.

Content changes should be documented when they affect behavior or meaning.

### 18.6 Content consistency

Terminology should remain consistent within a learning path.

A concept should not be defined differently in unrelated lessons without explanation.

Where languages use similar terminology differently, the distinction should be stated.

Examples should follow documented conventions.

Content metadata should use a consistent format.

### 18.7 Content limitations

A lesson can be correct but incomplete.

An example can illustrate a concept without covering every edge case.

A practice question can assess one part of a skill without proving broad competence.

The application should not exaggerate what a lesson or assessment demonstrates.

**Related documentation:** [Content Format](docs/CONTENT_FORMAT.md), [Testing](docs/TESTING.md), [Roadmap](docs/ROADMAP.md).

[Back to top](#new-bearings)

---

## 19. Practice, Recall, and Retention

New Bearings intends to support learning beyond initial exposure to information.

Practice and review should be designed around clear educational objectives.

The project should avoid promising that any particular schedule or technique guarantees retention.

### 19.1 Active practice

Active practice asks learners to do something with the material.

Depending on the subject, this may involve answering a question, predicting an outcome, explaining a concept, solving a problem, or writing code.

Practice can reveal gaps that passive reading does not.

Practice results should be interpreted in context.

A single incorrect answer should not automatically be treated as evidence that a learner cannot understand the topic.

### 19.2 Retrieval practice

Retrieval practice involves attempting to recall information rather than simply rereading it.

The application may eventually use retrieval-oriented activities.

These activities should be matched to the content.

A programming lesson might ask the learner to explain what a function does.

It might ask the learner to predict output.

It might ask the learner to complete a short example.

It might ask the learner to identify a mistake.

The task should measure the intended concept as clearly as possible.

### 19.3 Spaced review

Spaced review involves revisiting material over time.

A future review system may use past activity to suggest when to review a topic.

The system should document its scheduling logic.

It should not imply that one scheduling algorithm is perfect for every person.

It should allow appropriate user control where practical.

It should handle incomplete or missing history.

It should avoid making a missed review feel like a personal failure.

### 19.4 Feedback

Feedback should identify the result of an activity.

Where practical, it should explain why an answer is correct or incorrect.

Feedback should distinguish a conceptual error from a formatting issue.

Feedback should not shame the learner.

Feedback should avoid making unsupported judgments about ability.

When a question is ambiguous, the problem may be the question rather than the learner's answer.

Content contributors should review exercises for ambiguity.

### 19.5 Retention versus familiarity

Recognizing a concept is not always the same as recalling it independently.

Remembering a definition is not always the same as applying it correctly.

Completing a guided example is not always the same as solving a new problem.

The application should avoid collapsing these distinctions into a single simplistic measure.

Progress displays should explain what they represent.

### 19.6 Review recommendations

Review recommendations should be treated as guidance.

They may be based on incomplete information.

They may not account for every external learning experience.

The application should avoid suggesting that its records contain the user's entire knowledge history.

The learner should remain able to choose what to review.

**Related documentation:** [Architecture](docs/ARCHITECTURE.md), [Content Format](docs/CONTENT_FORMAT.md), [Roadmap](docs/ROADMAP.md).

[Back to top](#new-bearings)

---

## 20. Progress Tracking

Progress tracking should help learners understand what they have done and what may deserve attention.

It should not pretend to measure every aspect of learning.

### 20.1 Possible progress information

Depending on implementation, progress information may include completed lessons.

It may include practice attempts.

It may include recent activity.

It may include topics studied.

It may include review activity.

It may include saved learning goals.

It may include achievement milestones.

It may include a record of content completed.

These examples do not guarantee that each data point is currently tracked.

### 20.2 Clear definitions

Every progress indicator should have a defined meaning.

A completed lesson should mean that the application's completion criteria were met.

A correct answer should mean that the answer met the relevant scoring rules.

A review count should reflect the events the application actually records.

A streak should have a documented definition if implemented.

A score should have a documented scale.

A progress percentage should explain its denominator when that information is not obvious.

### 20.3 Avoiding misleading metrics

A high activity count does not automatically mean a learner understands the material.

A low activity count does not automatically mean the learner has made little progress outside the application.

A long session does not automatically mean the session was effective.

A short session does not automatically mean the session was ineffective.

A completed exercise does not automatically prove mastery.

The application should avoid presenting a single metric as a complete description of learning.

### 20.4 Data integrity

Progress should be saved consistently.

The application should handle storage failures explicitly.

A failed save should not silently appear successful.

Progress migrations should preserve information where possible.

The application should avoid duplicate progress records when retrying operations.

The application should handle incomplete records safely.

### 20.5 User understanding

Progress displays should be understandable.

Labels should explain what is being counted.

Graphs, if introduced, should use meaningful scales.

Color should not be the only way to distinguish progress states.

Historical data should not be shown with misleading precision.

The application should explain relevant limitations.

**Related documentation:** [Privacy and Data Handling](docs/PRIVACY_AND_DATA_HANDLING.md), [Architecture](docs/ARCHITECTURE.md), [Testing](docs/TESTING.md).

[Back to top](#new-bearings)

---

## 21. Achievements and Motivation

Achievements may provide optional recognition for learning activity.

They should support the learning experience rather than become the primary purpose of the application.

### 21.1 Purpose of achievements

An achievement may recognize completing a learning path.

It may recognize practicing a particular topic.

It may recognize returning to review material.

It may recognize completing a challenging activity.

It may recognize progress toward a user-selected goal.

The exact conditions should be documented for each implemented achievement.

### 21.2 Healthy motivation

The application should avoid shaming users for missed days.

It should avoid suggesting that rest invalidates prior effort.

It should avoid making every activity competitive.

It should avoid treating every learner as though they want public rankings.

It should allow motivational features to remain optional where practical.

It should avoid pressuring users to spend more time in the application than they intend.

### 21.3 Achievement accuracy

An achievement should describe what it actually recognizes.

An achievement for completing a course should not be described as professional certification.

An achievement for answering questions correctly should not be presented as a guarantee of practical competence.

An achievement for reviewing material should not be treated as proof of long-term retention.

The application should avoid inflated labels.

### 21.4 Optional rewards

Rewards may be useful for some users and distracting for others.

Where supported, users should have reasonable control over optional motivational elements.

Reward systems should not interfere with core learning workflows.

Reward systems should not make important educational information harder to access.

**Related documentation:** [Roadmap](docs/ROADMAP.md), [Architecture](docs/ARCHITECTURE.md), [Accessibility](ACCESSIBILITY.md).

[Back to top](#new-bearings)

---

## 22. Reminders and Study Routines

Reminders may help learners return to material they intend to study.

They should remain a support mechanism rather than a source of pressure.

### 22.1 Optional reminders

Reminders should be configurable where implemented.

Users should understand whether reminders are enabled.

Users should understand how reminder timing is determined.

Users should be able to change reminder preferences through an understandable workflow.

The application should not silently introduce intrusive notifications.

### 22.2 Practical scheduling

A reminder system should respect the platform's actual scheduling capabilities.

The application should not promise that a notification will appear at an exact time if the operating system may delay or suppress it.

Sleep, power-saving behavior, permissions, and application state may affect notifications.

Relevant limitations should be documented.

### 22.3 Missed sessions

A missed reminder should not be treated as failure.

A missed session should not automatically erase progress.

The application should avoid escalating reminders without user consent.

The application should not imply that more frequent reminders always improve learning.

### 22.4 Reminder testing

Reminder behavior should be tested on supported platforms.

Tests should examine enabled and disabled states.

Tests should examine preference persistence.

Tests should examine invalid schedules.

Tests should examine relevant timezone and clock behavior if applicable.

Tests should examine platform-specific limitations.

**Related documentation:** [Architecture](docs/ARCHITECTURE.md), [Testing](docs/TESTING.md), [Privacy and Data Handling](docs/PRIVACY_AND_DATA_HANDLING.md).

[Back to top](#new-bearings)

---

## 23. Navigation and User Experience

The application should make it clear where the user is and how to reach the next relevant action.

Navigation should remain predictable.

The interface should avoid exposing unnecessary complexity during ordinary learning tasks.

### 23.1 Clear structure

Related actions should be grouped together.

Important actions should have understandable labels.

Navigation labels should remain consistent.

The same action should not have unrelated meanings in different parts of the interface without a clear reason.

The application should provide a sensible way to return to the previous context where appropriate.

### 23.2 Beginner-friendly language

The interface should use direct language.

Technical terminology should be explained when needed.

Buttons should describe their action.

Warnings should explain the relevant consequence.

Errors should identify the problem where practical.

Instructions should avoid unnecessary ambiguity.

### 23.3 Predictability

A control should behave consistently with its label.

A button should not silently perform a different action.

A settings change should not unexpectedly discard unrelated progress.

A navigation change should not silently reset an active learning activity unless that behavior is necessary and communicated.

Dialogs should have clear actions.

Cancellation should behave predictably.

### 23.4 Focus and interruption

The interface should avoid unnecessary interruptions.

Notifications should not obscure important learning content without a reason.

Dialogs should preserve appropriate context.

Focus should move predictably after interactions.

The application should avoid losing user input when an avoidable transition occurs.

### 23.5 Empty states

An empty lesson catalogue should not appear to be a crash.

A lack of saved progress should be explained.

An unavailable optional feature should be described clearly.

A missing content pack should produce a useful message.

An empty history should not be presented as an error if it is an expected state.

### 23.6 Visible functionality

Every visible control should have a defined purpose.

A control that is not implemented should not be presented as fully functional.

If a feature is intentionally unavailable, the interface should communicate that state.

The application should avoid fake progress indicators.

The application should avoid controls that appear to work but discard the result.

**Related documentation:** [Accessibility](ACCESSIBILITY.md), [Architecture](docs/ARCHITECTURE.md), [Testing](docs/TESTING.md).

[Back to top](#new-bearings)

---

## 24. Settings and Configuration

Settings should help users configure the application without requiring them to understand its internal architecture.

Defaults should be sensible.

Configuration should be validated.

Settings changes should have predictable effects.

### 24.1 Configuration principles

Each setting should have a clear purpose.

Each setting should have an understandable label.

The application should explain settings whose effects are not obvious.

Settings should have reasonable defaults.

Optional settings should remain optional.

Invalid values should be handled safely.

Settings should not silently change unrelated behavior.

### 24.2 Persistence

Settings may need to persist between sessions.

Persistence behavior should be documented.

Failed writes should produce appropriate errors.

The application should avoid replacing valid settings with incomplete data.

Settings should be loaded and validated before they are relied upon.

Schema changes should be managed deliberately.

### 24.3 Defaults and recovery

The application should define how it behaves when no settings file exists.

It should define how it behaves when a settings file is invalid.

It should avoid crashing unnecessarily because an optional preference is missing.

It should not silently ignore a serious configuration failure.

Where practical, the application should offer a clear recovery path.

### 24.4 Settings privacy

Settings should not collect unnecessary personal information.

Sensitive settings should be handled appropriately.

The application should not expose secrets in logs.

If a setting controls network behavior, the effect should be documented.

If a setting controls data retention, the effect should be documented.

### 24.5 Reset behavior

A reset option, if implemented, should explain what it resets.

Resetting interface preferences should not silently delete learning progress.

Resetting learning progress should require clear user intent.

Destructive actions should be confirmed when appropriate.

The application should avoid deleting unknown files during cleanup.

**Related documentation:** [Architecture](docs/ARCHITECTURE.md), [Privacy and Data Handling](docs/PRIVACY_AND_DATA_HANDLING.md), [Testing](docs/TESTING.md).

[Back to top](#new-bearings)

---

## 25. Storage and User Data

Storage behavior is important because learning history and preferences may represent substantial user effort.

The application should handle stored data deliberately.

### 25.1 Data categories

Depending on implementation, local data may include settings.

It may include learning progress.

It may include review history.

It may include achievements.

It may include content-related state.

It may include user-selected preferences.

It may include logs needed for troubleshooting.

The application should document which categories are actually stored.

### 25.2 Storage boundaries

Application code should have a clear understanding of where data is stored.

The application should avoid hard-coding a developer's personal directories.

Paths should be constructed using appropriate platform-aware mechanisms.

The application should handle missing directories.

The application should handle insufficient permissions.

The application should handle disk-related errors.

The application should not assume that every user has the same drive layout.

### 25.3 Data validation

Stored data should be validated before it is used.

Missing optional fields should be handled according to documented defaults.

Invalid required fields should produce a useful error.

Unexpected values should not cause silent data corruption.

Corrupt files should not be treated as valid merely because they can be opened.

Validation should occur at appropriate boundaries.

### 25.4 Safe writes

Where practical, storage should reduce the risk of partial writes.

A failed write should not be reported as successful.

The application should consider whether a backup or recovery mechanism is appropriate for important data.

Temporary files should be handled carefully.

Cleanup should not delete unrelated files.

Storage behavior should be tested under failure conditions where practical.

### 25.5 Data migrations

Stored formats may change over time.

Migrations should be explicit.

Migrations should be tested.

The application should avoid silently discarding data it does not understand.

A migration should preserve existing information where practical.

The application should explain unrecoverable migration failures.

Compatibility decisions should be documented.

### 25.6 Export and deletion

Data export may be useful for portability and backups.

Data deletion may be important for user control.

These operations should have clear scope.

An export should not include unexpected sensitive information.

A deletion operation should explain which data it affects.

The application should distinguish deletion of local application data from deletion of unrelated backups.

The exact export and deletion capabilities must be documented only after implementation.

**Related documentation:** [Privacy and Data Handling](docs/PRIVACY_AND_DATA_HANDLING.md), [Architecture](docs/ARCHITECTURE.md), [Testing](docs/TESTING.md).

[Back to top](#new-bearings)

---

## 26. Privacy and Data Handling

New Bearings should treat user data with care.

Privacy expectations should be based on actual application behavior rather than assumptions about how the software works.

### 26.1 Data minimization

The application should collect or store only information needed for defined functionality.

It should not collect personal information merely because it might be useful later.

It should not require sensitive disclosures to enable ordinary learning features.

It should not infer medical conditions from learning preferences.

It should not collect unrelated information about the user's device.

### 26.2 Local data

Where data is stored locally, the application should document the relevant storage behavior.

Local storage can reduce the need to transmit information to external services.

Local storage does not automatically make an application completely secure.

Other software running on the device may have access to files according to operating-system permissions.

Users may back up their devices to external services.

The application should avoid making absolute security claims about local data.

### 26.3 Network behavior

Any network-dependent feature should be documented.

The application should explain the purpose of network requests.

It should identify external services where appropriate.

It should distinguish essential network access from optional network access.

It should not silently upload learning history.

It should not silently introduce telemetry.

It should not transmit personal data without a clear, documented purpose and appropriate user control.

### 26.4 Third-party services

If a future feature uses an external service, the application should explain the dependency.

It should identify the data that the service receives where that information is known.

It should explain whether the service requires an account.

It should explain whether the service may cost money.

It should explain whether a local or offline alternative exists, when practical.

It should not claim that a third-party service follows a particular privacy practice without reliable evidence.

### 26.5 Logs

Logs should be useful for debugging.

Logs should not expose secrets unnecessarily.

Logs should avoid unnecessary personal information.

Sensitive values should be redacted where appropriate.

Error messages should avoid exposing irrelevant absolute paths.

Diagnostic reports should be reviewed before they are shared when they may contain private information.

### 26.6 User control

Users should receive clear information about data handling.

Where implemented, export and deletion options should have understandable behavior.

Settings should not imply privacy protections that do not exist.

Data retention behavior should be documented.

The application should avoid keeping data indefinitely without a defined reason.

### 26.7 Privacy limitations

This README is not a substitute for the detailed privacy policy.

It does not establish that the application has undergone a privacy audit.

It does not establish that the application complies with every privacy law.

It does not guarantee that all future integrations will have identical data-handling behavior.

Actual behavior should be reviewed as the project evolves.

**Related documentation:** [Privacy Policy](PRIVACY_POLICY.md), [Privacy and Data Handling](docs/PRIVACY_AND_DATA_HANDLING.md), [Security Policy](docs/SECURITY_POLICY.md).

[Back to top](#new-bearings)

---

## 27. Licensing and Third-Party Software

Licensing must be handled deliberately before public distribution.

The repository's current license metadata and license files should be checked for consistency before a release.

A license declaration should not be treated as proof that every file in the repository can legally be distributed under that license.

### 27.1 Project license

The project's intended license must be confirmed.

The package metadata, root license file, repository metadata, and release documentation should agree.

A license file must contain the actual license text or other appropriate legal content.

A placeholder file is not an adequate substitute for a completed license.

The project should not claim that its licensing is finalized until the relevant files have been reviewed.

### 27.2 Dependencies

Rust dependencies may have different licenses.

Dependency licenses should be reviewed before distribution.

The project should preserve required notices.

The project should not assume that every dependency uses the same license.

The project should not assume that every license permits every possible form of redistribution.

### 27.3 Content licenses

Learning content may have licensing requirements separate from the application code.

Code examples may have different origins.

Images and other resources may have separate terms.

External documentation may not be freely reproducible.

Contributors should identify the origin of third-party material.

Unverified material should not be presented as though its redistribution rights are known.

### 27.4 Notices and attribution

Attribution should be provided when required.

Required license notices should be preserved.

Third-party notices should be kept consistent with the distributed software.

If the project begins bundling additional assets, the license review should be updated.

Automated license reports may help, but they do not replace review of ambiguous cases.

### 27.5 Release requirements

Before a public release, verify the actual license file.

Verify package metadata.

Review dependency licenses.

Review bundled resources.

Review attribution requirements.

Document any unresolved licensing questions.

Do not distribute material with unresolved restrictions as though its licensing were settled.

### 27.6 Legal limitations

This section provides development guidance rather than legal advice.

The final licensing decision should be made deliberately.

Where necessary, seek qualified legal advice.

**Related documentation:** [Security Policy](docs/SECURITY_POLICY.md), [Release Plan](docs/RELEASE_PLAN.md), [Contributing](CONTRIBUTING.md).

[Back to top](#new-bearings)

---

## 28. Security and Responsible Development

Security should be considered throughout development.

The application should not rely on the assumption that educational software has no meaningful security risks.

File handling, dependency management, stored data, external integrations, and future code execution features can all introduce risks.

### 28.1 Secure defaults

Defaults should avoid unnecessary exposure.

Optional features should not be enabled silently when doing so creates meaningful risk.

The application should avoid unnecessary privileges.

The application should not disable operating-system protections to simplify installation.

The application should not expose secrets in logs.

The application should not execute arbitrary imported content without a deliberate and reviewed design.

### 28.2 Dependency security

Dependencies should be reviewed.

Known vulnerabilities should be evaluated.

Updates should be tested.

Unmaintained dependencies should be considered carefully.

A dependency should not be treated as safe solely because it is available through a package registry.

The project should document relevant security limitations.

### 28.3 Imported content

Content packs may contain malformed data.

Imported files should be validated.

Unexpected fields should be handled according to the content format's rules.

File paths should not be trusted merely because they came from a content pack.

Content loading should not execute scripts unless a separately designed, reviewed capability explicitly requires it.

Invalid content should not be allowed to silently corrupt unrelated user data.

### 28.4 Future code execution

If New Bearings eventually executes learner-written code, that feature will need additional security design.

Running code can access resources according to the permissions of its process.

A subprocess is not automatically a secure sandbox.

A timeout alone does not prevent every form of resource exhaustion.

The application should define the execution environment.

It should consider resource limits.

It should handle process termination.

It should avoid exposing unnecessary files or credentials.

It should document the limitations of any isolation mechanism.

### 28.5 Reporting security issues

Security issues should be reported through an appropriate private channel where possible.

Reports should include enough detail to reproduce and assess the issue.

Sensitive exploit details should not be disclosed publicly before the issue has been assessed when doing so could put users at risk.

The project should communicate relevant fixes and limitations responsibly.

### 28.6 Security claims

Do not claim that the application is completely secure.

Do not claim that a dependency scanner proves the absence of vulnerabilities.

Do not claim that a test suite proves that every attack is prevented.

Security assurance depends on scope, design, implementation, testing, and ongoing maintenance.

**Related documentation:** [Security Policy](docs/SECURITY_POLICY.md), [Privacy and Data Handling](docs/PRIVACY_AND_DATA_HANDLING.md), [Testing](docs/TESTING.md).

[Back to top](#new-bearings)

---

## 29. Error Handling and Reliability

Errors are an expected part of software operation.

A reliable application handles them deliberately and communicates them clearly.

### 29.1 Useful error messages

An error should explain what failed when that information is available.

It should avoid vague messages when a more specific explanation is possible.

It should identify a practical next step when appropriate.

It should distinguish user-correctable problems from internal failures.

It should avoid exposing secrets.

It should preserve useful technical details for debugging.

### 29.2 Error categories

Possible error categories include invalid input.

They may include missing files.

They may include inaccessible files.

They may include malformed content.

They may include invalid settings.

They may include storage failures.

They may include unsupported operations.

They may include unavailable optional dependencies.

They may include unexpected internal failures.

The actual error model should be documented according to the implementation.

### 29.3 Recovery

Where practical, the application should recover from errors without discarding unrelated work.

A missing optional resource should not necessarily prevent the entire application from starting.

A corrupt optional content pack should not necessarily corrupt other packs.

A failed progress write should not be reported as successful.

A failed settings migration should not silently replace the user's settings with unrelated values.

Recovery behavior should be tested.

### 29.4 Logging

Logs should help developers identify problems.

They should contain useful context without unnecessary private information.

Error levels should be used consistently.

Repeated failures should not produce uncontrolled log growth.

Logs should not be treated as a substitute for user-facing error messages.

### 29.5 Reliability testing

Tests should cover normal behavior.

Tests should cover invalid inputs.

Tests should cover expected failures.

Tests should cover storage errors where practical.

Tests should cover incomplete data.

Tests should cover recovery behavior.

Regression tests should be added when a defect is fixed and the relevant behavior can be tested.

**Related documentation:** [Testing](docs/TESTING.md), [Architecture](docs/ARCHITECTURE.md), [Development](docs/DEVELOPMENT.md).

[Back to top](#new-bearings)

---

## 30. Performance and Resource Use

New Bearings should aim to remain responsive and practical to use.

Performance should be measured rather than assumed.

The appropriate performance target depends on the operation, platform, hardware, and implementation.

### 30.1 Responsiveness

Routine navigation should avoid unnecessary delays.

Long-running operations should not block essential interaction when a suitable alternative exists.

The application should provide meaningful feedback for lengthy operations where appropriate.

Progress indicators should represent real work.

The application should avoid fake progress percentages.

### 30.2 Memory use

Memory use should be monitored when it becomes relevant to application behavior.

The application should avoid retaining unnecessary copies of large data structures.

Data should have a clear lifetime.

Caching should be justified by measurable benefits.

Memory use should be tested with representative workloads where practical.

### 30.3 CPU use

CPU-intensive operations should be identified.

The application should avoid unnecessary repeated work.

Background processing may be appropriate for operations that would otherwise block the interface.

Concurrency should be introduced only when it improves the design and can be tested.

The application should avoid creating uncontrolled background tasks.

### 30.4 Storage use

Persistent data should have a defined purpose.

Temporary files should be managed carefully.

Caches should not grow without a reason.

The application should avoid deleting unknown files during cleanup.

Large content packs should be handled according to documented limits.

### 30.5 Measurement

Performance reports should identify the tested environment.

They should describe the operation being measured.

They should state the measurement method.

They should distinguish a single observation from a reliable benchmark.

They should not invent results.

They should not compare measurements gathered under materially different conditions without explaining the differences.

**Related documentation:** [Testing](docs/TESTING.md), [Development](docs/DEVELOPMENT.md), [Release Plan](docs/RELEASE_PLAN.md).

[Back to top](#new-bearings)

---

## 31. Installation and Getting Started

Installation instructions must match the actual release artifacts and build configuration.

Because the project is in early development, the availability of a ready-to-install release should not be assumed.

The current installation guide should be treated as the detailed source for verified setup instructions.

### 31.1 Before installing

Check the current project status.

Check the supported operating system.

Check whether a release artifact exists.

Check the release notes.

Check any documented prerequisites.

Check whether the installation method requires the Rust toolchain.

Check whether any optional dependency is needed for the workflow you intend to use.

Do not assume that an installer exists merely because installation documentation exists.

### 31.2 Development setup versus normal installation

A development setup is intended for people building or modifying the source code.

A normal installation is intended for people using a packaged application.

These are different workflows.

A developer may need Rust and Cargo.

A normal user should not be required to install a development toolchain unless the distribution method explicitly requires it.

Installation instructions should make this distinction clear.

### 31.3 Source-based setup

A source-based workflow typically requires access to the repository and the relevant build tools.

The exact commands should be verified against the current project.

The project should not publish guessed commands as though they have been tested.

Build dependencies should be documented.

Platform-specific requirements should be explained.

### 31.4 First launch

First-launch behavior should be documented when verified.

The application should provide understandable initial navigation.

Required setup should be distinguished from optional preferences.

Missing optional resources should be handled sensibly.

First-launch errors should explain what the user can do next.

### 31.5 Installation problems

If installation fails, capture the exact error message.

Record the operating system and relevant tool versions.

Identify whether the failure occurs during dependency installation, compilation, packaging, or launch.

Avoid changing multiple unrelated components before identifying the likely cause.

Use the troubleshooting guidance in the installation and development documentation.

### 31.6 Removal

Removal instructions should match the actual installation method.

An installed application may store user data separately from its executable files.

The documentation should explain the difference where applicable.

Users should not be told that uninstalling automatically deletes every related file unless that behavior has been verified.

User data should not be removed unexpectedly.

**Related documentation:** [Installation](docs/INSTALLATION.md), [Development](docs/DEVELOPMENT.md), [Release Plan](docs/RELEASE_PLAN.md).

[Back to top](#new-bearings)

---

## 32. Development Environment

Development instructions should help contributors establish a predictable working environment.

The current developer workflow uses Windows, VS Code, Git, and Rust tooling.

Exact requirements should be verified against the current repository.

### 32.1 Required tools

A development environment generally needs Git.

It needs a compatible Rust toolchain.

It needs Cargo, which is distributed with the standard Rust toolchain.

It needs an editor or IDE.

It may need platform-specific build tools depending on the selected dependencies.

The installation guide should identify the actual prerequisites.

### 32.2 Repository access

The repository is hosted on GitHub.

The current repository URL is:

<https://github.com/Pandagamerteen08/New-Bearings>

The default branch is `main`.

Contributors should confirm that they are working in the intended repository.

They should inspect the current branch before making significant changes.

They should avoid committing unrelated personal files.

### 32.3 Initial inspection

Before changing code, inspect the current project structure.

Read the relevant documentation.

Review the module related to the intended change.

Identify existing tests.

Check the working tree.

Understand the expected behavior before editing.

Avoid rewriting unrelated modules without a reason.

### 32.4 Incremental development

Make one coherent change at a time.

Build or test the relevant area.

Review the result.

Fix regressions before adding unrelated changes.

Update documentation when behavior changes.

Keep the working tree understandable.

Use commits that explain the purpose of the change.

### 32.5 Editor configuration

Editor settings should support a consistent workflow.

Project-wide settings should be committed only when they are useful to contributors.

Personal settings should not be required to build the application.

Generated editor files should not be committed unnecessarily.

The repository's `.gitignore` should reflect the intended sharing policy.

### 32.6 Environment-specific paths

Do not hard-code the original developer's absolute project path.

Do not assume the repository is on a particular drive.

Use relative paths for project resources where appropriate.

Use platform-aware path handling for runtime data.

Document paths that genuinely need to be configured.

**Related documentation:** [Development](docs/DEVELOPMENT.md), [Installation](docs/INSTALLATION.md), [Contributing](CONTRIBUTING.md).

[Back to top](#new-bearings)

---

## 33. Building the Application

The application should be built using the project's Rust and Cargo configuration.

The commands below are standard starting points. They should be run from the repository root and verified against the current project configuration.

### 33.1 Check the toolchain

```bash
rustc --version
cargo --version
```

These commands report the installed Rust compiler and Cargo versions.

If either command is unavailable, the development environment may not be configured correctly.

Use the official Rust installation guidance to establish a compatible toolchain.

### 33.2 Check the project

```bash
cargo check
```

This command checks the project for compilation errors without producing a normal final executable in the same way as a full build.

A successful check is useful, but it does not prove that every runtime workflow behaves correctly.

### 33.3 Build a development executable

```bash
cargo build
```

This builds the project in the development profile.

The exact output depends on the package and binary configuration.

Build output is normally placed under Cargo's `target` directory.

Generated build output should not normally be committed to the repository.

### 33.4 Build an optimized executable

```bash
cargo build --release
```

This builds the project using the release profile.

An optimized build should be verified independently when preparing a release.

A successful development build does not guarantee that packaging or release-specific behavior works correctly.

### 33.5 Formatting

```bash
cargo fmt --check
```

This checks whether Rust source files are formatted according to rustfmt's rules.

If formatting changes are needed, run:

```bash
cargo fmt
```

Review the resulting diff before committing.

Formatting changes should not be mixed with unrelated behavioral changes when doing so would make review harder.

### 33.6 Linting

```bash
cargo clippy --all-targets
```

Clippy can identify common mistakes and potential improvements.

Its warnings should be evaluated rather than blindly suppressed.

A warning may reveal a real problem.

A warning may also require a context-specific decision.

Suppressions should be justified.

### 33.7 Build verification

After a build, inspect the command's exit status.

Read any reported warnings or errors.

Confirm that the intended target was built.

Run the relevant tests.

Do not report a successful build if the command failed.

Do not infer a successful build from the existence of an older executable.

### 33.8 Build documentation

The detailed development guide should remain aligned with the actual build process.

If required tools change, update the guide.

If platform-specific dependencies are introduced, document them.

If release packaging uses additional commands, document those separately.

**Related documentation:** [Development](docs/DEVELOPMENT.md), [Installation](docs/INSTALLATION.md), [Testing](docs/TESTING.md).

[Back to top](#new-bearings)

---

## 34. Running Tests

Tests help verify that application behavior matches expectations.

They should be treated as an essential part of development.

### 34.1 Run the test suite

```bash
cargo test
```

Run this command from the repository root.

The command compiles and executes the applicable Rust tests.

The result should be reviewed rather than assumed.

A failed test should be investigated.

A test that was not executed should not be reported as passing.

### 34.2 Run checks together

A useful local verification sequence is:

```bash
cargo fmt --check
cargo check
cargo clippy --all-targets
cargo test
```

This sequence is a starting point for local verification.

The project's actual continuous integration workflow may use additional targets or flags.

The commands should be adapted when the repository's requirements change.

### 34.3 Test results

Record the commands that were run.

Record their actual results.

Distinguish warnings from errors.

Distinguish failed tests from skipped tests.

Do not delete a failing test simply to obtain a passing result.

Do not weaken a test without understanding what behavior it is supposed to protect.

Do not claim that a test ran if it was not executed.

### 34.4 Test scope

A passing unit test does not prove that the entire application works.

A passing test suite does not prove that every platform works.

A passing CI run does not prove that all possible user inputs are handled correctly.

Tests provide evidence within their defined scope.

The project should describe that scope accurately.

### 34.5 When a test fails

Read the full error message.

Identify the failing test.

Inspect the relevant code.

Determine whether the failure is caused by the implementation, test assumptions, environment, or dependency behavior.

Reproduce the problem where practical.

Fix the underlying issue.

Re-run the relevant test.

Run broader checks before declaring the change complete.

**Related documentation:** [Testing](docs/TESTING.md), [Development](docs/DEVELOPMENT.md).

[Back to top](#new-bearings)

---

## 35. Testing Strategy

Testing should evolve with the application.

The objective is to verify meaningful behavior and reduce regressions.

### 35.1 Unit tests

Unit tests should verify small pieces of behavior.

They should cover important valid inputs.

They should cover invalid inputs where relevant.

They should verify boundary conditions.

They should avoid depending unnecessarily on unrelated modules.

They should be readable enough that failures are understandable.

### 35.2 Integration tests

Integration tests should verify interactions between components.

They may cover content loading.

They may cover settings persistence.

They may cover progress updates.

They may cover storage operations.

They may cover error propagation.

They may cover navigation behavior where suitable.

The actual test categories should match the implementation.

### 35.3 Data tests

Stored data should be tested with valid examples.

Missing optional fields should be considered.

Invalid values should be considered.

Malformed files should be considered.

Schema migration should be tested when introduced.

Failure during storage operations should be considered where practical.

### 35.4 Content tests

Content tests should check required fields.

They should check identifiers.

They should check prerequisite references where applicable.

They should check malformed content.

They should check supported content versions.

They should check whether content references required resources correctly.

### 35.5 Accessibility tests

Accessibility testing should include manual checks.

Keyboard navigation should be checked.

Focus order should be checked.

Control labels should be checked.

Text scaling should be checked where supported.

Error messages should be checked.

Important information should not rely exclusively on color.

The tests should reflect the actual interface technology.

### 35.6 Platform tests

Platform-specific behavior should be tested on the relevant platform.

A Windows test should not automatically be treated as a macOS test.

A Linux compilation should not automatically establish correct behavior on every Linux distribution.

Platform-specific installation instructions should be verified.

### 35.7 Regression tests

When a defect is fixed, consider adding a test that would detect the same defect in the future.

A regression test should verify the relevant behavior.

It should not merely reproduce the internal implementation.

Existing tests should be preserved unless there is a clear reason to replace them.

### 35.8 Test quality

Tests should be deterministic where practical.

Tests should avoid unnecessary dependence on the network.

Tests should clean up resources they create.

Tests should not modify real user data.

Tests should avoid relying on personal absolute paths.

Test failures should be diagnosable.

### 35.9 Release testing

Release verification should include the actual artifact when practical.

The project should confirm that the artifact exists.

It should confirm that the artifact is the expected type.

It should test important startup and application workflows.

It should record known limitations.

It should not claim successful release verification when only a source build was performed.

**Related documentation:** [Testing](docs/TESTING.md), [Release Plan](docs/RELEASE_PLAN.md), [Accessibility](ACCESSIBILITY.md).

[Back to top](#new-bearings)

---

## 36. Debugging and Troubleshooting

Debugging should begin with evidence.

The first error shown is not always the root cause.

A useful investigation identifies the failing operation, its context, and the conditions under which it occurs.

### 36.1 General troubleshooting process

Reproduce the problem when practical.

Record the exact error message.

Identify what action triggered the error.

Identify whether the problem occurs consistently.

Check whether the working tree contains recent changes.

Inspect relevant logs.

Inspect the code associated with the failing behavior.

Identify the likely cause before making broad changes.

Apply a targeted fix.

Re-run the relevant test or workflow.

### 36.2 Compilation errors

Check the first meaningful compiler error.

Identify the file and location.

Read the surrounding code.

Check whether the error is a consequence of an earlier failure.

Check whether imports and type definitions are correct.

Check whether the compiler version is compatible with the project.

Avoid changing unrelated files to address one compilation error.

### 36.3 Dependency errors

Check whether dependency resolution succeeded.

Check whether the required toolchain is available.

Check whether the error indicates a network problem.

Check whether the error is specific to one platform.

Check whether a dependency version changed.

Do not assume every dependency error is caused by application source code.

### 36.4 Runtime errors

Identify the operation that failed.

Determine whether the error is recoverable.

Check whether required resources exist.

Check whether the application has appropriate permissions.

Check whether the relevant settings are valid.

Check whether the problem is reproducible with a clean configuration when appropriate.

### 36.5 Storage errors

Check the path involved.

Check whether the directory exists.

Check whether the application has permission to access it.

Check whether the disk is available.

Check whether the data is valid.

Avoid deleting user data as a first troubleshooting step.

Preserve backups before attempting risky recovery.

### 36.6 Reporting a useful bug

Include the expected behavior.

Include the actual behavior.

Include reproduction steps.

Include relevant error messages.

Include the operating system.

Include relevant tool versions.

Remove secrets and unnecessary personal information.

Do not include private learning history unless it is necessary and has been appropriately reviewed.

**Related documentation:** [Development](docs/DEVELOPMENT.md), [Testing](docs/TESTING.md), [Security Policy](docs/SECURITY_POLICY.md).

[Back to top](#new-bearings)

---

## 37. Repository Structure

The repository is organized into source code, tests, content-related resources, documentation, and project configuration.

The structure may evolve as the application grows.

The following tree reflects the known organization at the time this README was prepared. It is a high-level reference, not a guarantee that every directory contains a completed feature.

```text
New-Bearings/
├── .github/
│   └── workflows/
│       └── ci.yml
├── .gitignore
├── .editorconfig
├── ACCESSIBILITY.md
├── CHANGELOG.md
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── DISCLAIMER.md
├── LICENSE
├── PRIVACY_POLICY.md
├── README.md
├── SECURITY.md
├── TERMS_OF_SERVICE.md
├── Cargo.toml
├── Cargo.lock
├── docs/
│   ├── ARCHITECTURE.md
│   ├── CONTENT_FORMAT.md
│   ├── DECISIONS.md
│   ├── DEVELOPMENT.md
│   ├── INSTALLATION.md
│   ├── PRIVACY_AND_DATA_HANDLING.md
│   ├── RELEASE_PLAN.md
│   ├── ROADMAP.md
│   ├── SECURITY_POLICY.md
│   └── TESTING.md
├── content/
├── resources/
├── schemas/
├── scripts/
├── src/
│   ├── accessibility/
│   ├── achievements/
│   ├── app/
│   ├── content_packs/
│   ├── errors/
│   ├── learning/
│   ├── lessons/
│   ├── navigation/
│   ├── platform/
│   ├── progress/
│   ├── reminders/
│   ├── resources/
│   ├── review/
│   ├── rewards/
│   ├── settings/
│   ├── setup/
│   ├── storage/
│   └── ui/
├── tests/
└── target/
```

The exact filenames and subdirectories should be checked against the working tree before this tree is used as a definitive inventory.

Generated build output such as `target/` is not normally committed to version control.

### 37.1 Root files

`README.md` is the primary project overview.

`Cargo.toml` contains package metadata and dependency configuration.

`Cargo.lock` records resolved dependency versions for the application package.

`.gitignore` specifies files and directories that Git should ignore.

`.editorconfig` defines editor settings that supported editors can use.

`CHANGELOG.md` records relevant changes across versions.

`CONTRIBUTING.md` explains contribution expectations.

`CODE_OF_CONDUCT.md` describes community behavior expectations.

`ACCESSIBILITY.md` documents accessibility commitments and guidance.

`PRIVACY_POLICY.md` describes the intended privacy policy and should match actual behavior.

`TERMS_OF_SERVICE.md` is a separate legal document whose final contents require deliberate review.

`DISCLAIMER.md` contains disclaimer-related material and should be consistent with the application's actual scope.

`LICENSE` should contain the finalized project license rather than placeholder text.

`SECURITY.md` provides security-reporting guidance.

### 37.2 Documentation directory

The `docs/` directory contains detailed technical and project documentation.

`ARCHITECTURE.md` explains the architecture and module boundaries.

`CONTENT_FORMAT.md` describes the learning content format.

`DECISIONS.md` records important design decisions.

`DEVELOPMENT.md` explains the developer workflow.

`INSTALLATION.md` documents installation and setup.

`PRIVACY_AND_DATA_HANDLING.md` explains data handling in greater technical detail.

`RELEASE_PLAN.md` describes release planning and verification.

`ROADMAP.md` records planned development priorities.

`SECURITY_POLICY.md` documents security-related expectations and reporting.

`TESTING.md` describes the test strategy and relevant commands.

### 37.3 Source directory

The `src/` directory contains Rust source code.

Source modules should have clear responsibilities.

Shared functionality should be placed where it can be reused appropriately.

The source structure should remain consistent with the architecture documentation.

### 37.4 Tests directory

The `tests/` directory contains integration tests or other external test files as appropriate.

Unit tests may also be located alongside Rust source code.

The location of a test should follow the project's conventions.

### 37.5 Content and resources

The `content/` directory is associated with learning content.

The `resources/` directory is associated with application resources.

The `schemas/` directory is associated with data or content schemas.

The `scripts/` directory is associated with development or maintenance scripts.

Their precise contents and responsibilities should be documented as they evolve.

### 37.6 Generated files

Generated output should be distinguished from maintained source.

Build artifacts should not be committed unnecessarily.

Temporary files should not be committed.

Personal machine configuration should not be committed unless explicitly intended.

Secrets should never be committed.

The repository's ignore rules should be reviewed whenever new generated directories are introduced.

**Related documentation:** [Architecture](docs/ARCHITECTURE.md), [Development](docs/DEVELOPMENT.md), [Contributing](CONTRIBUTING.md).

[Back to top](#new-bearings)

---

## 38. Documentation Guide

Documentation should help users and contributors understand the project without needing to reconstruct every decision from source code.

Each document should have a clear purpose.

The same information should not be copied into multiple files without a reason.

### 38.1 Main README

This README provides the overall project introduction.

It describes the purpose and philosophy.

It explains the intended feature direction.

It provides a high-level architecture overview.

It links to detailed documentation.

It explains how to get started.

It describes development expectations.

It identifies known limitations and the provisional project name.

### 38.2 Architecture documentation

The architecture document should explain the system's major components.

It should explain how components communicate.

It should identify important data flows.

It should explain storage boundaries.

It should explain the role of platform-specific code.

It should identify important constraints.

It should be updated when significant architecture changes occur.

### 38.3 Content format documentation

The content format document should describe the actual structure of supported learning content.

It should explain required fields.

It should explain optional fields.

It should explain identifier rules.

It should explain prerequisite references where applicable.

It should explain validation behavior.

It should explain compatibility expectations.

It should include accurate examples.

### 38.4 Development documentation

The development guide should explain the environment and workflow.

It should list verified prerequisites.

It should provide verified commands.

It should explain how to run checks.

It should explain how to run tests.

It should explain common development problems.

It should avoid relying on a particular developer's personal paths.

### 38.5 Installation documentation

The installation guide should distinguish source setup from normal installation.

It should identify actual prerequisites.

It should describe supported installation methods.

It should explain first launch where verified.

It should explain removal where appropriate.

It should document relevant platform limitations.

### 38.6 Testing documentation

The testing guide should describe test categories.

It should explain the standard test commands.

It should identify known limitations.

It should explain how to add tests.

It should explain how to interpret results.

It should distinguish automated checks from manual verification.

### 38.7 Privacy documentation

The privacy documents should reflect actual data behavior.

They should identify relevant data categories.

They should describe storage behavior.

They should describe network behavior.

They should describe external services where applicable.

They should explain relevant limitations.

They should be reviewed whenever data-handling behavior changes.

### 38.8 Security documentation

The security documents should explain how to report security issues.

They should identify appropriate reporting channels.

They should describe supported security expectations.

They should not promise an unverified response time.

They should be updated when reporting procedures change.

### 38.9 Roadmap and release documents

The roadmap should identify priorities and planned work.

The release plan should describe release readiness and verification.

The two documents should not contradict one another.

A roadmap item should not be presented as completed without evidence.

A release should not be declared ready merely because a planned date has arrived.

### 38.10 Documentation maintenance

Links should be checked after files move.

File names should remain consistent.

Claims should be reviewed after implementation changes.

Commands should be tested where practical.

Known limitations should be updated.

Documentation should be reviewed as part of significant feature changes.

**Related documentation:** [Architecture](docs/ARCHITECTURE.md), [Content Format](docs/CONTENT_FORMAT.md), [Development](docs/DEVELOPMENT.md), [Testing](docs/TESTING.md).

[Back to top](#new-bearings)

---

## 39. Content Packs and Extensibility

Content packs may provide a way to organize and distribute learning material independently of the application's core implementation.

Their exact capabilities should be defined by the supported content format.

### 39.1 Purpose of content packs

Content packs may group related lessons.

They may organize topics.

They may define prerequisites.

They may include examples and exercises.

They may provide metadata for a learning path.

They may allow content to be updated independently of some application code.

The exact supported behavior must be verified before being advertised.

### 39.2 Validation

Content packs should be validated before their contents are used.

Required fields should be checked.

Identifiers should be checked.

References should be checked.

Unsupported format versions should be handled explicitly.

Malformed files should produce useful errors.

Invalid content should not silently overwrite unrelated data.

### 39.3 Compatibility

The content format should have documented compatibility rules.

Changes to required fields should be handled deliberately.

Older content should not silently be interpreted incorrectly.

Version information should be clear.

Migration behavior should be documented when applicable.

### 39.4 Extensibility

New subjects should be able to reuse appropriate core services.

Subject-specific logic should remain separate when it has distinct requirements.

Extensions should not require unrelated modules to know their internal details.

Optional capabilities should be represented explicitly.

Unsupported content types should not be treated as supported merely because they can be loaded as text.

### 39.5 Content trust

Imported content should not automatically be considered trustworthy.

Content should not execute arbitrary code merely because it is included in a pack.

Resource references should be validated.

Unexpected paths should be handled safely.

The application should preserve user data when content validation fails.

### 39.6 Contribution quality

Contributed content should have a clear learning objective.

It should use the documented format.

It should be checked for accuracy.

It should identify relevant prerequisites.

It should include appropriate explanations.

It should avoid unlicensed material.

It should not describe unsupported application behavior as though it exists.

**Related documentation:** [Content Format](docs/CONTENT_FORMAT.md), [Architecture](docs/ARCHITECTURE.md), [Contributing](CONTRIBUTING.md).

[Back to top](#new-bearings)

---

## 40. Versioning and Compatibility

Versioning communicates changes to users and contributors.

The project should use a consistent versioning policy as releases mature.

### 40.1 Application versions

The application version should be defined through the project's package metadata.

Version changes should be deliberate.

The version should not be changed merely to make a release appear more substantial.

The changelog should explain meaningful changes.

### 40.2 Compatibility changes

A change may affect stored data.

A change may affect content packs.

A change may affect configuration.

A change may affect command-line behavior.

A change may affect platform compatibility.

These effects should be considered before a release.

### 40.3 Data compatibility

Stored data formats should be versioned when necessary.

Migrations should be tested.

Users should not be expected to understand internal schema changes.

A migration failure should not silently destroy existing data.

### 40.4 Content compatibility

Content pack versions should have documented rules.

The application should identify unsupported versions.

Validation errors should explain the problem.

Compatibility should be tested with representative content.

### 40.5 Release notes

Release notes should describe user-visible changes.

They should identify important fixes.

They should identify known limitations.

They should explain breaking changes.

They should avoid claiming features that were not included in the release.

### 40.6 Reproducibility

The project should record relevant build and dependency information.

Build procedures should be documented.

Release artifacts should be identifiable.

Checksums may be provided where appropriate.

The project should not claim reproducibility without having established the relevant process.

**Related documentation:** [Changelog](CHANGELOG.md), [Release Plan](docs/RELEASE_PLAN.md), [Development](docs/DEVELOPMENT.md).

[Back to top](#new-bearings)

---

## 41. Roadmap and Development Priorities

The roadmap should guide development without pretending that every planned feature has a fixed completion date.

Priorities may change when testing reveals technical constraints or user needs.

### 41.1 Foundation

The initial priority is a maintainable Rust application foundation.

The application should build reliably in the supported development environment.

The module structure should be understandable.

Error handling should be consistent.

Core configuration should be predictable.

The test workflow should be documented.

The repository should have clear contribution guidance.

### 41.2 Core learning workflow

A coherent learning workflow should take priority over disconnected feature additions.

The application should provide a clear route into learning content.

It should present content in a usable format.

It should support the relevant learner interaction.

It should communicate the result.

It should handle progress consistently where progress tracking is implemented.

It should handle errors without unnecessary data loss.

### 41.3 Content quality

Content structure should be documented.

Content validation should be tested.

Lesson objectives should be clear.

Examples should be accurate.

Prerequisites should be represented appropriately.

The project should avoid expanding the catalogue at the expense of quality.

### 41.4 Accessibility and personalization

Accessibility improvements should be incorporated into relevant features.

Preferences should remain understandable.

Optional settings should not create unnecessary barriers.

Keyboard navigation should be evaluated.

The project should test actual user workflows rather than relying only on design intentions.

### 41.5 Persistence and reliability

Settings should behave predictably.

Progress persistence should be verified.

Storage errors should be handled clearly.

Data migrations should be tested when introduced.

Recovery behavior should be considered before major storage changes.

### 41.6 Review and retention

Review functionality should be developed when the underlying content and progress models can support it.

Review rules should be understandable.

Recommendations should be appropriately qualified.

Progress should not be equated automatically with mastery.

### 41.7 Broader language support

Additional programming languages should be introduced through a maintainable content strategy.

Language-specific behavior should be documented.

Language support should not be claimed merely because a language is mentioned in the README.

Tooling integrations should be evaluated separately.

### 41.8 Cross-platform development

Cross-platform support should be pursued through deliberate testing.

Platform-specific behavior should be isolated where practical.

Packaging should be verified separately for each supported platform.

Installation instructions should match the actual artifacts.

### 41.9 Release preparation

Release preparation should include code review.

It should include test verification.

It should include dependency and license review.

It should include documentation review.

It should include artifact verification.

It should include a review of known limitations.

### 41.10 Roadmap discipline

The roadmap should be updated when priorities change.

Completed work should be based on evidence.

Blocked work should identify its obstacle.

Planned work should not be described as delivered.

The project should prefer meaningful progress over arbitrary feature counts.

**Related documentation:** [Roadmap](docs/ROADMAP.md), [Release Plan](docs/RELEASE_PLAN.md), [Testing](docs/TESTING.md).

[Back to top](#new-bearings)

---

## 42. Contributing

Contributions can help improve code quality, accessibility, documentation, content, testing, and the overall learning experience.

Contributions should be focused and reviewable.

The contribution guide contains the detailed repository-specific process.

### 42.1 Areas for contribution

Potential contribution areas include Rust development.

They include bug fixes.

They include test improvements.

They include documentation corrections.

They include accessibility improvements.

They include content validation.

They include lesson development.

They include programming examples.

They include user-experience improvements.

They include platform compatibility work.

They include dependency and release maintenance.

The project may not be ready to accept every type of contribution at every stage.

### 42.2 Before contributing

Read the contribution guide.

Review the relevant documentation.

Inspect the current code.

Check whether the issue is already being addressed.

Understand the expected behavior.

Ask for clarification when requirements are genuinely unclear.

Avoid beginning a large rewrite based on an assumption.

### 42.3 Focused changes

A focused change is easier to review.

It is easier to test.

It is easier to explain.

It is easier to revert if necessary.

It reduces the risk of unrelated regressions.

Large changes may still be appropriate when the work genuinely requires them, but they should have a clear scope.

### 42.4 Code contributions

Code should follow the project's conventions.

Changes should preserve unrelated functionality.

New behavior should be tested where practical.

Error handling should remain consistent.

Public interfaces should not be changed casually.

Dependencies should be justified.

Documentation should be updated when behavior changes.

### 42.5 Content contributions

Content should be accurate.

It should have a clear learning objective.

It should use the supported format.

It should avoid unexplained prerequisites.

It should use appropriate terminology.

It should include attribution when needed.

It should be checked for misleading or ambiguous questions.

### 42.6 Documentation contributions

Documentation should match actual behavior.

Commands should be verified where practical.

Links should point to the intended files.

Claims should be qualified when verification is incomplete.

Examples should be understandable to their intended audience.

### 42.7 Review

Reviewers should focus on correctness, maintainability, scope, accessibility, and reliability.

Feedback should be specific.

Disagreement should focus on the technical or educational issue.

A contributor should not be expected to infer an unstated requirement.

A rejected proposal should receive a useful explanation where practical.

### 42.8 Contribution boundaries

A contribution does not authorize unrelated changes.

A bug fix should not silently remove a feature.

A refactor should not silently change user-facing behavior.

A documentation update should not invent completed functionality.

A dependency change should not be treated as risk-free.

**Related documentation:** [Contributing](CONTRIBUTING.md), [Code of Conduct](CODE_OF_CONDUCT.md), [Development](docs/DEVELOPMENT.md).

[Back to top](#new-bearings)

---

## 43. Code Style and Engineering Standards

Code quality affects how safely the project can evolve.

The goal is not to enforce style for its own sake.

The goal is to keep code understandable, testable, and maintainable.

### 43.1 Naming

Names should communicate purpose.

Functions should describe their behavior.

Types should represent meaningful concepts.

Variables should not use unnecessarily cryptic names.

Abbreviations should be used consistently.

Names should avoid implying behavior that the implementation does not provide.

### 43.2 Function responsibilities

Functions should have a clear purpose.

Very large functions should be reviewed for natural boundaries.

Abstraction should solve a real problem.

Small functions should not be created solely to increase the number of functions.

Shared logic should be factored out when doing so improves clarity and maintainability.

### 43.3 Error handling

Errors should not be silently discarded without justification.

Fallback behavior should be deliberate.

Error messages should preserve useful context.

Recoverable and unrecoverable failures should be distinguished where appropriate.

User-facing messages should be understandable.

### 43.4 Type safety

Types should communicate the meaning of data.

Invalid states should be difficult to represent when practical.

Conversions should be explicit when they affect correctness.

Optional values should be handled deliberately.

The application should avoid using loosely structured data when a clear type would be more maintainable.

### 43.5 Comments

Comments should explain reasoning that is not obvious from the code.

Comments should not simply repeat every line.

Comments should be updated when behavior changes.

Temporary notes should not be mistaken for implemented functionality.

Important constraints should be documented.

### 43.6 Dependencies

Dependencies should be justified.

Unnecessary dependencies should be avoided.

License compatibility should be reviewed.

Dependency updates should be tested.

Unmaintained packages should be evaluated carefully.

### 43.7 Refactoring

Refactoring should preserve behavior unless a behavior change is intentional.

The expected behavior should be understood before refactoring.

Relevant tests should be run.

Large changes should be divided where practical.

The diff should be reviewed for unrelated modifications.

### 43.8 No fake implementations

A stub should not be described as a working feature.

A placeholder should not be exposed as a completed workflow.

A control should not pretend to save data when it does not.

A test should not report success without checking the intended behavior.

Documentation should not conceal missing implementation.

### 43.9 Beginner-friendly development

The project should remain approachable for contributors who are still learning.

Complex decisions should be documented.

Useful error messages should be preserved.

Development instructions should be clear.

A contributor should be able to understand why an approach was chosen, not only what code to copy.

**Related documentation:** [Development](docs/DEVELOPMENT.md), [Architecture](docs/ARCHITECTURE.md), [Testing](docs/TESTING.md).

[Back to top](#new-bearings)

---

## 44. Git and Repository Workflow

Git provides a record of project changes and supports collaboration.

Repository operations should be performed deliberately.

### 44.1 Check the working tree

Before making changes, inspect the working tree.

```bash
git status
```

This command helps identify modified, staged, and untracked files.

Review the output before committing.

Do not assume the working tree is clean.

### 44.2 Inspect changes

Use:

```bash
git diff
```

This shows unstaged changes.

Use:

```bash
git diff --staged
```

This shows staged changes.

Review the relevant diff before committing.

Confirm that the changes match the intended task.

### 44.3 Stage deliberately

Stage only the files intended for the commit.

Avoid staging generated output.

Avoid staging secrets.

Avoid staging unrelated personal configuration.

Review the staged diff before committing.

### 44.4 Commit messages

Commit messages should explain the purpose of the change.

A message such as `Fix progress persistence error` is more useful than `Update files`.

A message should not claim a feature is complete if the change only introduces part of it.

Separate unrelated changes when doing so improves review.

### 44.5 Branches and remotes

Check the current branch before changing repository history.

Check the configured remote before pushing.

Understand whether a push will update the intended repository and branch.

Do not force-push without a clear reason.

A history rewrite can affect collaborators and remote references.

### 44.6 Remote history

Local and remote branches may differ.

A local commit may exist even when it is no longer present on the remote branch.

Do not assume that a remote branch rewind deleted the local commit.

Before rewriting history, identify the intended result.

Use safer approaches when possible.

### 44.7 Generated files

Build output should normally remain untracked.

The ignore rules should match the project's actual build process.

A file should not be ignored merely because it looks inconvenient.

A configuration file that contributors need may belong in version control.

A private configuration file may need to remain local.

### 44.8 Repository naming

The working project name is New Bearings.

The repository URL uses `New-Bearings`.

The Rust package name and executable name should be kept consistent with the intended naming decision.

Renaming package metadata should be handled deliberately.

`Cargo.lock` should be updated through Cargo rather than manually edited.

### 44.9 History safety

Before a destructive Git operation, verify the current branch and intended target.

Understand what the operation changes.

Preserve work that should remain local.

Do not delete local history merely to change the remote branch.

Do not assume that a successful push means every other branch or local commit has changed.

**Related documentation:** [Development](docs/DEVELOPMENT.md), [Contributing](CONTRIBUTING.md), [Release Plan](docs/RELEASE_PLAN.md).

[Back to top](#new-bearings)

---

## 45. Continuous Integration

Continuous integration helps verify that changes continue to build and pass relevant checks.

The repository contains a GitHub Actions workflow at `.github/workflows/ci.yml`.

The actual workflow configuration determines which operating systems, toolchains, and commands are tested.

### 45.1 Purpose

CI can catch compilation failures.

It can run tests.

It can verify formatting.

It can run selected linting checks.

It can help identify platform-specific issues when configured for multiple platforms.

It provides repeatable evidence about the checks that were executed.

### 45.2 Limits

CI does not prove that the application is free of defects.

CI does not prove that every user workflow works.

CI does not prove accessibility.

CI does not prove legal compliance.

CI does not automatically verify every possible platform.

The value of CI depends on its configuration and the checks it runs.

### 45.3 Network failures

A CI job may fail because of an external network problem.

A toolchain download may fail because of DNS or service availability.

A dependency registry may be temporarily unavailable.

A runner may experience a transient infrastructure problem.

Such failures should be distinguished from source-code failures.

Read the logs before changing application code.

Retry a transient infrastructure failure when appropriate.

Investigate repeated failures rather than assuming every failure is transient.

### 45.4 Workflow maintenance

The workflow should remain consistent with the project's actual development commands.

Changes to the Rust toolchain should be reflected where necessary.

New required checks should be added deliberately.

Platform-specific steps should be documented.

Failed checks should not be ignored merely to obtain a green status.

### 45.5 Reporting

When reporting CI results, identify the run and relevant job.

Distinguish a successful job from a failed job.

Distinguish an infrastructure failure from a code failure when the logs support that conclusion.

Do not claim that all platforms passed if only one platform was tested.

**Related documentation:** [Testing](docs/TESTING.md), [Development](docs/DEVELOPMENT.md), [Release Plan](docs/RELEASE_PLAN.md).

[Back to top](#new-bearings)

---

## 46. Community Standards

A productive project community depends on clear expectations and respectful communication.

Contributors may disagree about architecture, priorities, learning design, or implementation details.

Disagreement should remain focused on the work.

### 46.1 Respectful discussion

Critique ideas rather than attacking people.

Explain technical concerns clearly.

Ask questions when context is missing.

Avoid assuming bad intent from a disagreement.

Respect contributors with different experience levels.

### 46.2 Beginner-friendly collaboration

New contributors may not know every convention.

Documentation should help them get started.

Reviews should explain important issues.

Technical feedback should be specific.

The project should not expect every contributor to have the same background.

### 46.3 Constructive criticism

A proposal may have disadvantages.

Reviewers should explain those disadvantages.

A contributor should be able to ask for clarification.

A technical decision should be based on requirements and tradeoffs rather than personal attacks.

Strong disagreement can remain professional.

### 46.4 Accessibility in community participation

Communication should remain understandable.

Documentation should be available in accessible formats where practical.

Contributors should avoid relying on color alone when communicating essential status.

Issue templates should request useful information without demanding irrelevant personal details.

### 46.5 Conduct enforcement

The repository's code of conduct defines the applicable expectations and reporting process.

This README does not replace that document.

Reports should be handled according to the published process.

The project should avoid promising enforcement procedures that have not been established.

**Related documentation:** [Code of Conduct](CODE_OF_CONDUCT.md), [Contributing](CONTRIBUTING.md).

[Back to top](#new-bearings)

---

## 47. Reporting Bugs and Requesting Features

Bug reports and feature requests help identify problems and prioritize improvements.

Clear reports reduce unnecessary back-and-forth.

### 47.1 Bug reports

A useful bug report should include the expected behavior.

It should include the actual behavior.

It should include steps to reproduce the problem.

It should include the relevant operating system.

It should include relevant tool or application versions.

It should include error output where appropriate.

It should explain whether the problem occurs consistently.

It should avoid including secrets.

It should avoid unnecessary personal information.

### 47.2 Feature requests

A useful feature request should describe the problem it is intended to solve.

It should explain who would benefit.

It should describe the expected behavior.

It should identify important constraints.

It should explain relevant accessibility considerations where applicable.

It should distinguish a required capability from one possible implementation.

### 47.3 Scope and priorities

Not every request can be implemented immediately.

A feature may require foundational work.

A feature may introduce substantial maintenance costs.

A feature may conflict with an existing architectural decision.

A feature may create privacy or security risks.

The project should explain important tradeoffs.

### 47.4 Avoid duplicate reports

Search existing issues before opening a new report where practical.

If an existing issue describes the same problem, add relevant new information there.

Avoid opening duplicate reports merely to increase visibility.

### 47.5 Follow-up information

Maintainers may need additional information to reproduce an issue.

Provide relevant details when practical.

If the problem cannot be reproduced, explain what was tried.

If the issue is resolved, confirm the result when possible.

**Related documentation:** [Contributing](CONTRIBUTING.md), [Development](docs/DEVELOPMENT.md), [Security Policy](docs/SECURITY_POLICY.md).

[Back to top](#new-bearings)

---

## 48. Security Issue Reporting

Security issues should be handled with care.

A vulnerability may affect user data, application reliability, or the safety of future integrations.

### 48.1 What to report

Report vulnerabilities that affect the application or its supported workflows.

Examples may include unsafe file handling.

They may include unintended disclosure of sensitive information.

They may include insecure handling of credentials.

They may include a vulnerability in a supported integration.

They may include a security problem involving imported content.

These examples are illustrative and do not establish that such vulnerabilities currently exist.

### 48.2 Reporting channel

Use the reporting instructions in [`SECURITY.md`](SECURITY.md) or [`docs/SECURITY_POLICY.md`](docs/SECURITY_POLICY.md).

If a private reporting channel is provided, use it for issues that should not be disclosed publicly.

Do not assume that a public issue tracker is appropriate for every vulnerability.

### 48.3 Report details

Include a concise description.

Include the affected component.

Include reproduction steps where safe.

Include relevant version information.

Include the potential impact.

Avoid including unrelated personal information.

Do not publish secrets or real user data in a report.

### 48.4 Responsible handling

The project should assess the report.

It should determine the affected versions and scope where possible.

It should develop and verify a fix when necessary.

It should communicate relevant limitations.

It should not promise a particular response time unless one has been established.

### 48.5 Security disclosure

Disclosure should consider the risk to users.

A public report should not unnecessarily expose users to an unpatched vulnerability.

The appropriate timing depends on the issue and the available mitigation.

The project's security policy should provide the authoritative process.

**Related documentation:** [Security Policy](docs/SECURITY_POLICY.md), [Security Reporting](SECURITY.md), [Privacy and Data Handling](docs/PRIVACY_AND_DATA_HANDLING.md).

[Back to top](#new-bearings)

---

## 49. Frequently Asked Questions

### 49.1 What is New Bearings?

New Bearings is an early-stage Rust application intended to help people learn and retain knowledge through a flexible learning environment.

Its initial focus is programming education.

Its long-term scope may include additional subjects.

### 49.2 Is New Bearings the final name?

No.

New Bearings is a working project name.

The name may change before a stable public release.

### 49.3 Is the application finished?

No stable release is established by this README.

The project is in early development.

The actual state of each feature must be checked against the current code and verified behavior.

### 49.4 Does the application support every programming language listed?

Not necessarily.

The language list describes the intended direction.

A language should only be described as supported when the relevant functionality has been implemented and verified.

### 49.5 Does the application execute code?

This README does not establish that a complete code execution environment is available.

Code execution would require a separate implementation and security review.

Check the current source and release documentation for verified capabilities.

### 49.6 Is the application intended for beginners?

Yes, beginners are an important part of the intended audience.

The application should also aim to accommodate returning learners and people with more experience.

### 49.7 Does New Bearings diagnose learning conditions?

No.

New Bearings is not intended to diagnose medical, psychological, or learning conditions.

It should not infer a diagnosis from a user's preferences.

### 49.8 Do I need a diagnosis to use accessibility features?

No.

Accessibility and personalization should be available according to the implemented settings and workflows, without requiring a diagnosis.

### 49.9 Does every learner need to use personalization?

No.

Personalization is intended to be optional.

Reasonable defaults should allow users to begin without extensive configuration.

### 49.10 Does the application guarantee that I will learn a subject?

No.

Learning outcomes depend on many factors.

The application can provide learning tools and practice opportunities, but it should not guarantee mastery or professional competence.

### 49.11 Does completing a lesson prove mastery?

No.

Completion indicates that the lesson's completion criteria were met.

It does not automatically prove that the learner can independently apply the concept in every context.

### 49.12 Is the application cross-platform?

Cross-platform support is a long-term objective.

Windows is the initial development platform.

Other platforms should be considered supported only when the relevant builds and workflows have been verified.

### 49.13 Do I need Rust to use the application?

A developer building from source generally needs the Rust toolchain.

A future packaged release may not require end users to install Rust.

The requirements depend on the actual distribution method.

### 49.14 How do I build the application?

Start with the verified instructions in [`docs/DEVELOPMENT.md`](docs/DEVELOPMENT.md).

The standard Cargo commands in this README are useful starting points, but the current repository configuration remains authoritative.

### 49.15 How do I run the tests?

From the repository root, the standard Rust test command is:

```bash
cargo test
```

Review the actual output.

Do not assume tests passed if the command failed or was not run.

### 49.16 Where can I find installation instructions?

See [`docs/INSTALLATION.md`](docs/INSTALLATION.md).

That document should identify the supported installation methods and prerequisites.

### 49.17 Where can I find the architecture?

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md).

It should provide more detail than the high-level architecture overview in this README.

### 49.18 Where can I find content format information?

See [`docs/CONTENT_FORMAT.md`](docs/CONTENT_FORMAT.md).

Use the documented format rather than assuming that arbitrary files can be loaded as valid learning content.

### 49.19 Where can I find the roadmap?

See [`docs/ROADMAP.md`](docs/ROADMAP.md).

The roadmap describes planned priorities.

It should not be interpreted as a guarantee that every item is already implemented or has a fixed delivery date.

### 49.20 Where can I report a bug?

Use the repository's issue-reporting process where appropriate.

Include reproducible steps and relevant technical information.

Avoid including secrets or unnecessary personal data.

### 49.21 Where can I report a security issue?

Read [`SECURITY.md`](SECURITY.md) and [`docs/SECURITY_POLICY.md`](docs/SECURITY_POLICY.md).

Use the reporting channel specified there.

### 49.22 Can I contribute lessons?

Content contributions may be possible according to the current contribution process.

Lessons should be accurate, clearly structured, and consistent with the documented content format.

### 49.23 Can I add another subject?

The long-term direction includes additional subjects.

New subject support should use a suitable content model and should not assume that every subject uses the same teaching or assessment methods.

### 49.24 Is my learning data sent to a server?

Do not infer network behavior from the product vision alone.

Consult the current privacy documents and inspect the implementation of any network-dependent features.

The documentation should be updated whenever data-handling behavior changes.

### 49.25 Is the application completely private?

No absolute privacy claim should be made without evidence.

The project's privacy documentation should describe actual storage, network behavior, and relevant limitations.

### 49.26 Is the application completely secure?

No software should be described as completely secure.

The project should document its security practices, known limitations, and reporting process.

### 49.27 Is the project open source?

The repository is hosted publicly on GitHub, but the project's license files and metadata must be finalized and kept consistent before making definitive claims about its licensing status.

### 49.28 Can I use the code in my own project?

That depends on the finalized project license and the licenses of relevant third-party components.

Review the actual license files before redistributing or incorporating the code.

### 49.29 Why is Rust being used?

Rust provides strong typing, memory-safety features, and established build tooling.

These are useful properties for a maintainable application.

Rust does not eliminate the need for testing, careful design, or secure data handling.

### 49.30 Why is the project being developed incrementally?

Incremental development makes it easier to identify regressions.

It keeps changes reviewable.

It makes failures easier to investigate.

It helps the developer learn the system.

It reduces the risk of building a large collection of disconnected or unverified features.

### 49.31 Why are some features described as planned?

A planned feature represents the product direction, not a verified implementation.

Clearly labeling planned work helps prevent users and contributors from relying on behavior that does not yet exist.

### 49.32 How can I help?

Read the contribution guide.

Review existing issues.

Report reproducible bugs.

Improve documentation.

Review accessibility.

Contribute tests.

Help validate learning content.

Suggest focused improvements with a clear explanation of the problem they solve.

**Related documentation:** [Installation](docs/INSTALLATION.md), [Development](docs/DEVELOPMENT.md), [Roadmap](docs/ROADMAP.md), [Contributing](CONTRIBUTING.md).

[Back to top](#new-bearings)

---

## 50. Glossary

This glossary explains terms used throughout the project.

Definitions may be refined as the application develops.

### Accessibility

The practice of reducing barriers that prevent people from using software and its content.

### Active practice

A learning activity that requires the learner to produce a response, solve a problem, explain an idea, or otherwise engage with the material.

### Adaptive learning

An approach in which some aspects of the learning experience change according to relevant information, preferences, or demonstrated activity.

### Achievement

An optional recognition of a defined activity or milestone.

An achievement is not automatically proof of mastery.

### Application module

A section of the codebase with a defined responsibility.

### Assessment

An activity used to evaluate a learner's response or understanding.

An assessment's result should be interpreted according to what it actually measures.

### Accessibility setting

A configurable option intended to improve interaction for a user with a particular preference or need.

### Cargo

Rust's package manager and build tool.

### CI

Continuous integration.

CI runs automated checks on a configured environment and workflow.

### Content pack

A collection of learning content organized according to a supported format.

### Cross-platform

Designed to operate on more than one operating system, subject to implementation and verification.

### Dependency

A library, tool, or other component that the application relies on.

### Error handling

The process of detecting, representing, communicating, and recovering from failures.

### Exercise

An activity intended to give a learner an opportunity to apply or demonstrate a concept.

### Feature status

A description of whether a feature is implemented, partial, planned, blocked, or in another defined state.

### Integration test

A test that verifies behavior across multiple components or at a broader system boundary.

### Lesson

A structured unit of learning content with a defined purpose.

### Learning path

An organized sequence or collection of learning activities.

### Local storage

Data stored on the user's device rather than necessarily being stored by an external service.

Local storage does not by itself guarantee complete security.

### Mastery

A description of demonstrated competence in a defined context.

The meaning of mastery depends on the assessment and the skill being measured.

### Module boundary

The interface between separate areas of an application.

### Personalization

Changes to the learning experience based on user-selected preferences or relevant information.

### Prerequisite

A concept or activity that supports understanding of another concept or activity.

### Progress tracking

Recording and presenting selected information about learning activity.

Progress tracking is not a complete measurement of a person's knowledge.

### Retrieval practice

Practice that asks a learner to recall information rather than simply reread it.

### Review

An activity intended to revisit previously studied material.

### Rust

The primary implementation language used by New Bearings.

### Schema

A definition of the expected structure and constraints of data.

### Spaced review

Revisiting material across separate periods rather than only studying it in one continuous session.

### Test coverage

An indication of how much code or behavior is exercised by a defined set of tests.

Coverage does not by itself prove correctness.

### Unit test

A test focused on a relatively small unit of behavior.

### User preference

A setting or choice that reflects how a user wants the application to behave.

A preference is not a diagnosis.

### Validation

Checking whether data or an operation meets defined requirements.

### Working project name

A provisional name that may change before the project is finalized or released.

**Related documentation:** [Architecture](docs/ARCHITECTURE.md), [Content Format](docs/CONTENT_FORMAT.md), [Development](docs/DEVELOPMENT.md).

[Back to top](#new-bearings)

---

## 51. Project Limitations

New Bearings is an evolving project.

Its limitations should be documented honestly and updated as the implementation changes.

### 51.1 Implementation limitations

Some capabilities described in this README may be planned rather than implemented.

Some modules may provide only part of their intended functionality.

Some workflows may not yet be complete.

Some interfaces may change as the architecture develops.

The current source and verified behavior determine what is actually available.

### 51.2 Educational limitations

The application cannot guarantee learning outcomes.

A completed activity does not automatically prove broad competence.

A progress indicator cannot represent every aspect of knowledge.

A recommendation may be based on incomplete information.

Different learners may respond differently to the same activity.

Educational content may require correction or revision.

### 51.3 Accessibility limitations

Design intentions do not prove accessibility.

A feature may work differently across platforms.

A particular interface framework may impose limitations.

Manual testing may reveal problems that automated checks miss.

Accessibility should be evaluated continuously.

### 51.4 Platform limitations

Windows is the initial development platform.

Other platforms require their own verification.

Platform-specific integrations may not be available everywhere.

Packaging behavior may differ between operating systems.

Compatibility should be reported according to actual testing.

### 51.5 Privacy limitations

Privacy depends on actual application behavior and deployment choices.

Local storage does not guarantee that no other software can access a file.

External integrations may introduce separate data-handling considerations.

Documentation must be updated when the application begins using new services.

### 51.6 Security limitations

No application can guarantee the absence of all vulnerabilities.

Security reviews have a defined scope.

Automated tools cannot identify every possible issue.

Future integrations may introduce new risks.

The project should continue reviewing security as functionality expands.

### 51.7 Documentation limitations

This README provides a high-level project reference.

It does not replace detailed implementation documentation.

Some technical details may change.

Commands should be verified against the current repository.

File structures may evolve.

### 51.8 Keeping limitations current

When a limitation is resolved, update the relevant documentation.

When a new limitation is discovered, document it where appropriate.

Do not hide known defects merely because they complicate a release.

Do not leave outdated warnings in place after they are no longer accurate.

**Related documentation:** [Roadmap](docs/ROADMAP.md), [Testing](docs/TESTING.md), [Release Plan](docs/RELEASE_PLAN.md).

[Back to top](#new-bearings)

---

## 52. Release Readiness

A public release should be based on verification rather than a desired date alone.

The release process should consider implementation, testing, documentation, licensing, security, and distribution.

### 52.1 Functional readiness

The features included in the release should have defined behavior.

Important workflows should be tested.

Known defects should be assessed.

Incomplete features should be clearly identified.

The release should not advertise capabilities that are unavailable.

### 52.2 Build readiness

The intended release target should build successfully.

The relevant build command should be recorded.

The build output should be inspected.

The resulting artifact should be identified.

The release process should not rely on an unverified artifact left over from an earlier build.

### 52.3 Test readiness

Relevant tests should run.

Failures should be investigated.

Skipped tests should be recorded.

Platform-specific verification should be described accurately.

The project should not claim universal compatibility based on a narrow test run.

### 52.4 Documentation readiness

Installation instructions should match the release.

The README should match the actual capabilities.

Known limitations should be documented.

The changelog should identify relevant changes.

Privacy and security documentation should match the implementation.

Internal links should work.

### 52.5 Licensing readiness

The project license should be finalized.

The license file should contain the correct text.

Package metadata should be consistent.

Third-party licenses should be reviewed.

Required notices should be included.

Unresolved licensing concerns should be addressed before distribution.

### 52.6 Security readiness

Dependencies should be reviewed.

Relevant vulnerabilities should be assessed.

Secrets should not be included in the release.

Data-handling behavior should be understood.

The security-reporting process should be available.

Known limitations should be documented.

### 52.7 Artifact readiness

The release artifact should be the expected type.

Its name should be clear.

The version should be identifiable.

Checksums may be supplied where appropriate.

The artifact should be tested according to the release plan.

Installation and removal instructions should match the actual package.

### 52.8 Release communication

Release notes should describe meaningful changes.

Breaking changes should be identified.

Known issues should be stated.

Platform limitations should be clear.

The release should not claim that unimplemented features are available.

### 52.9 After release

User reports should be reviewed.

Important regressions should be prioritized.

Documentation should be corrected when errors are discovered.

Security reports should be handled according to the security policy.

Release lessons should inform the next development cycle.

**Related documentation:** [Release Plan](docs/RELEASE_PLAN.md), [Testing](docs/TESTING.md), [Changelog](CHANGELOG.md).

[Back to top](#new-bearings)

---

## 53. Long-Term Vision

The long-term vision is to make New Bearings a useful, adaptable learning environment that supports people as they develop knowledge over time.

The application should remain grounded in practical learning needs.

It should not expand simply to follow trends.

It should not promise that software can remove every obstacle to learning.

It should focus on providing tools that help learners study, practice, review, and understand their progress.

### 53.1 A broader learning companion

Programming is the starting point.

Additional subjects may be introduced when the architecture and content model can support them appropriately.

Different subjects should be able to use suitable teaching methods.

The application should avoid imposing programming-specific assumptions on unrelated content.

### 53.2 Better personalization

Personalization should become more useful without becoming intrusive.

User-selected preferences should remain important.

Adaptive recommendations should be explainable where practical.

The system should avoid unsupported claims about how a person learns.

Users should retain meaningful control.

### 53.3 Better retention support

The application may develop more useful review workflows.

It may help learners revisit important concepts.

It may provide varied practice opportunities.

It may help identify topics that deserve additional attention.

It should avoid treating one score as a complete description of knowledge.

### 53.4 Stronger accessibility

Accessibility should improve as the interface grows.

Keyboard support should remain a priority.

Readable content should remain a priority.

Settings should remain understandable.

Accessibility should be tested across actual workflows.

The project should respond constructively to accessibility feedback.

### 53.5 Sustainable architecture

The codebase should remain understandable as features are added.

Modules should have clear responsibilities.

Dependencies should remain deliberate.

Important decisions should be documented.

Testing should grow with the application.

Refactoring should preserve behavior unless a change is intentional.

### 53.6 Responsible growth

The application should not add complexity without a clear benefit.

Features should have a defined purpose.

Features should be verified before they are advertised.

Privacy and security should be reviewed when new data flows are introduced.

Cross-platform support should be earned through testing.

Release claims should remain grounded in evidence.

### 53.7 Community and contribution

A healthy contribution process can improve the application over time.

Documentation should help new contributors get started.

Content contributions should be reviewed for accuracy.

Technical changes should be tested.

Community feedback should inform priorities without making every request an automatic commitment.

### 53.8 The measure of success

The success of New Bearings should not be measured only by the number of features.

It should be measured by whether the application helps people accomplish meaningful learning tasks.

It should be measured by whether the interface is understandable.

It should be measured by whether users can control relevant preferences.

It should be measured by whether progress information is honest.

It should be measured by whether the application behaves reliably.

It should be measured by whether the project can be maintained and improved.

**Related documentation:** [Roadmap](docs/ROADMAP.md), [Architecture](docs/ARCHITECTURE.md), [Release Plan](docs/RELEASE_PLAN.md).

[Back to top](#new-bearings)

---

## 54. Final Project Principles

New Bearings should remain guided by a small set of enduring commitments.

### 54.1 The learner comes first

The application exists to support learning.

Its interface should help users understand what they can do.

Its content should explain ideas accurately.

Its feedback should help learners move forward.

Its progress information should remain honest.

Its optional features should serve a useful purpose.

### 54.2 Adaptability should be practical

Personalization should solve real problems.

It should not require unnecessary setup.

It should not rely on rigid learner stereotypes.

It should not make unsupported claims.

It should remain understandable and controllable.

### 54.3 Accessibility is ongoing work

Accessibility should be considered in every relevant feature.

It should be tested.

It should be documented.

It should improve as the application develops.

It should not be treated as a one-time checklist.

### 54.4 Privacy should be deliberate

Data collection should have a defined purpose.

Storage behavior should be documented.

Network behavior should be transparent.

External services should be identified where relevant.

Privacy claims should reflect actual implementation.

### 54.5 Reliability matters

Data should be handled carefully.

Errors should be useful.

Failures should not be hidden.

Important workflows should be tested.

Release artifacts should be verified.

Known limitations should be communicated.

### 54.6 Engineering should remain maintainable

Modules should have clear responsibilities.

Dependencies should be justified.

Changes should be reviewable.

Tests should protect important behavior.

Documentation should match the code.

The project should avoid unnecessary complexity.

### 54.7 The project name may change

New Bearings remains a working name.

The project's purpose and engineering commitments matter more than the current label.

If the name changes, repository references and package metadata should be updated deliberately.

Documentation should not retain inconsistent names after a rename.

### 54.8 Plans must remain distinct from implementation

A planned feature is not a completed feature.

A module is not proof that every intended behavior exists.

A successful build is not proof that every workflow works.

A passing test suite is not proof that every possible failure has been prevented.

A release should be described according to verified evidence.

### 54.9 Keep learning

The application is intended to help users learn.

The development process should reflect that same principle.

Design decisions should be examined.

Mistakes should be investigated.

Documentation should improve.

Tests should expand when useful.

The architecture should evolve deliberately.

The project should remain open to better approaches when evidence supports them.

---

### Project Links

- **Repository:** [Pandagamerteen08/New-Bearings](https://github.com/Pandagamerteen08/New-Bearings)
- **Architecture:** [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- **Content format:** [docs/CONTENT_FORMAT.md](docs/CONTENT_FORMAT.md)
- **Development:** [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md)
- **Installation:** [docs/INSTALLATION.md](docs/INSTALLATION.md)
- **Roadmap:** [docs/ROADMAP.md](docs/ROADMAP.md)
- **Testing:** [docs/TESTING.md](docs/TESTING.md)
- **Release plan:** [docs/RELEASE_PLAN.md](docs/RELEASE_PLAN.md)
- **Accessibility:** [ACCESSIBILITY.md](ACCESSIBILITY.md)
- **Privacy policy:** [PRIVACY_POLICY.md](PRIVACY_POLICY.md)
- **Contributing:** [CONTRIBUTING.md](CONTRIBUTING.md)
- **Code of conduct:** [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
- **Security reporting:** [SECURITY.md](SECURITY.md)
- **Security policy:** [docs/SECURITY_POLICY.md](docs/SECURITY_POLICY.md)

---

New Bearings is a work in progress.

Its purpose is to help people learn in ways that make sense for them, while remaining honest about what the application can do, careful with user data, accessible by design, and maintainable for the people who build it.

Learning should adapt to the individual, not force the individual to adapt to a single way of learning.

[Back to top](#new-bearings)
