//! Learning progress: what has been answered correctly and what counts as
//! finished.
//!
//! A lesson is finished only when every one of its knowledge checks has been
//! answered correctly. Opening a lesson does not complete it.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Identifies one lesson inside one learning path.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LessonKey {
    path_id: String,
    lesson_id: String,
}

impl LessonKey {
    /// Creates a key for a lesson in a path.
    pub fn new(path_id: impl Into<String>, lesson_id: impl Into<String>) -> Self {
        Self {
            path_id: path_id.into(),
            lesson_id: lesson_id.into(),
        }
    }

    /// The stable string form used in progress records, `path/lesson`.
    pub fn as_id(&self) -> String {
        format!("{}/{}", self.path_id, self.lesson_id)
    }
}

/// What happened when an answer was recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnswerEffect {
    /// The answer was stored; the lesson is still unfinished.
    Recorded,
    /// This answer finished the lesson.
    LessonCompleted,
}

/// The progress stored for a single lesson.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonRecord {
    #[serde(default)]
    correct_checks: BTreeSet<String>,
    #[serde(default)]
    completed_at: Option<u64>,
}

impl LessonRecord {
    /// When the lesson was finished, as seconds since the Unix epoch.
    pub fn completed_at(&self) -> Option<u64> {
        self.completed_at
    }

    /// How many checks have been answered correctly.
    pub fn answered_count(&self) -> usize {
        self.correct_checks.len()
    }
}

/// All progress known for this learner.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProgressState {
    lessons: BTreeMap<String, LessonRecord>,
}

impl ProgressState {
    /// Creates an empty progress state.
    pub fn new() -> Self {
        Self::default()
    }

    /// The record for a lesson, if the learner has touched it.
    pub fn lesson(&self, key: &LessonKey) -> Option<&LessonRecord> {
        self.lessons.get(&key.as_id())
    }

    /// Whether every check of the lesson has been answered correctly.
    pub fn is_complete(&self, key: &LessonKey) -> bool {
        self.lesson(key)
            .is_some_and(|record| record.completed_at.is_some())
    }

    /// Records a correct answer and reports whether it finished the lesson.
    ///
    /// `required_checks` is the full set of check ids the lesson expects; the
    /// lesson is finished the first time all of them are present. `now` is
    /// passed in so the completion time is testable.
    pub fn record_correct(
        &mut self,
        key: &LessonKey,
        check_id: &str,
        required_checks: &[String],
        now: u64,
    ) -> AnswerEffect {
        let record = self.lessons.entry(key.as_id()).or_default();

        record.correct_checks.insert(check_id.to_owned());

        let answered_all = required_checks
            .iter()
            .all(|required| record.correct_checks.contains(required));

        if record.completed_at.is_none() && answered_all {
            record.completed_at = Some(now);
            AnswerEffect::LessonCompleted
        } else {
            AnswerEffect::Recorded
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn checks() -> Vec<String> {
        vec!["first".to_owned(), "second".to_owned()]
    }

    fn key() -> LessonKey {
        LessonKey::new("rust-foundations", "first-program")
    }

    #[test]
    fn new_progress_is_empty() {
        let progress = ProgressState::new();

        assert!(progress.lesson(&key()).is_none());
        assert!(!progress.is_complete(&key()));
    }

    #[test]
    fn one_answer_does_not_complete_a_lesson() {
        let mut progress = ProgressState::new();

        let effect = progress.record_correct(&key(), "first", &checks(), 100);

        assert_eq!(effect, AnswerEffect::Recorded);
        assert!(!progress.is_complete(&key()));
        assert_eq!(
            progress.lesson(&key()).map(LessonRecord::answered_count),
            Some(1)
        );
    }

    #[test]
    fn every_answer_completes_the_lesson_once() {
        let mut progress = ProgressState::new();

        progress.record_correct(&key(), "first", &checks(), 100);
        let effect = progress.record_correct(&key(), "second", &checks(), 120);

        assert_eq!(effect, AnswerEffect::LessonCompleted);
        assert!(progress.is_complete(&key()));
        assert_eq!(
            progress.lesson(&key()).and_then(LessonRecord::completed_at),
            Some(120)
        );
    }

    #[test]
    fn answering_again_keeps_the_original_completion_time() {
        let mut progress = ProgressState::new();

        progress.record_correct(&key(), "first", &checks(), 100);
        progress.record_correct(&key(), "second", &checks(), 120);
        let effect = progress.record_correct(&key(), "first", &checks(), 999);

        assert_eq!(effect, AnswerEffect::Recorded);
        assert_eq!(
            progress.lesson(&key()).and_then(LessonRecord::completed_at),
            Some(120)
        );
    }

    #[test]
    fn lessons_are_tracked_independently() {
        let mut progress = ProgressState::new();
        let other = LessonKey::new("rust-foundations", "variables");

        progress.record_correct(&key(), "first", &checks(), 100);

        assert!(progress.lesson(&key()).is_some());
        assert!(progress.lesson(&other).is_none());
    }

    #[test]
    fn partial_answers_are_counted_for_display() {
        let mut progress = ProgressState::new();

        progress.record_correct(&key(), "first", &checks(), 100);

        let record = progress.lesson(&key()).expect("record exists");

        assert_eq!(record.answered_count(), 1);
        assert!(record.completed_at().is_none());
    }

    #[test]
    fn progress_round_trips_through_serialization() {
        let mut progress = ProgressState::new();
        progress.record_correct(&key(), "first", &checks(), 100);
        progress.record_correct(&key(), "second", &checks(), 120);

        let json = serde_json::to_string(&progress).expect("progress should serialize");
        let restored: ProgressState =
            serde_json::from_str(&json).expect("progress should parse back");

        assert_eq!(restored, progress);
        assert!(restored.is_complete(&key()));
    }

    #[test]
    fn incomplete_records_parse_without_a_completion_time() {
        let json = r#"{"rust-foundations/first-program":{"correctChecks":["first"]}}"#;

        let progress: ProgressState =
            serde_json::from_str(json).expect("legacy shape should parse");

        assert_eq!(
            progress.lesson(&key()).map(LessonRecord::answered_count),
            Some(1)
        );
        assert!(!progress.is_complete(&key()));
    }
}
