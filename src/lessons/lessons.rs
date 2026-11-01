//! Learning content: the shape of a learning path and its lessons, the
//! validation that must pass before content is used, and the projections the
//! interface is allowed to see.
//!
//! Answers to knowledge checks live here and are deliberately absent from the
//! view types below, so they cannot reach the interface by accident.

use serde::{Deserialize, Serialize};

/// The content format version this build understands.
pub const SUPPORTED_FORMAT_VERSION: u32 = 1;

/// How demanding a learning path is meant to be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

/// A single block of lesson material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
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
}

/// A knowledge check as authored, including its answer.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Check {
    pub id: String,
    pub prompt: String,
    pub options: Vec<String>,
    answer: usize,
    pub explanation: String,
}

impl Check {
    /// The index of the correct option.
    pub fn answer(&self) -> usize {
        self.answer
    }

    /// Whether a submitted option index is the correct one.
    pub fn is_correct(&self, submitted: usize) -> bool {
        submitted == self.answer()
    }

    /// The projection shown to the learner: no answer, no explanation.
    pub fn view(&self) -> CheckView {
        CheckView {
            id: self.id.clone(),
            prompt: self.prompt.clone(),
            options: self.options.clone(),
        }
    }
}

/// A knowledge check as presented to the learner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckView {
    pub id: String,
    pub prompt: String,
    pub options: Vec<String>,
}

/// A lesson as authored.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub minutes: Option<u32>,
    pub sections: Vec<Section>,
    pub checks: Vec<Check>,
}

impl Lesson {
    /// The ids of every check the learner must answer to finish the lesson.
    pub fn check_ids(&self) -> Vec<String> {
        self.checks.iter().map(|check| check.id.clone()).collect()
    }

    /// Finds a check by id.
    pub fn check(&self, check_id: &str) -> Option<&Check> {
        self.checks.iter().find(|check| check.id == check_id)
    }

    /// The projection shown to the learner: material and questions, no answers.
    pub fn view(&self) -> LessonView {
        LessonView {
            id: self.id.clone(),
            title: self.title.clone(),
            summary: self.summary.clone(),
            minutes: self.minutes,
            sections: self.sections.clone(),
            checks: self.checks.iter().map(Check::view).collect(),
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
    fn new(location: impl Into<String>, message: impl Into<String>) -> Self {
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

    if lesson.sections.is_empty() {
        problems.push(ContentProblem::new(
            format!("{at}.sections"),
            "a lesson needs at least one section",
        ));
    }

    for (section_index, section) in lesson.sections.iter().enumerate() {
        let section_at = format!("{at}.sections[{section_index}]");

        match section {
            Section::Explanation { heading, body } => {
                require_text(
                    heading,
                    &format!("{section_at}.heading"),
                    "a heading",
                    problems,
                );
                require_text(body, &format!("{section_at}.body"), "a body", problems);
            }
            Section::Code {
                heading,
                language,
                code,
            } => {
                if let Some(heading) = heading {
                    require_text(
                        heading,
                        &format!("{section_at}.heading"),
                        "a heading",
                        problems,
                    );
                }
                require_text(
                    language,
                    &format!("{section_at}.language"),
                    "a language",
                    problems,
                );
                require_text(code, &format!("{section_at}.code"), "code", problems);
            }
            Section::Note { body } => {
                require_text(body, &format!("{section_at}.body"), "a body", problems);
            }
        }
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
            &check.id,
            &format!("{check_at}.id"),
            "knowledge check",
            problems,
        );

        if check_ids.contains(&check.id.as_str()) {
            problems.push(ContentProblem::new(
                format!("{check_at}.id"),
                format!(
                    "the check id `{}` is used more than once in this lesson",
                    check.id
                ),
            ));
        } else {
            check_ids.push(&check.id);
        }

        require_text(
            &check.prompt,
            &format!("{check_at}.prompt"),
            "a prompt",
            problems,
        );
        require_text(
            &check.explanation,
            &format!("{check_at}.explanation"),
            "an explanation",
            problems,
        );

        if check.options.len() < 2 {
            problems.push(ContentProblem::new(
                format!("{check_at}.options"),
                "a knowledge check needs at least two options",
            ));
        }

        for (option_index, option) in check.options.iter().enumerate() {
            require_text(
                option,
                &format!("{check_at}.options[{option_index}]"),
                "option text",
                problems,
            );
        }

        if check.answer >= check.options.len() {
            problems.push(ContentProblem::new(
                format!("{check_at}.answer"),
                format!(
                    "the answer index {} is outside the {} available options",
                    check.answer,
                    check.options.len()
                ),
            ));
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

    const EMBEDDED: &str = include_str!("../../content/programming/rust/foundations.json");

    #[test]
    fn embedded_content_is_valid() {
        let paths = parse_pack(EMBEDDED).expect("the bundled content must be valid");

        assert!(!paths.is_empty());
    }

    #[test]
    fn embedded_content_has_unique_check_ids_per_lesson() {
        for path in parse_pack(EMBEDDED).expect("valid content") {
            for lesson in path.lessons {
                let mut ids = lesson.check_ids();
                let total = ids.len();
                ids.sort();
                ids.dedup();

                assert_eq!(ids.len(), total, "duplicate check ids in {}", lesson.id);
            }
        }
    }

    #[test]
    fn lesson_views_never_include_answers_or_explanations() {
        let paths = parse_pack(EMBEDDED).expect("valid content");
        let view = paths[0].lessons[0].view();
        let json = serde_json::to_string(&view).expect("view should serialize");

        // Section kinds include "explanation", so look for keys, not words.
        assert!(!json.contains("\"answer\":"));
        assert!(!json.contains("\"explanation\":"));
    }

    #[test]
    fn invalid_answer_index_is_reported() {
        let json = r#"{
            "formatVersion": 1,
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
                        "id": "q1",
                        "prompt": "Pick one.",
                        "options": ["a", "b"],
                        "answer": 5,
                        "explanation": "Because."
                    }]
                }]
            }]
        }"#;

        let problems = parse_pack(json).expect_err("the answer index is out of range");

        assert!(
            problems
                .iter()
                .any(|problem| problem.location() == "paths[0].lessons[0].checks[0].answer"),
            "got {problems:?}"
        );
    }

    #[test]
    fn lesson_without_checks_is_rejected() {
        let json = r#"{
            "formatVersion": 1,
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
                    "checks": []
                }]
            }]
        }"#;

        let problems = parse_pack(json).expect_err("a lesson needs a check");

        assert!(
            problems
                .iter()
                .any(|problem| problem.location() == "paths[0].lessons[0].checks"),
            "got {problems:?}"
        );
    }

    #[test]
    fn unsupported_format_version_is_rejected() {
        let json = r#"{ "formatVersion": 99, "paths": [] }"#;

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
            "formatVersion": 1,
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
                        "sections": [{ "kind": "note", "body": "Read this." }],
                        "checks": [{
                            "id": "q1",
                            "prompt": "Pick one.",
                            "options": ["a", "b"],
                            "answer": 0,
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
                        "sections": [{ "kind": "note", "body": "Read this." }],
                        "checks": [{
                            "id": "q1",
                            "prompt": "Pick one.",
                            "options": ["a", "b"],
                            "answer": 0,
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
}
