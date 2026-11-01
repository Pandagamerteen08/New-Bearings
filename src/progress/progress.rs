//! Learning progress: what the learner has answered, where they stopped, what
//! they saved, and what counts as finished.
//!
//! A lesson is finished only when every one of its knowledge checks has been
//! answered correctly. Opening a lesson or answering it wrongly does not
//! complete it, but both are remembered: the last opened lesson makes "continue
//! learning" meaningful, and wrong answers build the review queue.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Identifies one lesson inside one learning path.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

    /// The path this lesson belongs to.
    pub fn path_id(&self) -> &str {
        &self.path_id
    }

    /// The lesson id inside its path.
    pub fn lesson_id(&self) -> &str {
        &self.lesson_id
    }

    /// The stable string form used in progress records, `path/lesson`.
    pub fn as_id(&self) -> String {
        format!("{}/{lesson_id}", self.path_id, lesson_id = self.lesson_id)
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

/// The progress stored for a single knowledge check.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckRecord {
    /// Every answer ever submitted for this check.
    #[serde(default)]
    pub attempts: u32,
    /// How many of those answers were wrong.
    #[serde(default)]
    pub mistakes: u32,
    /// Whether the check has been answered correctly at least once.
    #[serde(default)]
    pub correct: bool,
    /// When an answer was last submitted, as seconds since the Unix epoch.
    #[serde(default)]
    pub last_attempt_at: u64,
}

impl CheckRecord {
    fn record(&mut self, correct: bool, now: u64) {
        self.attempts = self.attempts.saturating_add(1);
        self.last_attempt_at = now;

        if correct {
            self.correct = true;
        } else {
            self.mistakes = self.mistakes.saturating_add(1);
        }
    }
}

/// The progress stored for a single lesson.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonRecord {
    #[serde(default)]
    checks: BTreeMap<String, CheckRecord>,
    #[serde(default)]
    completed_at: Option<u64>,
}

impl LessonRecord {
    /// When the lesson was finished, as seconds since the Unix epoch.
    pub fn completed_at(&self) -> Option<u64> {
        self.completed_at
    }

    /// The state of one check, if it has been attempted.
    pub fn check(&self, check_id: &str) -> Option<&CheckRecord> {
        self.checks.get(check_id)
    }

    /// How many checks have been answered correctly.
    pub fn solved_count(&self) -> usize {
        self.checks.values().filter(|check| check.correct).count()
    }

    /// How many answers have been submitted in this lesson.
    pub fn attempt_count(&self) -> u32 {
        self.checks.values().map(|check| check.attempts).sum()
    }

    /// How many wrong answers have been submitted in this lesson.
    pub fn mistakes(&self) -> u32 {
        self.checks.values().map(|check| check.mistakes).sum()
    }

    /// When an answer was last submitted in this lesson.
    pub fn last_attempt_at(&self) -> u64 {
        self.checks
            .values()
            .map(|check| check.last_attempt_at)
            .max()
            .unwrap_or(0)
    }
}

/// All progress known for this learner.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressState {
    #[serde(default)]
    lessons: BTreeMap<String, LessonRecord>,
    #[serde(default)]
    bookmarks: BTreeSet<String>,
    /// The lesson the learner opened most recently, so "continue" can return to
    /// it exactly. A pointer is used rather than a timestamp because two
    /// openings can happen within the same instant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    last_opened: Option<LessonKey>,
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

    /// Records an answer and reports whether it finished the lesson.
    ///
    /// `required_checks` is the full set of check ids the lesson expects; the
    /// lesson is finished the first time all of them are correct. `now` is
    /// passed in so the recorded times are testable.
    pub fn record_attempt(
        &mut self,
        key: &LessonKey,
        check_id: &str,
        correct: bool,
        required_checks: &[String],
        now: u64,
    ) -> AnswerEffect {
        let record = self.lessons.entry(key.as_id()).or_default();

        record
            .checks
            .entry(check_id.to_owned())
            .or_default()
            .record(correct, now);

        let solved_all = required_checks.iter().all(|required| {
            record
                .checks
                .get(required)
                .is_some_and(|check| check.correct)
        });

        if correct && solved_all && record.completed_at.is_none() {
            record.completed_at = Some(now);
            AnswerEffect::LessonCompleted
        } else {
            AnswerEffect::Recorded
        }
    }

    /// Notes that the learner opened a lesson.
    pub fn mark_opened(&mut self, key: &LessonKey) {
        self.lessons.entry(key.as_id()).or_default();
        self.last_opened = Some(key.clone());
    }

    /// The lesson the learner opened most recently, if any.
    pub fn last_opened(&self) -> Option<&LessonKey> {
        self.last_opened.as_ref()
    }

    /// Whether a lesson has been saved for later.
    pub fn is_bookmarked(&self, key: &LessonKey) -> bool {
        self.bookmarks.contains(&key.as_id())
    }

    /// Saves or unsaves a lesson, returning whether it is now saved.
    pub fn toggle_bookmark(&mut self, key: &LessonKey) -> bool {
        let id = key.as_id();

        if self.bookmarks.remove(&id) {
            false
        } else {
            self.bookmarks.insert(id);
            true
        }
    }

    /// How many lessons are saved for later.
    pub fn bookmark_count(&self) -> usize {
        self.bookmarks.len()
    }

    /// Rebuilds progress from the version 1 record shape.
    ///
    /// Version 1 stored only the checks that had been answered correctly, so
    /// each of them becomes a single successful attempt and the saved completion
    /// time is preserved.
    pub fn from_legacy(legacy: BTreeMap<String, LegacyLessonRecord>) -> Self {
        let mut progress = Self::default();

        for (key, record) in legacy {
            let mut lesson = LessonRecord::default();

            for check_id in record.correct_checks {
                lesson.checks.insert(
                    check_id,
                    CheckRecord {
                        attempts: 1,
                        mistakes: 0,
                        correct: true,
                        last_attempt_at: record.completed_at.unwrap_or(0),
                    },
                );
            }

            lesson.completed_at = record.completed_at;
            progress.lessons.insert(key, lesson);
        }

        progress
    }
}

/// The lesson record shape written by format version 1 of the saved file.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyLessonRecord {
    #[serde(default)]
    pub correct_checks: BTreeSet<String>,
    #[serde(default)]
    pub completed_at: Option<u64>,
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
        assert_eq!(progress.bookmark_count(), 0);
    }

    #[test]
    fn one_answer_does_not_complete_a_lesson() {
        let mut progress = ProgressState::new();

        let effect = progress.record_attempt(&key(), "first", true, &checks(), 100);

        assert_eq!(effect, AnswerEffect::Recorded);
        assert!(!progress.is_complete(&key()));
        assert_eq!(
            progress.lesson(&key()).map(LessonRecord::solved_count),
            Some(1)
        );
    }

    #[test]
    fn every_answer_completes_the_lesson_once() {
        let mut progress = ProgressState::new();

        progress.record_attempt(&key(), "first", true, &checks(), 100);
        let effect = progress.record_attempt(&key(), "second", true, &checks(), 120);

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

        progress.record_attempt(&key(), "first", true, &checks(), 100);
        progress.record_attempt(&key(), "second", true, &checks(), 120);
        let effect = progress.record_attempt(&key(), "first", true, &checks(), 999);

        assert_eq!(effect, AnswerEffect::Recorded);
        assert_eq!(
            progress.lesson(&key()).and_then(LessonRecord::completed_at),
            Some(120)
        );
        assert_eq!(
            progress.lesson(&key()).map(LessonRecord::attempt_count),
            Some(3),
            "review attempts are counted"
        );
    }

    #[test]
    fn a_wrong_answer_is_remembered_but_not_solved() {
        let mut progress = ProgressState::new();

        let effect = progress.record_attempt(&key(), "first", false, &checks(), 50);

        let record = progress.lesson(&key()).expect("record exists");

        assert_eq!(effect, AnswerEffect::Recorded);
        assert_eq!(record.solved_count(), 0);
        assert_eq!(record.attempt_count(), 1);
        assert_eq!(record.mistakes(), 1);
        assert!(!progress.is_complete(&key()));
    }

    #[test]
    fn a_wrong_answer_cannot_complete_a_lesson() {
        let mut progress = ProgressState::new();

        progress.record_attempt(&key(), "first", true, &checks(), 10);
        let effect = progress.record_attempt(&key(), "second", false, &checks(), 20);

        assert_eq!(effect, AnswerEffect::Recorded);
        assert!(!progress.is_complete(&key()));
        assert_eq!(progress.lesson(&key()).map(LessonRecord::mistakes), Some(1));
    }

    #[test]
    fn answering_correctly_after_a_mistake_keeps_the_mistake_for_review() {
        let mut progress = ProgressState::new();

        progress.record_attempt(&key(), "first", false, &checks(), 10);
        progress.record_attempt(&key(), "first", true, &checks(), 20);

        let record = progress.lesson(&key()).expect("record exists");

        assert_eq!(record.solved_count(), 1);
        assert_eq!(record.mistakes(), 1);
        assert_eq!(record.attempt_count(), 2);
    }

    #[test]
    fn lessons_are_tracked_independently() {
        let mut progress = ProgressState::new();
        let other = LessonKey::new("rust-foundations", "variables");

        progress.record_attempt(&key(), "first", true, &checks(), 100);

        assert!(progress.lesson(&key()).is_some());
        assert!(progress.lesson(&other).is_none());
    }
    #[test]
    fn opening_a_lesson_records_where_the_learner_stopped() {
        let mut progress = ProgressState::new();
        let later = LessonKey::new("rust-foundations", "variables");

        progress.mark_opened(&key());
        progress.mark_opened(&later);

        assert_eq!(progress.last_opened(), Some(&later));
        assert!(progress.lesson(&key()).is_some());
        assert!(!progress.is_complete(&key()));
    }

    #[test]
    fn bookmarks_can_be_toggled() {
        let mut progress = ProgressState::new();

        assert!(!progress.is_bookmarked(&key()));
        assert!(progress.toggle_bookmark(&key()));
        assert!(progress.is_bookmarked(&key()));
        assert_eq!(progress.bookmark_count(), 1);

        assert!(!progress.toggle_bookmark(&key()));
        assert!(!progress.is_bookmarked(&key()));
        assert_eq!(progress.bookmark_count(), 0);
    }

    #[test]
    fn progress_round_trips_through_serialization() {
        let mut progress = ProgressState::new();
        progress.record_attempt(&key(), "first", true, &checks(), 100);
        progress.record_attempt(&key(), "second", false, &checks(), 110);
        progress.record_attempt(&key(), "second", true, &checks(), 120);
        progress.mark_opened(&key());
        progress.toggle_bookmark(&key());

        let json = serde_json::to_string(&progress).expect("progress should serialize");
        let restored: ProgressState =
            serde_json::from_str(&json).expect("progress should parse back");

        assert_eq!(restored, progress);
        assert!(restored.is_complete(&key()));
        assert!(restored.is_bookmarked(&key()));
    }

    #[test]
    fn version_one_records_are_migrated() {
        let mut legacy = BTreeMap::new();
        legacy.insert(
            String::from("rust-foundations/first-program"),
            LegacyLessonRecord {
                correct_checks: ["first".to_owned(), "second".to_owned()].into(),
                completed_at: Some(1_700_000_000),
            },
        );
        legacy.insert(
            String::from("rust-foundations/variables"),
            LegacyLessonRecord {
                correct_checks: ["third".to_owned()].into(),
                completed_at: None,
            },
        );

        let progress = ProgressState::from_legacy(legacy);
        let complete = LessonKey::new("rust-foundations", "first-program");
        let partial = LessonKey::new("rust-foundations", "variables");

        assert!(progress.is_complete(&complete));
        assert_eq!(
            progress
                .lesson(&complete)
                .and_then(LessonRecord::completed_at),
            Some(1_700_000_000)
        );
        assert_eq!(
            progress.lesson(&complete).map(LessonRecord::solved_count),
            Some(2)
        );
        assert_eq!(
            progress.lesson(&complete).map(LessonRecord::mistakes),
            Some(0)
        );

        assert!(!progress.is_complete(&partial));
        assert_eq!(
            progress.lesson(&partial).map(LessonRecord::solved_count),
            Some(1)
        );
    }

    #[test]
    fn records_written_without_optional_fields_still_parse() {
        let json = r#"{
            "lessons": {
                "rust-foundations/first-program": {
                    "checks": { "first": { "attempts": 2, "mistakes": 1, "correct": true } }
                }
            }
        }"#;

        let progress: ProgressState = serde_json::from_str(json).expect("shape should parse");
        let record = progress.lesson(&key()).expect("record exists");

        assert_eq!(record.solved_count(), 1);
        assert_eq!(record.mistakes(), 1);
        assert_eq!(record.completed_at(), None);
        assert_eq!(progress.bookmark_count(), 0);
    }
}
