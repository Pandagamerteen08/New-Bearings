# Accessibility

## Purpose

New Bearings aims to make learning more approachable for people with different
needs, preferences, abilities, and levels of experience.

Accessibility is a continuing design responsibility, not a feature that can be
considered complete after a single implementation pass.

The project is in early development. The principles in this document describe
the intended direction and do not certify that the application currently meets
any particular accessibility standard.

## Core principle

Learning should adapt to the individual, rather than forcing every individual
to follow one rigid learning method.

Accessibility decisions should support meaningful user choice. Users should be
able to select the options that help them without being required to disclose a
diagnosis or explain a personal circumstance.

## Intended accessibility considerations

### Readability

The application should work toward:

- Clear headings and predictable page structure.
- Readable text with appropriate spacing.
- Understandable instructions.
- Consistent names for recurring controls.
- Clear error messages that explain how to recover.
- Avoidance of unnecessary technical jargon in learner-facing content.
- Options for adjusting text presentation where supported.

### Visual presentation

The application should consider:

- Adequate contrast between text and its background.
- Clear visual distinctions between interactive and noninteractive elements.
- Status indicators that do not rely on color alone.
- Reduced dependence on animation.
- Sensible handling of window resizing and display scaling.
- Consistent spacing and visual hierarchy.
- Support for light and dark presentation where implemented.

### Keyboard access

Interactive functionality should be designed with keyboard operation in mind.

Where the interface framework permits it, users should be able to navigate
controls, identify the current focus, activate actions, and leave interactive
areas without being forced to use a mouse.

Keyboard shortcuts should not create conflicts with assistive technology or
common operating-system behavior.

### Screen readers and assistive technology

Interface controls should expose meaningful names, roles, states, and values
through the capabilities of the selected interface framework.

Decorative information should not obscure meaningful information. Instructions
should not depend solely on visual positioning or color.

Actual compatibility must be evaluated with supported assistive technologies
rather than assumed from the use of a particular framework.

### Cognitive accessibility

Learning screens should avoid unnecessary complexity.

Potential design approaches include:

- Breaking complex tasks into manageable steps.
- Explaining unfamiliar terminology.
- Providing examples before expecting independent application.
- Allowing learners to review earlier explanations.
- Making progress and next steps understandable.
- Avoiding unnecessary time pressure.
- Providing predictable navigation.
- Allowing users to pause and resume where supported.

These approaches are design goals, not guarantees that every learner will
prefer or benefit from the same presentation.

### Learning preferences

Users may prefer examples, written explanations, guided exercises, independent
practice, repetition, or other approaches.

The application should support useful choices without assigning permanent
learning identities to people or treating a preference as a diagnosis.

### Motion and sensory considerations

The interface should avoid unnecessary flashing, abrupt transitions, and
excessive animation.

Where motion settings are provided, their behavior should be documented and
tested. Sound should not be the only way to communicate important information.

## Personalization principles

Personalization should be:

1. Optional where practical.
2. Understandable to the person using it.
3. Reversible.
4. Adjustable over time.
5. Respectful of privacy.
6. Based on useful feedback rather than unsupported assumptions.

A user should not have to complete a lengthy assessment before using basic
learning functionality.

The application should not assume that people who share a diagnosis, age,
background, or stated preference all learn in the same way.

## Accessibility testing

Accessibility testing should grow alongside the interface.

Testing may include:

- Keyboard-only navigation.
- Focus visibility and focus order.
- Text scaling and display scaling.
- Contrast checks.
- Screen-reader checks on supported platforms.
- Resizing and layout behavior.
- Clear identification of errors and success states.
- Review of instructions for clarity.
- Testing of reduced-motion options where available.

Automated checks can identify some problems but cannot establish that an
application is accessible to every user.

## Reporting accessibility problems

When reporting an accessibility issue, include the affected screen or workflow,
the operating system, the input method, the expected behavior, and the observed
behavior when those details are available.

Do not include sensitive personal information unless it is genuinely necessary.
Reports should focus on the obstacle and its practical impact.

Use the project's available issue-reporting process to submit non-sensitive
accessibility feedback. For privately reportable security concerns, follow
SECURITY.md.

## Scope and limitations

This document records project intentions. It is not a formal accessibility
conformance report, a guarantee of compatibility, or a substitute for testing.

Standards and platform behavior should be evaluated against the actual interface
and release under consideration.

## Related documentation

- [README](README.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Development](docs/DEVELOPMENT.md)
- [Testing](docs/TESTING.md)
- [Privacy and Data Handling](docs/PRIVACY_AND_DATA_HANDLING.md)

## Maintenance

Update this document when accessibility behavior, supported platforms, testing
coverage, or user-facing customization changes materially.
