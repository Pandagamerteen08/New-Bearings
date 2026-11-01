//! Learning content: the shape of a learning path and its lessons, the
//! validation that must pass before content is used, and the projections the
//! interface is allowed to see.
//!
//! Content is authored as JSON and compiled into the application, so nothing is
//! fetched at runtime. Answers live in [`Check`], which is deliberately not
//! serializable: the interface only ever receives a [`CheckView`], which has no
//! field that could reveal the answer or its explanation before an attempt.

use serde::{Deserialize, Serialize};

use crate::progress::LessonRecord;

/// The content format version this build understands.
pub const SUPPORTED_FORMAT_VERSION: u32 = 2;

/// How demanding a learning path is meant to be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

/// A single block of lesson material.
///
/// Sections are presentation-neutral: the interface chooses how to draw each
/// kind, and adding a kind here is a content format change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Section {
    /// Prose that explains an idea.
    Explanation { heading: String, body: String },
    /// A code sample with the language it is written in.
    Code {
        heading: Option<String>,
        language: String,
        code: String,
    },
    /// A short aside the learner should notice.
    Note { body: String },
    /// An ordered procedure, such as the steps of a process.
    Steps { heading: String, steps: Vec<String> },
    /// A small comparison table: columns name the values in each row.
    Table {
        heading: String,
        columns: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    /// A mistake learners commonly make, and what to do instead.
    Mistake { body: String },
}

/// The kinds of knowledge check a lesson can contain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CheckKind {
    /// Pick one of several offered options.
    Choice,
    /// Decide whether a statement is true or false.
    TrueFalse,
    /// Type an answer that is compared with the accepted wordings.
    ShortAnswer,
}

/// An answer as submitted by the interface.
///
/// The shape is tagged so a malformed or misdirected answer is rejected instead
/// of being silently compared against the wrong kind of check.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SubmittedAnswer {
    /// The index of the chosen option.
    Choice { option: usize },
    /// The chosen truth value.
    TrueFalse { value: bool },
    /// Text typed by the learner.
    ShortAnswer { text: String },
}

/// Why a submitted answer could not be graded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnswerProblem {
    /// The answer kind does not match the check kind.
    KindMismatch,
    /// The option index is not one of the offered options.
    OptionOutOfRange,
    /// Nothing was typed into a short-answer check.
    EmptyAnswer,
}

/// A knowledge check as authored, including how it is graded.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Check {
    /// Several options, exactly one of which is correct.
    Choice {
        id: String,
        prompt: String,
        options: Vec<String>,
        answer: usize,
        explanation: String,
    },
    /// A statement the learner marks as true or false.
    TrueFalse {
        id: String,
        prompt: String,
        answer: bool,
        explanation: String,
    },
    /// A typed answer, compared with one or more accepted wordings.
    ShortAnswer {
        id: String,
        prompt: String,
        accepted: Vec<String>,
        explanation: String,
    },
}

impl Check {
    /// The stable id used in progress records.
    pub fn id(&self) -> &str {
        match self {
            Self::Choice { id, .. } | Self::TrueFalse { id, .. } | Self::ShortAnswer { id, .. } => {
                id
            }
        }
    }

    /// The question as the learner reads it.
    pub fn prompt(&self) -> &str {
        match self {
            Self::Choice { prompt, .. }
            | Self::TrueFalse { prompt, .. }
            | Self::ShortAnswer { prompt, .. } => prompt,
        }
    }

    /// The wording shown once an answer has been judged.
    pub fn explanation(&self) -> &str {
        match self {
            Self::Choice { explanation, .. }
            | Self::TrueFalse { explanation, .. }
            | Self::ShortAnswer { explanation, .. } => explanation,
        }
    }

    /// Which kind of interaction this check needs.
    pub fn kind(&self) -> CheckKind {
        match self {
            Self::Choice { .. } => CheckKind::Choice,
            Self::TrueFalse { .. } => CheckKind::TrueFalse,
            Self::ShortAnswer { .. } => CheckKind::ShortAnswer,
        }
    }

    /// The offered options, or an empty slice when the check offers none.
    pub fn options(&self) -> &[String] {
        match self {
            Self::Choice { options, .. } => options,
            Self::TrueFalse { .. } | Self::ShortAnswer { .. } => &[],
        }
    }

    /// Judges a submitted answer.
    ///
    /// This is the only place an answer is compared, so the correct answer never
    /// has to leave the backend.
    pub fn grade(&self, submitted: &SubmittedAnswer) -> Result<bool, AnswerProblem> {
        match (self, submitted) {
            (
                Self::Choice {
                    options, answer, ..
                },
                SubmittedAnswer::Choice { option },
            ) => {
                if *option >= options.len() {
                    return Err(AnswerProblem::OptionOutOfRange);
                }

                Ok(option == answer)
            }
            (Self::TrueFalse { answer, .. }, SubmittedAnswer::TrueFalse { value }) => {
                Ok(value == answer)
            }
            (Self::ShortAnswer { accepted, .. }, SubmittedAnswer::ShortAnswer { text }) => {
                let submitted = normalize_answer(text);

                if submitted.is_empty() {
                    return Err(AnswerProblem::EmptyAnswer);
                }

                Ok(accepted
                    .iter()
                    .any(|candidate| normalize_answer(candidate) == submitted))
            }
            _ => Err(AnswerProblem::KindMismatch),
        }
    }

    /// The projection shown to the learner: the question and the options, with
    /// the learner's own state, but no answer and no explanation.
    pub fn view(&self, state: Option<&crate::progress::CheckRecord>) -> CheckView {
        CheckView {
            id: self.id().to_owned(),
            kind: self.kind(),
            prompt: self.prompt().to_owned(),
            options: self.options().to_vec(),
            solved: state.is_some_and(|state| state.correct),
            attempts: state.map_or(0, |state| state.attempts),
        }
    }

    /// The correct option index, for tests that need to answer a check.
    #[cfg(test)]
    pub fn answer_index(&self) -> Option<usize> {
        match self {
            Self::Choice { answer, .. } => Some(*answer),
            _ => None,
        }
    }

    /// The accepted wordings, for tests that need to answer a check.
    #[cfg(test)]
    pub fn accepted_answers(&self) -> &[String] {
        match self {
            Self::ShortAnswer { accepted, .. } => accepted,
            _ => &[],
        }
    }

    /// The correct truth value, for tests that need to answer a check.
    #[cfg(test)]
    pub fn correct_bool(&self) -> Option<bool> {
        match self {
            Self::TrueFalse { answer, .. } => Some(*answer),
            _ => None,
        }
    }
}

/// A knowledge check as presented to the learner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckView {
    pub id: String,
    pub kind: CheckKind,
    pub prompt: String,
    /// Empty for checks that do not offer options.
    pub options: Vec<String>,
    /// Whether the learner has already answered this check correctly.
    pub solved: bool,
    /// How many times the learner has submitted an answer for this check.
    pub attempts: u32,
}

/// Normalizes a typed answer before comparison.
///
/// Case, surrounding whitespace, repeated spaces and a trailing full stop are
/// ignored, so a correct answer is not rejected for its punctuation.
pub fn normalize_answer(value: &str) -> String {
    let lowered = value.trim().to_lowercase();
    let collapsed = lowered.split_whitespace().collect::<Vec<_>>().join(" ");

    collapsed.trim_end_matches('.').trim().to_owned()
}

/// A lesson as authored.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub minutes: Option<u32>,
    /// What the learner should be able to do afterwards.
    #[serde(default)]
    pub objectives: Vec<String>,
    pub sections: Vec<Section>,
    pub checks: Vec<Check>,
}

impl Lesson {
    /// The ids of every check the learner must answer to finish the lesson.
    pub fn check_ids(&self) -> Vec<String> {
        self.checks
            .iter()
            .map(|check| check.id().to_owned())
            .collect()
    }

    /// Finds a check by id.
    pub fn check(&self, check_id: &str) -> Option<&Check> {
        self.checks.iter().find(|check| check.id() == check_id)
    }

    /// The projection shown to the learner, including their own state.
    pub fn view(&self, record: Option<&LessonRecord>) -> LessonView {
        LessonView {
            id: self.id.clone(),
            title: self.title.clone(),
            summary: self.summary.clone(),
            minutes: self.minutes,
            objectives: self.objectives.clone(),
            sections: self.sections.clone(),
            checks: self
                .checks
                .iter()
                .map(|check| check.view(record.and_then(|record| record.check(check.id()))))
                .collect(),
        }
    }
}

/// A lesson as presented to the learner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonView {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub minutes: Option<u32>,
    pub objectives: Vec<String>,
    pub sections: Vec<Section>,
    pub checks: Vec<CheckView>,
}

/// A learning path as authored.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub subject: String,
    pub difficulty: Difficulty,
    #[serde(default)]
    pub tags: Vec<String>,
    pub lessons: Vec<Lesson>,
}

impl Path {
    /// Finds a lesson by id.
    pub fn lesson(&self, lesson_id: &str) -> Option<&Lesson> {
        self.lessons.iter().find(|lesson| lesson.id == lesson_id)
    }

    /// The total reading time of every lesson that declares one.
    pub fn total_minutes(&self) -> u32 {
        self.lessons
            .iter()
            .filter_map(|lesson| lesson.minutes)
            .sum()
    }
}

/// A problem found while loading content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentProblem {
    location: String,
    message: String,
}

impl ContentProblem {
    pub(crate) fn new(location: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            location: location.into(),
            message: message.into(),
        }
    }

    /// Where the problem was found, such as `paths[0].lessons[1].checks[0]`.
    pub fn location(&self) -> &str {
        &self.location
    }

    /// What is wrong, stated plainly.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Adds outer context to the location when several packs are loaded.
    pub fn prefix(&mut self, prefix: &str) {
        self.location = format!("{prefix}.{}", self.location);
    }
}

/// The on-disk shape of a content pack file.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PackFile {
    format_version: u32,
    paths: Vec<Path>,
}

/// Parses and validates a content pack.
///
/// Every problem found is reported rather than only the first, so a content
/// author can fix the whole file in one pass.
pub fn parse_pack(json: &str) -> Result<Vec<Path>, Vec<ContentProblem>> {
    let pack: PackFile = match serde_json::from_str(json) {
        Ok(pack) => pack,
        Err(error) => {
            return Err(vec![ContentProblem::new(
                "content",
                format!("the content file is not valid JSON for the expected shape: {error}"),
            )]);
        }
    };

    let mut problems = Vec::new();

    if pack.format_version != SUPPORTED_FORMAT_VERSION {
        problems.push(ContentProblem::new(
            "formatVersion",
            format!(
                "expected format version {SUPPORTED_FORMAT_VERSION} but found {}",
                pack.format_version
            ),
        ));
    }

    if pack.paths.is_empty() {
        problems.push(ContentProblem::new("paths", "no learning paths were found"));
    }

    let mut path_ids: Vec<&str> = Vec::new();

    for (path_index, path) in pack.paths.iter().enumerate() {
        let at = format!("paths[{path_index}]");
        validate_path(path, &at, &mut path_ids, &mut problems);
    }

    if problems.is_empty() {
        Ok(pack.paths)
    } else {
        Err(problems)
    }
}

fn validate_path<'a>(
    path: &'a Path,
    at: &str,
    path_ids: &mut Vec<&'a str>,
    problems: &mut Vec<ContentProblem>,
) {
    validate_slug(&path.id, &format!("{at}.id"), "learning path", problems);

    if path_ids.contains(&path.id.as_str()) {
        problems.push(ContentProblem::new(
            format!("{at}.id"),
            format!("the learning path id `{}` is used more than once", path.id),
        ));
    } else {
        path_ids.push(&path.id);
    }

    require_text(&path.title, &format!("{at}.title"), "a title", problems);
    require_text(
        &path.summary,
        &format!("{at}.summary"),
        "a summary",
        problems,
    );
    require_text(
        &path.subject,
        &format!("{at}.subject"),
        "a subject",
        problems,
    );

    for (tag_index, tag) in path.tags.iter().enumerate() {
        require_text(
            tag,
            &format!("{at}.tags[{tag_index}]"),
            "tag text",
            problems,
        );
    }

    if path.lessons.is_empty() {
        problems.push(ContentProblem::new(
            format!("{at}.lessons"),
            "a learning path needs at least one lesson",
        ));
    }

    let mut lesson_ids: Vec<&str> = Vec::new();

    for (lesson_index, lesson) in path.lessons.iter().enumerate() {
        validate_lesson(
            lesson,
            &format!("{at}.lessons[{lesson_index}]"),
            &mut lesson_ids,
            problems,
        );
    }
}

fn validate_lesson<'a>(
    lesson: &'a Lesson,
    at: &str,
    lesson_ids: &mut Vec<&'a str>,
    problems: &mut Vec<ContentProblem>,
) {
    validate_slug(&lesson.id, &format!("{at}.id"), "lesson", problems);

    if lesson_ids.contains(&lesson.id.as_str()) {
        problems.push(ContentProblem::new(
            format!("{at}.id"),
            format!(
                "the lesson id `{}` is used more than once in this path",
                lesson.id
            ),
        ));
    } else {
        lesson_ids.push(&lesson.id);
    }

    require_text(&lesson.title, &format!("{at}.title"), "a title", problems);
    require_text(
        &lesson.summary,
        &format!("{at}.summary"),
        "a summary",
        problems,
    );

    if lesson.objectives.is_empty() {
        problems.push(ContentProblem::new(
            format!("{at}.objectives"),
            "a lesson needs at least one learning objective",
        ));
    }

    for (objective_index, objective) in lesson.objectives.iter().enumerate() {
        require_text(
            objective,
            &format!("{at}.objectives[{objective_index}]"),
            "objective text",
            problems,
        );
    }

    if lesson.sections.is_empty() {
        problems.push(ContentProblem::new(
            format!("{at}.sections"),
            "a lesson needs at least one section",
        ));
    }

    for (section_index, section) in lesson.sections.iter().enumerate() {
        validate_section(
            section,
            &format!("{at}.sections[{section_index}]"),
            problems,
        );
    }

    if lesson.checks.is_empty() {
        problems.push(ContentProblem::new(
            format!("{at}.checks"),
            "a lesson needs at least one knowledge check so completion can be decided",
        ));
    }

    let mut check_ids: Vec<&str> = Vec::new();

    for (check_index, check) in lesson.checks.iter().enumerate() {
        let check_at = format!("{at}.checks[{check_index}]");

        validate_slug(
            check.id(),
            &format!("{check_at}.id"),
            "knowledge check",
            problems,
        );

        if check_ids.contains(&check.id()) {
            problems.push(ContentProblem::new(
                format!("{check_at}.id"),
                format!(
                    "the check id `{}` is used more than once in this lesson",
                    check.id()
                ),
            ));
        } else {
            check_ids.push(check.id());
        }

        require_text(
            check.prompt(),
            &format!("{check_at}.prompt"),
            "a prompt",
            problems,
        );
        require_text(
            check.explanation(),
            &format!("{check_at}.explanation"),
            "an explanation",
            problems,
        );

        validate_check_shape(check, &check_at, problems);
    }
}

fn validate_section(section: &Section, at: &str, problems: &mut Vec<ContentProblem>) {
    match section {
        Section::Explanation { heading, body } => {
            require_text(heading, &format!("{at}.heading"), "a heading", problems);
            require_text(body, &format!("{at}.body"), "a body", problems);
        }
        Section::Code {
            heading,
            language,
            code,
        } => {
            if let Some(heading) = heading {
                require_text(heading, &format!("{at}.heading"), "a heading", problems);
            }
            require_text(language, &format!("{at}.language"), "a language", problems);
            require_text(code, &format!("{at}.code"), "code", problems);
        }
        Section::Note { body } | Section::Mistake { body } => {
            require_text(body, &format!("{at}.body"), "a body", problems);
        }
        Section::Steps { heading, steps } => {
            require_text(heading, &format!("{at}.heading"), "a heading", problems);

            if steps.is_empty() {
                problems.push(ContentProblem::new(
                    format!("{at}.steps"),
                    "a steps section needs at least one step",
                ));
            }

            for (step_index, step) in steps.iter().enumerate() {
                require_text(
                    step,
                    &format!("{at}.steps[{step_index}]"),
                    "step text",
                    problems,
                );
            }
        }
        Section::Table {
            heading,
            columns,
            rows,
        } => {
            require_text(heading, &format!("{at}.heading"), "a heading", problems);

            if columns.len() < 2 {
                problems.push(ContentProblem::new(
                    format!("{at}.columns"),
                    "a table needs at least two columns",
                ));
            }

            for (column_index, column) in columns.iter().enumerate() {
                require_text(
                    column,
                    &format!("{at}.columns[{column_index}]"),
                    "column text",
                    problems,
                );
            }

            if rows.is_empty() {
                problems.push(ContentProblem::new(
                    format!("{at}.rows"),
                    "a table needs at least one row",
                ));
            }

            for (row_index, row) in rows.iter().enumerate() {
                if row.len() != columns.len() {
                    problems.push(ContentProblem::new(
                        format!("{at}.rows[{row_index}]"),
                        format!(
                            "the row has {} values but the table declares {} columns",
                            row.len(),
                            columns.len()
                        ),
                    ));
                }

                for (cell_index, cell) in row.iter().enumerate() {
                    require_text(
                        cell,
                        &format!("{at}.rows[{row_index}][{cell_index}]"),
                        "cell text",
                        problems,
                    );
                }
            }
        }
    }
}

fn validate_check_shape(check: &Check, at: &str, problems: &mut Vec<ContentProblem>) {
    match check {
        Check::Choice {
            options, answer, ..
        } => {
            if options.len() < 2 {
                problems.push(ContentProblem::new(
                    format!("{at}.options"),
                    "a choice check needs at least two options",
                ));
            }

            for (option_index, option) in options.iter().enumerate() {
                require_text(
                    option,
                    &format!("{at}.options[{option_index}]"),
                    "option text",
                    problems,
                );
            }

            if *answer >= options.len() {
                problems.push(ContentProblem::new(
                    format!("{at}.answer"),
                    format!(
                        "the answer index {} is outside the {} available options",
                        answer,
                        options.len()
                    ),
                ));
            }
        }
        Check::TrueFalse { .. } => {}
        Check::ShortAnswer { accepted, .. } => {
            if accepted.is_empty() {
                problems.push(ContentProblem::new(
                    format!("{at}.accepted"),
                    "a short-answer check needs at least one accepted wording",
                ));
            }

            for (accepted_index, wording) in accepted.iter().enumerate() {
                require_text(
                    wording,
                    &format!("{at}.accepted[{accepted_index}]"),
                    "accepted wording",
                    problems,
                );
            }
        }
    }
}

fn require_text(value: &str, at: &str, what: &str, problems: &mut Vec<ContentProblem>) {
    if value.trim().is_empty() {
        problems.push(ContentProblem::new(at, format!("{what} is required")));
    }
}

/// Ids end up in progress records and command arguments, so they must be
/// predictable: ASCII lowercase letters, digits and single hyphens.
fn validate_slug(value: &str, at: &str, what: &str, problems: &mut Vec<ContentProblem>) {
    let valid = !value.is_empty()
        && value.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--");

    if !valid {
        problems.push(ContentProblem::new(
            at,
            format!(
                "{what} id `{value}` must use lowercase letters, digits and single hyphens, and must not start or end with a hyphen"
            ),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::progress::{LessonKey, ProgressState};

    const EMBEDDED: &str = include_str!("../../content/programming/rust/foundations.json");

    const DEFAULT_SECTIONS: &str = r#"[{ "kind": "note", "body": "Read this." }]"#;

    const DEFAULT_CHECKS: &str = r#"[{
        "kind": "choice",
        "id": "q1",
        "prompt": "Pick one.",
        "options": ["a", "b"],
        "answer": 0,
        "explanation": "Because."
    }]"#;

    fn pack_with(sections: &str, checks: &str) -> String {
        format!(
            r#"{{
                "formatVersion": 2,
                "paths": [{{
                    "id": "sample",
                    "title": "Sample",
                    "summary": "A sample path.",
                    "subject": "Programming",
                    "difficulty": "beginner",
                    "lessons": [{{
                        "id": "one",
                        "title": "One",
                        "summary": "A sample lesson.",
                        "objectives": ["Understand the sample."],
                        "sections": {sections},
                        "checks": {checks}
                    }}]
                }}]
            }}"#
        )
    }

    fn valid_pack() -> String {
        pack_with(DEFAULT_SECTIONS, DEFAULT_CHECKS)
    }

    #[test]
    fn embedded_content_is_valid() {
        let paths = parse_pack(EMBEDDED).expect("the bundled content must be valid");

        assert!(!paths.is_empty());
    }

    #[test]
    fn embedded_lessons_teach_with_objectives_and_unique_checks() {
        for path in parse_pack(EMBEDDED).expect("valid content") {
            for lesson in path.lessons {
                assert!(
                    !lesson.objectives.is_empty(),
                    "{} has no objectives",
                    lesson.id
                );

                let mut ids = lesson.check_ids();
                let total = ids.len();
                ids.sort();
                ids.dedup();

                assert_eq!(ids.len(), total, "duplicate check ids in {}", lesson.id);
            }
        }
    }

    #[test]
    fn a_valid_pack_parses() {
        let paths = parse_pack(&valid_pack()).expect("the pack is valid");

        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].lessons[0].objectives.len(), 1);
    }

    #[test]
    fn lesson_views_never_include_answers_or_explanations() {
        let paths = parse_pack(EMBEDDED).expect("valid content");
        let view = paths[0].lessons[0].view(None);
        let json = serde_json::to_string(&view).expect("view should serialize");

        // Section kinds include "explanation", so look for keys, not words.
        for key in ["\"answer\":", "\"explanation\":", "\"accepted\":"] {
            assert!(!json.contains(key), "{key} leaked into {json}");
        }
    }

    #[test]
    fn lesson_views_carry_the_learners_own_state() {
        let paths = parse_pack(EMBEDDED).expect("valid content");
        let lesson = &paths[0].lessons[0];
        let key = LessonKey::new(&paths[0].id, &lesson.id);
        let mut progress = ProgressState::new();

        progress.record_attempt(&key, lesson.checks[0].id(), true, &lesson.check_ids(), 10);

        let view = lesson.view(progress.lesson(&key));

        assert!(view.checks[0].solved);
        assert_eq!(view.checks[0].attempts, 1);
        assert!(!view.checks[1].solved);
    }

    #[test]
    fn a_lesson_without_objectives_is_rejected() {
        let json = r#"{
            "formatVersion": 2,
            "paths": [{
                "id": "sample",
                "title": "Sample",
                "summary": "A sample path.",
                "subject": "Programming",
                "difficulty": "beginner",
                "lessons": [{
                    "id": "one",
                    "title": "One",
                    "summary": "A sample lesson.",
                    "sections": [{ "kind": "note", "body": "Read this." }],
                    "checks": [{
                        "kind": "trueFalse",
                        "id": "q1",
                        "prompt": "True?",
                        "answer": true,
                        "explanation": "Because."
                    }]
                }]
            }]
        }"#;

        let problems = parse_pack(json).expect_err("objectives are required");

        assert!(
            problems
                .iter()
                .any(|problem| problem.location() == "paths[0].lessons[0].objectives"),
            "got {problems:?}"
        );
    }

    #[test]
    fn a_table_row_must_match_the_declared_columns() {
        let sections = r#"[{
            "kind": "table",
            "heading": "Comparison",
            "columns": ["Name", "Meaning"],
            "rows": [["one"], ["two", "second"]]
        }]"#;

        let problems =
            parse_pack(&pack_with(sections, DEFAULT_CHECKS)).expect_err("the first row is short");

        assert!(
            problems
                .iter()
                .any(|problem| problem.location() == "paths[0].lessons[0].sections[0].rows[0]"),
            "got {problems:?}"
        );
    }

    #[test]
    fn a_steps_section_needs_a_step() {
        let sections = r#"[{ "kind": "steps", "heading": "How", "steps": [] }]"#;

        let problems = parse_pack(&pack_with(sections, DEFAULT_CHECKS))
            .expect_err("an empty list of steps teaches nothing");

        assert!(
            problems
                .iter()
                .any(|problem| problem.location() == "paths[0].lessons[0].sections[0].steps"),
            "got {problems:?}"
        );
    }

    #[test]
    fn a_short_answer_check_needs_an_accepted_wording() {
        let checks = r#"[{
            "kind": "shortAnswer",
            "id": "q1",
            "prompt": "Type it.",
            "accepted": [],
            "explanation": "Because."
        }]"#;

        let problems = parse_pack(&pack_with(DEFAULT_SECTIONS, checks))
            .expect_err("nothing could ever be correct");

        assert!(
            problems
                .iter()
                .any(|problem| problem.location() == "paths[0].lessons[0].checks[0].accepted"),
            "got {problems:?}"
        );
    }

    #[test]
    fn a_choice_check_needs_two_options() {
        let checks = r#"[{
            "kind": "choice",
            "id": "q1",
            "prompt": "Pick one.",
            "options": ["only"],
            "answer": 0,
            "explanation": "Because."
        }]"#;

        let problems = parse_pack(&pack_with(DEFAULT_SECTIONS, checks))
            .expect_err("a single option is not a choice");

        assert!(
            problems
                .iter()
                .any(|problem| problem.location() == "paths[0].lessons[0].checks[0].options"),
            "got {problems:?}"
        );
    }

    #[test]
    fn invalid_answer_index_is_reported() {
        let checks = r#"[{
            "kind": "choice",
            "id": "q1",
            "prompt": "Pick one.",
            "options": ["a", "b"],
            "answer": 5,
            "explanation": "Because."
        }]"#;

        let problems = parse_pack(&pack_with(DEFAULT_SECTIONS, checks))
            .expect_err("the answer index is out of range");

        assert!(
            problems
                .iter()
                .any(|problem| problem.location() == "paths[0].lessons[0].checks[0].answer"),
            "got {problems:?}"
        );
    }

    #[test]
    fn lesson_without_checks_is_rejected() {
        let problems =
            parse_pack(&pack_with(DEFAULT_SECTIONS, "[]")).expect_err("a lesson needs a check");

        assert!(
            problems
                .iter()
                .any(|problem| problem.location() == "paths[0].lessons[0].checks"),
            "got {problems:?}"
        );
    }

    #[test]
    fn unsupported_format_version_is_rejected() {
        let json = r#"{ "formatVersion": 1, "paths": [] }"#;

        let problems = parse_pack(json).expect_err("the version is not supported");

        assert!(
            problems
                .iter()
                .any(|problem| problem.location() == "formatVersion"),
            "got {problems:?}"
        );
    }

    #[test]
    fn duplicate_path_ids_are_rejected() {
        let json = r#"{
            "formatVersion": 2,
            "paths": [
                {
                    "id": "same",
                    "title": "First",
                    "summary": "First path.",
                    "subject": "Programming",
                    "difficulty": "beginner",
                    "lessons": [{
                        "id": "one",
                        "title": "One",
                        "summary": "A sample lesson.",
                        "objectives": ["Something."],
                        "sections": [{ "kind": "note", "body": "Read this." }],
                        "checks": [{
                            "kind": "trueFalse",
                            "id": "q1",
                            "prompt": "True?",
                            "answer": true,
                            "explanation": "Because."
                        }]
                    }]
                },
                {
                    "id": "same",
                    "title": "Second",
                    "summary": "Second path.",
                    "subject": "Programming",
                    "difficulty": "beginner",
                    "lessons": [{
                        "id": "one",
                        "title": "One",
                        "summary": "A sample lesson.",
                        "objectives": ["Something."],
                        "sections": [{ "kind": "note", "body": "Read this." }],
                        "checks": [{
                            "kind": "trueFalse",
                            "id": "q1",
                            "prompt": "True?",
                            "answer": true,
                            "explanation": "Because."
                        }]
                    }]
                }
            ]
        }"#;

        let problems = parse_pack(json).expect_err("path ids must be unique");

        assert!(
            problems
                .iter()
                .any(|problem| problem.message().contains("used more than once")),
            "got {problems:?}"
        );
    }

    #[test]
    fn malformed_json_reports_where_parsing_stopped() {
        let problems = parse_pack("{ not json").expect_err("this is not JSON");

        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].location(), "content");
    }

    #[test]
    fn total_minutes_sums_declared_lesson_times() {
        let paths = parse_pack(EMBEDDED).expect("valid content");

        assert!(paths[0].total_minutes() > 0);
    }

    #[test]
    fn choice_checks_grade_only_offered_options() {
        let check = Check::Choice {
            id: String::from("q1"),
            prompt: String::from("Pick one."),
            options: vec![String::from("a"), String::from("b")],
            answer: 1,
            explanation: String::from("Because."),
        };

        assert_eq!(
            check.grade(&SubmittedAnswer::Choice { option: 1 }),
            Ok(true)
        );
        assert_eq!(
            check.grade(&SubmittedAnswer::Choice { option: 0 }),
            Ok(false)
        );
        assert_eq!(
            check.grade(&SubmittedAnswer::Choice { option: 9 }),
            Err(AnswerProblem::OptionOutOfRange)
        );
        assert_eq!(
            check.grade(&SubmittedAnswer::TrueFalse { value: true }),
            Err(AnswerProblem::KindMismatch)
        );
    }

    #[test]
    fn true_false_checks_grade_both_values() {
        let check = Check::TrueFalse {
            id: String::from("q1"),
            prompt: String::from("True?"),
            answer: false,
            explanation: String::from("Because."),
        };

        assert_eq!(
            check.grade(&SubmittedAnswer::TrueFalse { value: false }),
            Ok(true)
        );
        assert_eq!(
            check.grade(&SubmittedAnswer::TrueFalse { value: true }),
            Ok(false)
        );
    }

    #[test]
    fn short_answers_ignore_case_spacing_and_a_trailing_stop() {
        let check = Check::ShortAnswer {
            id: String::from("q1"),
            prompt: String::from("Type it."),
            accepted: vec![String::from("ownership"), String::from("the owner")],
            explanation: String::from("Because."),
        };

        for text in ["ownership", "  Ownership ", "OWNERSHIP.", "The   Owner"] {
            assert_eq!(
                check.grade(&SubmittedAnswer::ShortAnswer {
                    text: String::from(text)
                }),
                Ok(true),
                "`{text}` should be accepted"
            );
        }

        assert_eq!(
            check.grade(&SubmittedAnswer::ShortAnswer {
                text: String::from("borrowing")
            }),
            Ok(false)
        );
    }

    #[test]
    fn an_empty_short_answer_is_rejected_rather_than_counted_as_wrong() {
        let check = Check::ShortAnswer {
            id: String::from("q1"),
            prompt: String::from("Type it."),
            accepted: vec![String::from("ownership")],
            explanation: String::from("Because."),
        };

        assert_eq!(
            check.grade(&SubmittedAnswer::ShortAnswer {
                text: String::from("   ")
            }),
            Err(AnswerProblem::EmptyAnswer)
        );
    }
}
