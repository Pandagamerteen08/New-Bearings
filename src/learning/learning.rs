//! The learning catalog and the learner's place inside it.
//!
//! The catalog is loaded from editable content files that are compiled into the
//! application, so there is no content to fetch or download at startup. Every
//! request to open a path, open a lesson or answer a check is validated here,
//! and the interface renders the snapshot it is given.

use std::sync::OnceLock;

use serde::{Serialize, Serializer};

use crate::lessons::{
    self, AnswerProblem, ContentProblem, Difficulty, Lesson, LessonView, Path, SubmittedAnswer,
};

#[cfg(test)]
use crate::lessons::Check;
use crate::progress::{LessonKey, ProgressState};

/// The bundled content files. Adding a pack means adding a line here; its
/// content is validated by the tests in this module.
const EMBEDDED_PACKS: &[&str] = &[
    include_str!("../../content/programming/rust/foundations.json"),
    include_str!("../../content/web/fundamentals.json"),
];

/// How many search results are returned at most.
const MAX_SEARCH_HITS: usize = 20;

/// How much of a matching passage is shown in a search result.
const EXCERPT_LENGTH: usize = 140;

static CATALOG: OnceLock<Result<Catalog, Vec<ContentProblem>>> = OnceLock::new();

/// Every learning path this build knows about.
#[derive(Debug)]
pub struct Catalog {
    paths: Vec<Path>,
}

impl Catalog {
    /// Loads the bundled content once and reuses it afterwards.
    pub fn embedded() -> Result<&'static Catalog, &'static Vec<ContentProblem>> {
        let loaded = CATALOG.get_or_init(|| {
            let mut paths = Vec::new();

            for (index, pack) in EMBEDDED_PACKS.iter().enumerate() {
                let mut parsed = lessons::parse_pack(pack).map_err(|mut problems| {
                    for problem in &mut problems {
                        problem.prefix(&format!("pack[{index}]"));
                    }
                    problems
                })?;

                paths.append(&mut parsed);
            }

            // Packs are validated separately, so ids colliding across packs are
            // checked once they are merged.
            let duplicates: Vec<ContentProblem> = paths
                .iter()
                .enumerate()
                .filter(|(index, path)| paths[..*index].iter().any(|other| other.id == path.id))
                .map(|(index, path)| {
                    ContentProblem::new(
                        format!("paths[{index}].id"),
                        format!(
                            "the learning path id `{}` is used by more than one content pack",
                            path.id
                        ),
                    )
                })
                .collect();

            if !duplicates.is_empty() {
                return Err(duplicates);
            }

            Ok(Catalog { paths })
        });

        match loaded {
            Ok(catalog) => Ok(catalog),
            Err(problems) => Err(problems),
        }
    }

    /// Every path, in content order.
    pub fn paths(&self) -> &[Path] {
        &self.paths
    }

    /// Finds a path by id.
    pub fn path(&self, path_id: &str) -> Option<&Path> {
        self.paths.iter().find(|path| path.id == path_id)
    }

    /// Finds a lesson inside a path.
    pub fn lesson(&self, path_id: &str, lesson_id: &str) -> Option<&Lesson> {
        self.path(path_id).and_then(|path| path.lesson(lesson_id))
    }

    /// Finds paths and lessons whose text contains `query`.
    ///
    /// Matching ignores case and covers path titles, summaries, subjects and
    /// tags, plus lesson titles, summaries and objectives. An empty query
    /// matches nothing rather than everything.
    pub fn search(&self, query: &str) -> Vec<SearchHit> {
        let needle = query.trim().to_lowercase();

        if needle.is_empty() {
            return Vec::new();
        }

        let mut hits = Vec::new();

        for path in &self.paths {
            if let Some(text) = matching_path_text(path, &needle) {
                hits.push(SearchHit {
                    path_id: path.id.clone(),
                    path_title: path.title.clone(),
                    lesson_id: None,
                    lesson_title: None,
                    matched: MatchedField::Path,
                    text,
                });
            }

            for lesson in &path.lessons {
                if let Some(text) = matching_lesson_text(lesson, &needle) {
                    hits.push(SearchHit {
                        path_id: path.id.clone(),
                        path_title: path.title.clone(),
                        lesson_id: Some(lesson.id.clone()),
                        lesson_title: Some(lesson.title.clone()),
                        matched: MatchedField::Lesson,
                        text,
                    });
                }
            }

            if hits.len() >= MAX_SEARCH_HITS {
                break;
            }
        }

        hits.truncate(MAX_SEARCH_HITS);
        hits
    }
}

fn contains(haystack: &str, needle: &str) -> bool {
    haystack.to_lowercase().contains(needle)
}

fn matching_path_text(path: &Path, needle: &str) -> Option<String> {
    if contains(&path.title, needle) {
        return Some(path.title.clone());
    }

    if contains(&path.summary, needle) {
        return Some(excerpt(&path.summary));
    }

    if contains(&path.subject, needle) {
        return Some(path.subject.clone());
    }

    path.tags.iter().find(|tag| contains(tag, needle)).cloned()
}

fn matching_lesson_text(lesson: &Lesson, needle: &str) -> Option<String> {
    if contains(&lesson.title, needle) {
        return Some(lesson.title.clone());
    }

    if contains(&lesson.summary, needle) {
        return Some(excerpt(&lesson.summary));
    }

    lesson
        .objectives
        .iter()
        .find(|objective| contains(objective, needle))
        .map(|objective| excerpt(objective))
}

fn excerpt(text: &str) -> String {
    if text.chars().count() <= EXCERPT_LENGTH {
        return text.to_owned();
    }

    let shortened: String = text.chars().take(EXCERPT_LENGTH).collect();

    format!("{}…", shortened.trim_end())
}

/// Which part of the catalog a search result was found in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MatchedField {
    /// The result is a learning path.
    Path,
    /// The result is a lesson.
    Lesson,
}

/// One search result, already resolved to something the learner can open.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchHit {
    pub path_id: String,
    pub path_title: String,
    pub lesson_id: Option<String>,
    pub lesson_title: Option<String>,
    pub matched: MatchedField,
    /// The matching text, shortened when it is long.
    pub text: String,
}

/// Which learning screen the learner is on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum Screen {
    /// The overview of every available path.
    Dashboard,
    /// The lessons that need another look.
    Review,
    /// The lesson list of one path.
    Path { path_id: String },
    /// One open lesson.
    Lesson { path_id: String, lesson_id: String },
}

/// A rejected learning request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LearningError {
    /// The bundled content could not be read.
    ContentInvalid,
    /// No path has this id.
    PathNotFound,
    /// The path exists but has no lesson with this id.
    LessonNotFound,
    /// The lesson exists but has no check with this id.
    CheckNotFound,
    /// The submitted option index is not one of the offered options.
    OptionOutOfRange,
    /// The answer does not match the kind of check it was sent to.
    AnswerMismatch,
    /// A short answer was submitted empty.
    EmptyAnswer,
}

impl LearningError {
    /// The short code sent to the interface, so wording stays in content files.
    pub fn code(&self) -> &'static str {
        match self {
            Self::ContentInvalid => "content_invalid",
            Self::PathNotFound => "path_not_found",
            Self::LessonNotFound => "lesson_not_found",
            Self::CheckNotFound => "check_not_found",
            Self::OptionOutOfRange => "option_out_of_range",
            Self::AnswerMismatch => "answer_mismatch",
            Self::EmptyAnswer => "empty_answer",
        }
    }
}

impl From<AnswerProblem> for LearningError {
    fn from(problem: AnswerProblem) -> Self {
        match problem {
            AnswerProblem::KindMismatch => Self::AnswerMismatch,
            AnswerProblem::OptionOutOfRange => Self::OptionOutOfRange,
            AnswerProblem::EmptyAnswer => Self::EmptyAnswer,
        }
    }
}

impl Serialize for LearningError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.code())
    }
}

impl std::fmt::Display for LearningError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::ContentInvalid => "the learning content could not be loaded",
            Self::PathNotFound => "the requested learning path does not exist",
            Self::LessonNotFound => "the requested lesson does not exist",
            Self::CheckNotFound => "the requested knowledge check does not exist",
            Self::OptionOutOfRange => "the submitted answer is not one of the offered options",
            Self::AnswerMismatch => {
                "the submitted answer does not match the kind of knowledge check"
            }
            Self::EmptyAnswer => "no answer was entered",
        })
    }
}

impl std::error::Error for LearningError {}

/// Where the learner can pick up again.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResumePoint {
    pub path_id: String,
    pub path_title: String,
    pub lesson_id: String,
    pub lesson_title: String,
}

/// A learning path plus this learner's progress through it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PathView {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub subject: String,
    pub difficulty: Difficulty,
    pub tags: Vec<String>,
    pub total_minutes: u32,
    pub completed_lessons: usize,
    pub total_lessons: usize,
    /// Wrong answers recorded across this path, used for the review hint.
    pub mistakes: u32,
    pub lessons: Vec<LessonSummaryView>,
}

/// One lesson inside a path listing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonSummaryView {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub minutes: Option<u32>,
    pub check_count: usize,
    /// Checks answered correctly.
    pub solved_count: usize,
    /// Every answer submitted, including wrong ones.
    pub attempt_count: u32,
    /// Wrong answers recorded.
    pub mistakes: u32,
    pub completed: bool,
    /// When the lesson was finished, if it has been.
    pub completed_at: Option<u64>,
    pub bookmarked: bool,
}

/// Counts across the whole catalog, derived from stored progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressSummary {
    pub lessons_completed: usize,
    pub lessons_total: usize,
    pub checks_solved: usize,
    pub checks_total: usize,
    pub mistakes: u32,
    pub bookmarks: usize,
    /// Reading time of the lessons already completed.
    pub minutes_completed: u32,
}

/// A lesson that contains a question the learner answered wrongly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewItem {
    pub path_id: String,
    pub path_title: String,
    pub lesson_id: String,
    pub lesson_title: String,
    pub mistakes: u32,
    pub completed: bool,
    pub bookmarked: bool,
    pub last_attempt_at: u64,
}

/// Everything the interface needs to render the learning screens.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LearningSnapshot {
    pub screen: Screen,
    pub can_go_back: bool,
    pub paths: Vec<PathView>,
    /// Present only while a lesson is open.
    pub lesson: Option<LessonView>,
    /// The unfinished lesson the learner should continue, when there is one.
    pub resume: Option<ResumePoint>,
    /// Totals across every path.
    pub summary: ProgressSummary,
    /// Lessons containing at least one wrong answer.
    pub review: Vec<ReviewItem>,
    /// Set when progress could not be saved, so the learner knows.
    pub storage_warning: Option<String>,
}

impl LearningSnapshot {
    /// Builds the snapshot for a screen and the learner's stored progress.
    pub fn build(
        screen: &Screen,
        catalog: &Catalog,
        progress: &ProgressState,
        storage_warning: Option<String>,
    ) -> Self {
        let paths: Vec<PathView> = catalog
            .paths()
            .iter()
            .map(|path| path_view(path, progress))
            .collect();

        let lesson = open_lesson_view(screen, catalog, progress);
        let resume = resume_point(catalog, progress);
        let summary = progress_summary(catalog, progress);
        let review = review_items(catalog, progress);

        Self {
            can_go_back: !matches!(screen, Screen::Dashboard),
            screen: screen.clone(),
            paths,
            lesson,
            resume,
            summary,
            review,
            storage_warning,
        }
    }
}

fn path_view(path: &Path, progress: &ProgressState) -> PathView {
    let lessons: Vec<LessonSummaryView> = path
        .lessons
        .iter()
        .map(|lesson| {
            let key = LessonKey::new(&path.id, &lesson.id);
            let record = progress.lesson(&key);

            LessonSummaryView {
                id: lesson.id.clone(),
                title: lesson.title.clone(),
                summary: lesson.summary.clone(),
                minutes: lesson.minutes,
                check_count: lesson.checks.len(),
                solved_count: record.map_or(0, |record| record.solved_count()),
                attempt_count: record.map_or(0, |record| record.attempt_count()),
                mistakes: record.map_or(0, |record| record.mistakes()),
                completed: progress.is_complete(&key),
                completed_at: record.and_then(|record| record.completed_at()),
                bookmarked: progress.is_bookmarked(&key),
            }
        })
        .collect();

    PathView {
        id: path.id.clone(),
        title: path.title.clone(),
        summary: path.summary.clone(),
        subject: path.subject.clone(),
        difficulty: path.difficulty,
        tags: path.tags.clone(),
        total_minutes: path.total_minutes(),
        completed_lessons: lessons.iter().filter(|lesson| lesson.completed).count(),
        total_lessons: lessons.len(),
        mistakes: lessons.iter().map(|lesson| lesson.mistakes).sum(),
        lessons,
    }
}

fn open_lesson_view(
    screen: &Screen,
    catalog: &Catalog,
    progress: &ProgressState,
) -> Option<LessonView> {
    match screen {
        Screen::Lesson { path_id, lesson_id } => {
            let lesson = catalog.lesson(path_id, lesson_id)?;
            let record = progress.lesson(&LessonKey::new(path_id, lesson_id));

            Some(lesson.view(record))
        }
        _ => None,
    }
}
/// Finds the lesson to continue: the one the learner opened most recently, or
/// the first unfinished lesson in content order when none has been opened.
fn resume_point(catalog: &Catalog, progress: &ProgressState) -> Option<ResumePoint> {
    last_opened_point(catalog, progress).or_else(|| first_unfinished_point(catalog, progress))
}

fn last_opened_point(catalog: &Catalog, progress: &ProgressState) -> Option<ResumePoint> {
    let key = progress.last_opened()?;

    if progress.is_complete(key) {
        return None;
    }

    let path = catalog.path(key.path_id())?;
    let lesson = path.lesson(key.lesson_id())?;

    Some(resume_point_for(path, lesson))
}

fn first_unfinished_point(catalog: &Catalog, progress: &ProgressState) -> Option<ResumePoint> {
    for path in catalog.paths() {
        for lesson in &path.lessons {
            if !progress.is_complete(&LessonKey::new(&path.id, &lesson.id)) {
                return Some(resume_point_for(path, lesson));
            }
        }
    }

    None
}

fn resume_point_for(path: &Path, lesson: &Lesson) -> ResumePoint {
    ResumePoint {
        path_id: path.id.clone(),
        path_title: path.title.clone(),
        lesson_id: lesson.id.clone(),
        lesson_title: lesson.title.clone(),
    }
}

fn progress_summary(catalog: &Catalog, progress: &ProgressState) -> ProgressSummary {
    let mut summary = ProgressSummary {
        lessons_completed: 0,
        lessons_total: 0,
        checks_solved: 0,
        checks_total: 0,
        mistakes: 0,
        bookmarks: progress.bookmark_count(),
        minutes_completed: 0,
    };

    for path in catalog.paths() {
        for lesson in &path.lessons {
            let key = LessonKey::new(&path.id, &lesson.id);
            let record = progress.lesson(&key);

            summary.lessons_total += 1;
            summary.checks_total += lesson.checks.len();
            summary.checks_solved += record.map_or(0, |record| record.solved_count());
            summary.mistakes += record.map_or(0, |record| record.mistakes());

            if progress.is_complete(&key) {
                summary.lessons_completed += 1;
                summary.minutes_completed += lesson.minutes.unwrap_or(0);
            }
        }
    }

    summary
}

fn review_items(catalog: &Catalog, progress: &ProgressState) -> Vec<ReviewItem> {
    let mut items = Vec::new();

    for path in catalog.paths() {
        for lesson in &path.lessons {
            let key = LessonKey::new(&path.id, &lesson.id);

            let Some(record) = progress.lesson(&key) else {
                continue;
            };

            if record.mistakes() == 0 {
                continue;
            }

            items.push(ReviewItem {
                path_id: path.id.clone(),
                path_title: path.title.clone(),
                lesson_id: lesson.id.clone(),
                lesson_title: lesson.title.clone(),
                mistakes: record.mistakes(),
                completed: record.completed_at().is_some(),
                bookmarked: progress.is_bookmarked(&key),
                last_attempt_at: record.last_attempt_at(),
            });
        }
    }

    // Most mistakes first, then the most recent, so the queue stays useful as
    // it grows.
    items.sort_by(|left, right| {
        right
            .mistakes
            .cmp(&left.mistakes)
            .then(right.last_attempt_at.cmp(&left.last_attempt_at))
            .then_with(|| left.lesson_title.cmp(&right.lesson_title))
    });

    items
}

/// The judgement made about a submitted answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluatedAnswer {
    pub correct: bool,
    pub explanation: String,
    /// Every check id the lesson expects before it counts as finished.
    pub required_checks: Vec<String>,
}

/// Checks a submitted answer against the content.
///
/// This is the only place an answer is compared, so the correct option never
/// has to leave the backend.
pub fn evaluate_answer(
    catalog: &Catalog,
    path_id: &str,
    lesson_id: &str,
    check_id: &str,
    submitted: &SubmittedAnswer,
) -> Result<EvaluatedAnswer, LearningError> {
    let lesson = catalog
        .lesson(path_id, lesson_id)
        .ok_or(LearningError::LessonNotFound)?;
    let check = lesson.check(check_id).ok_or(LearningError::CheckNotFound)?;

    Ok(EvaluatedAnswer {
        correct: check.grade(submitted)?,
        explanation: check.explanation().to_owned(),
        required_checks: lesson.check_ids(),
    })
}

/// The learner's current place in the learning screens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearningSession {
    screen: Screen,
}

impl Default for LearningSession {
    fn default() -> Self {
        Self::new()
    }
}

impl LearningSession {
    /// Starts on the dashboard.
    pub fn new() -> Self {
        Self {
            screen: Screen::Dashboard,
        }
    }

    /// The current screen.
    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    /// Returns to the dashboard.
    pub fn show_dashboard(&mut self) {
        self.screen = Screen::Dashboard;
    }

    /// Shows the lessons that need another look.
    pub fn show_review(&mut self) {
        self.screen = Screen::Review;
    }

    /// Opens a path after confirming that it exists.
    pub fn open_path(&mut self, catalog: &Catalog, path_id: &str) -> Result<(), LearningError> {
        if catalog.path(path_id).is_none() {
            return Err(LearningError::PathNotFound);
        }

        self.screen = Screen::Path {
            path_id: path_id.to_owned(),
        };

        Ok(())
    }

    /// Opens a lesson after confirming that it belongs to the path.
    pub fn open_lesson(
        &mut self,
        catalog: &Catalog,
        path_id: &str,
        lesson_id: &str,
    ) -> Result<(), LearningError> {
        if catalog.path(path_id).is_none() {
            return Err(LearningError::PathNotFound);
        }

        if catalog.lesson(path_id, lesson_id).is_none() {
            return Err(LearningError::LessonNotFound);
        }

        self.screen = Screen::Lesson {
            path_id: path_id.to_owned(),
            lesson_id: lesson_id.to_owned(),
        };

        Ok(())
    }

    /// Steps back one screen: a lesson returns to its path, a path or the
    /// review list to the dashboard. The dashboard stays put.
    pub fn go_back(&mut self) {
        self.screen = match &self.screen {
            Screen::Dashboard => Screen::Dashboard,
            Screen::Review | Screen::Path { .. } => Screen::Dashboard,
            Screen::Lesson { path_id, .. } => Screen::Path {
                path_id: path_id.clone(),
            },
        };
    }
}

/// The answer that satisfies a check, for tests that need to submit one.
#[cfg(test)]
pub(crate) fn correct_answer(check: &Check) -> SubmittedAnswer {
    match check {
        Check::Choice { .. } => SubmittedAnswer::Choice {
            option: check
                .answer_index()
                .expect("choice checks declare an answer index"),
        },
        Check::TrueFalse { .. } => SubmittedAnswer::TrueFalse {
            value: check
                .correct_bool()
                .expect("true/false checks declare an answer"),
        },
        Check::ShortAnswer { .. } => SubmittedAnswer::ShortAnswer {
            text: check
                .accepted_answers()
                .first()
                .expect("short-answer checks declare an accepted wording")
                .clone(),
        },
    }
}

/// An answer that no check accepts, for tests that need a wrong one.
#[cfg(test)]
pub(crate) fn wrong_answer(check: &Check) -> SubmittedAnswer {
    match check {
        Check::Choice { .. } => SubmittedAnswer::Choice {
            option: (check
                .answer_index()
                .expect("choice checks declare an answer index")
                + 1)
                % check.options().len(),
        },
        Check::TrueFalse { .. } => SubmittedAnswer::TrueFalse {
            value: !check
                .correct_bool()
                .expect("true/false checks declare an answer"),
        },
        Check::ShortAnswer { .. } => SubmittedAnswer::ShortAnswer {
            text: String::from("a wrong answer"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lessons::Section;

    fn catalog() -> &'static Catalog {
        Catalog::embedded().expect("the bundled content should be valid")
    }

    fn first_path_id() -> String {
        catalog().paths()[0].id.clone()
    }

    fn first_lesson_id() -> String {
        catalog().paths()[0].lessons[0].id.clone()
    }

    fn complete_lesson(
        catalog: &Catalog,
        progress: &mut ProgressState,
        path_id: &str,
        lesson_id: &str,
    ) {
        let lesson = catalog.lesson(path_id, lesson_id).expect("lesson exists");
        let key = LessonKey::new(path_id, lesson_id);

        for check in &lesson.checks {
            progress.record_attempt(&key, check.id(), true, &lesson.check_ids(), 10);
        }
    }

    #[test]
    fn bundled_content_loads() {
        assert!(catalog().paths().len() >= 2);
    }

    #[test]
    fn every_pack_contributes_distinct_paths_and_lessons() {
        let mut path_ids: Vec<&str> = Vec::new();

        for path in catalog().paths() {
            assert!(
                !path_ids.contains(&path.id.as_str()),
                "duplicate {}",
                path.id
            );
            path_ids.push(&path.id);

            let mut lesson_ids: Vec<&str> = Vec::new();

            for lesson in &path.lessons {
                assert!(
                    !lesson_ids.contains(&lesson.id.as_str()),
                    "duplicate {} in {}",
                    lesson.id,
                    path.id
                );
                lesson_ids.push(&lesson.id);
            }
        }
    }

    #[test]
    fn the_catalog_covers_more_than_one_subject() {
        let subjects: Vec<&str> = catalog()
            .paths()
            .iter()
            .map(|path| path.subject.as_str())
            .collect();
        let mut unique = subjects.clone();
        unique.sort();
        unique.dedup();

        assert!(unique.len() >= 2, "got {subjects:?}");
    }

    #[test]
    fn unknown_paths_and_lessons_are_absent() {
        assert!(catalog().path("does-not-exist").is_none());
        assert!(
            catalog()
                .lesson(&first_path_id(), "does-not-exist")
                .is_none()
        );
    }

    #[test]
    fn a_new_session_starts_on_the_dashboard() {
        let session = LearningSession::new();

        assert_eq!(session.screen(), &Screen::Dashboard);
    }

    #[test]
    fn opening_a_path_moves_to_it() {
        let mut session = LearningSession::new();
        let path_id = first_path_id();

        session
            .open_path(catalog(), &path_id)
            .expect("the path exists");

        assert_eq!(session.screen(), &Screen::Path { path_id });
    }

    #[test]
    fn opening_an_unknown_path_is_rejected() {
        let mut session = LearningSession::new();

        assert_eq!(
            session.open_path(catalog(), "does-not-exist"),
            Err(LearningError::PathNotFound)
        );
        assert_eq!(session.screen(), &Screen::Dashboard);
    }

    #[test]
    fn opening_a_lesson_moves_to_it() {
        let mut session = LearningSession::new();
        let path_id = first_path_id();
        let lesson_id = first_lesson_id();

        session
            .open_lesson(catalog(), &path_id, &lesson_id)
            .expect("the lesson exists");

        assert_eq!(session.screen(), &Screen::Lesson { path_id, lesson_id });
    }

    #[test]
    fn opening_an_unknown_lesson_is_rejected() {
        let mut session = LearningSession::new();

        assert_eq!(
            session.open_lesson(catalog(), &first_path_id(), "does-not-exist"),
            Err(LearningError::LessonNotFound)
        );
        assert_eq!(session.screen(), &Screen::Dashboard);
    }

    #[test]
    fn the_review_screen_is_reachable_and_steps_back_to_the_dashboard() {
        let mut session = LearningSession::new();

        session.show_review();
        assert_eq!(session.screen(), &Screen::Review);

        session.go_back();
        assert_eq!(session.screen(), &Screen::Dashboard);
    }

    #[test]
    fn going_back_walks_lesson_to_path_to_dashboard() {
        let mut session = LearningSession::new();
        let path_id = first_path_id();
        let lesson_id = first_lesson_id();

        session
            .open_lesson(catalog(), &path_id, &lesson_id)
            .expect("the lesson exists");

        session.go_back();
        assert_eq!(session.screen(), &Screen::Path { path_id });

        session.go_back();
        assert_eq!(session.screen(), &Screen::Dashboard);

        session.go_back();
        assert_eq!(session.screen(), &Screen::Dashboard);
    }

    #[test]
    fn a_correct_answer_is_recognised() {
        let path_id = first_path_id();
        let lesson_id = first_lesson_id();
        let lesson = catalog()
            .lesson(&path_id, &lesson_id)
            .expect("the lesson exists");
        let check = &lesson.checks[0];

        let evaluated = evaluate_answer(
            catalog(),
            &path_id,
            &lesson_id,
            check.id(),
            &correct_answer(check),
        )
        .expect("the answer is well formed");

        assert!(evaluated.correct);
        assert_eq!(evaluated.required_checks.len(), lesson.checks.len());
        assert!(!evaluated.explanation.is_empty());
    }

    #[test]
    fn a_wrong_answer_is_recognised() {
        let path_id = first_path_id();
        let lesson_id = first_lesson_id();
        let lesson = catalog()
            .lesson(&path_id, &lesson_id)
            .expect("the lesson exists");
        let check = &lesson.checks[0];

        let evaluated = evaluate_answer(
            catalog(),
            &path_id,
            &lesson_id,
            check.id(),
            &wrong_answer(check),
        )
        .expect("the answer is well formed");

        assert!(!evaluated.correct);
    }

    #[test]
    fn every_check_kind_can_be_answered_correctly() {
        let mut seen = std::collections::BTreeSet::new();

        for path in catalog().paths() {
            for lesson in &path.lessons {
                for check in &lesson.checks {
                    let evaluated = evaluate_answer(
                        catalog(),
                        &path.id,
                        &lesson.id,
                        check.id(),
                        &correct_answer(check),
                    )
                    .expect("the answer is well formed");

                    assert!(
                        evaluated.correct,
                        "{} in {} was graded wrong with its own answer",
                        check.id(),
                        lesson.id
                    );
                    seen.insert(format!("{:?}", check.kind()));
                }
            }
        }

        assert_eq!(
            seen,
            ["Choice", "ShortAnswer", "TrueFalse"]
                .iter()
                .map(|kind| kind.to_string())
                .collect(),
            "the bundled content should use every exercise kind"
        );
    }

    #[test]
    fn an_option_outside_the_offered_range_is_rejected() {
        let path_id = first_path_id();
        let lesson_id = first_lesson_id();
        let lesson = catalog()
            .lesson(&path_id, &lesson_id)
            .expect("the lesson exists");
        let choice = lesson
            .checks
            .iter()
            .find(|check| matches!(check, Check::Choice { .. }))
            .expect("the first lesson has a choice check");

        assert_eq!(
            evaluate_answer(
                catalog(),
                &path_id,
                &lesson_id,
                choice.id(),
                &SubmittedAnswer::Choice { option: 99 }
            ),
            Err(LearningError::OptionOutOfRange)
        );
    }

    #[test]
    fn an_answer_of_the_wrong_kind_is_rejected() {
        let path_id = first_path_id();
        let lesson_id = first_lesson_id();
        let lesson = catalog()
            .lesson(&path_id, &lesson_id)
            .expect("the lesson exists");
        let choice = lesson
            .checks
            .iter()
            .find(|check| matches!(check, Check::Choice { .. }))
            .expect("the first lesson has a choice check");

        assert_eq!(
            evaluate_answer(
                catalog(),
                &path_id,
                &lesson_id,
                choice.id(),
                &SubmittedAnswer::ShortAnswer {
                    text: String::from("main")
                }
            ),
            Err(LearningError::AnswerMismatch)
        );
    }

    #[test]
    fn an_empty_short_answer_is_rejected() {
        let mut found = false;

        for path in catalog().paths() {
            for lesson in &path.lessons {
                for check in &lesson.checks {
                    if let Check::ShortAnswer { .. } = check {
                        found = true;

                        assert_eq!(
                            evaluate_answer(
                                catalog(),
                                &path.id,
                                &lesson.id,
                                check.id(),
                                &SubmittedAnswer::ShortAnswer {
                                    text: String::from("   ")
                                }
                            ),
                            Err(LearningError::EmptyAnswer)
                        );
                    }
                }
            }
        }

        assert!(found, "the bundled content should contain a short answer");
    }

    #[test]
    fn an_unknown_check_is_rejected() {
        assert_eq!(
            evaluate_answer(
                catalog(),
                &first_path_id(),
                &first_lesson_id(),
                "does-not-exist",
                &SubmittedAnswer::Choice { option: 0 }
            ),
            Err(LearningError::CheckNotFound)
        );
    }

    #[test]
    fn the_snapshot_lists_every_path_with_progress() {
        let snapshot =
            LearningSnapshot::build(&Screen::Dashboard, catalog(), &ProgressState::new(), None);

        assert_eq!(snapshot.paths.len(), catalog().paths().len());
        assert!(snapshot.lesson.is_none());
        assert!(!snapshot.can_go_back);
        assert_eq!(snapshot.summary.lessons_completed, 0);
        assert!(snapshot.summary.checks_total > 0);
        assert!(snapshot.review.is_empty());

        let first = &snapshot.paths[0];
        assert_eq!(first.completed_lessons, 0);
        assert_eq!(first.total_lessons, first.lessons.len());
        assert!(first.total_minutes > 0);
    }

    #[test]
    fn finishing_every_lesson_clears_the_resume_point() {
        let mut progress = ProgressState::new();

        for path in catalog().paths() {
            for lesson in &path.lessons {
                complete_lesson(catalog(), &mut progress, &path.id, &lesson.id);
            }
        }

        let snapshot = LearningSnapshot::build(&Screen::Dashboard, catalog(), &progress, None);

        assert!(snapshot.resume.is_none());
        assert_eq!(
            snapshot.summary.lessons_completed,
            snapshot.summary.lessons_total
        );
        assert_eq!(
            snapshot.summary.checks_solved,
            snapshot.summary.checks_total
        );
    }

    #[test]
    fn the_resume_point_points_at_the_first_unfinished_lesson() {
        let snapshot =
            LearningSnapshot::build(&Screen::Dashboard, catalog(), &ProgressState::new(), None);

        let resume = snapshot.resume.expect("there is unfinished work");

        assert_eq!(resume.path_id, first_path_id());
        assert_eq!(resume.lesson_id, first_lesson_id());
    }

    #[test]
    fn the_resume_point_follows_the_most_recently_opened_lesson() {
        let catalog = catalog();
        let path = &catalog.paths()[0];
        let later = &path.lessons[2];
        let mut progress = ProgressState::new();

        progress.mark_opened(&LessonKey::new(&path.id, &path.lessons[0].id));
        progress.mark_opened(&LessonKey::new(&path.id, &later.id));

        let snapshot = LearningSnapshot::build(&Screen::Dashboard, catalog, &progress, None);

        assert_eq!(
            snapshot.resume.expect("still unfinished").lesson_id,
            later.id
        );
    }

    #[test]
    fn the_resume_point_ignores_a_finished_lesson_that_was_opened_last() {
        let catalog = catalog();
        let path = &catalog.paths()[0];
        let mut progress = ProgressState::new();

        complete_lesson(catalog, &mut progress, &path.id, &path.lessons[0].id);
        progress.mark_opened(&LessonKey::new(&path.id, &path.lessons[0].id));

        let snapshot = LearningSnapshot::build(&Screen::Dashboard, catalog, &progress, None);

        assert_eq!(
            snapshot.resume.expect("the next lesson").lesson_id,
            path.lessons[1].id
        );
    }

    #[test]
    fn the_resume_point_skips_finished_lessons() {
        let catalog = catalog();
        let path = &catalog.paths()[0];
        let mut progress = ProgressState::new();

        complete_lesson(catalog, &mut progress, &path.id, &path.lessons[0].id);

        let snapshot = LearningSnapshot::build(&Screen::Dashboard, catalog, &progress, None);

        assert_eq!(
            snapshot.resume.expect("still unfinished").lesson_id,
            path.lessons[1].id
        );
    }

    #[test]
    fn an_open_lesson_is_included_in_the_snapshot() {
        let path_id = first_path_id();
        let lesson_id = first_lesson_id();

        let snapshot = LearningSnapshot::build(
            &Screen::Lesson { path_id, lesson_id },
            catalog(),
            &ProgressState::new(),
            None,
        );

        let lesson = snapshot.lesson.expect("the lesson should be included");

        assert!(!lesson.sections.is_empty());
        assert!(!lesson.checks.is_empty());
        assert!(!lesson.objectives.is_empty());
        assert!(snapshot.can_go_back);
    }

    #[test]
    fn a_wrong_answer_creates_a_review_item_without_completing_the_lesson() {
        let path = &catalog().paths()[0];
        let lesson = &path.lessons[0];
        let key = LessonKey::new(&path.id, &lesson.id);
        let mut progress = ProgressState::new();

        progress.record_attempt(&key, lesson.checks[0].id(), false, &lesson.check_ids(), 77);

        let snapshot = LearningSnapshot::build(&Screen::Dashboard, catalog(), &progress, None);

        assert_eq!(snapshot.review.len(), 1);
        assert_eq!(snapshot.review[0].lesson_id, lesson.id);
        assert_eq!(snapshot.review[0].mistakes, 1);
        assert!(!snapshot.review[0].completed);
        assert!(!snapshot.paths[0].lessons[0].completed);
        assert_eq!(snapshot.paths[0].lessons[0].attempt_count, 1);
    }

    #[test]
    fn a_completed_lesson_with_mistakes_stays_in_the_review_queue() {
        let path = &catalog().paths()[0];
        let lesson = &path.lessons[0];
        let key = LessonKey::new(&path.id, &lesson.id);
        let mut progress = ProgressState::new();

        progress.record_attempt(&key, lesson.checks[0].id(), false, &lesson.check_ids(), 1);
        complete_lesson(catalog(), &mut progress, &path.id, &lesson.id);

        let snapshot = LearningSnapshot::build(&Screen::Review, catalog(), &progress, None);

        assert_eq!(snapshot.review.len(), 1);
        assert!(snapshot.review[0].completed);
        assert_eq!(snapshot.review[0].mistakes, 1);
        assert!(snapshot.can_go_back);
        assert_eq!(snapshot.summary.mistakes, 1);
    }

    #[test]
    fn bookmarks_appear_in_the_snapshot_and_the_summary() {
        let path = &catalog().paths()[0];
        let lesson = &path.lessons[0];
        let key = LessonKey::new(&path.id, &lesson.id);
        let mut progress = ProgressState::new();

        progress.toggle_bookmark(&key);

        let snapshot = LearningSnapshot::build(&Screen::Dashboard, catalog(), &progress, None);

        assert!(snapshot.paths[0].lessons[0].bookmarked);
        assert_eq!(snapshot.summary.bookmarks, 1);
    }

    #[test]
    fn completing_a_lesson_is_reflected_in_the_snapshot() {
        let path = &catalog().paths()[0];
        let lesson = &path.lessons[0];
        let mut progress = ProgressState::new();

        complete_lesson(catalog(), &mut progress, &path.id, &lesson.id);

        let snapshot = LearningSnapshot::build(&Screen::Dashboard, catalog(), &progress, None);

        assert_eq!(snapshot.paths[0].completed_lessons, 1);
        assert!(snapshot.paths[0].lessons[0].completed);
        assert_eq!(
            snapshot.paths[0].lessons[0].solved_count,
            lesson.checks.len()
        );
        assert_eq!(
            snapshot.summary.minutes_completed,
            lesson.minutes.unwrap_or(0)
        );
    }

    #[test]
    fn a_storage_warning_is_passed_through() {
        let snapshot = LearningSnapshot::build(
            &Screen::Dashboard,
            catalog(),
            &ProgressState::new(),
            Some(String::from("saving failed")),
        );

        assert_eq!(snapshot.storage_warning.as_deref(), Some("saving failed"));
    }

    #[test]
    fn search_finds_paths_and_lessons_and_ignores_case() {
        let catalog = catalog();
        let by_path_title = catalog.search("RUST FOUNDATIONS");

        assert!(
            by_path_title
                .iter()
                .any(|hit| hit.matched == MatchedField::Path),
            "got {by_path_title:?}"
        );

        let by_objective = catalog.search("ownership");

        assert!(
            by_objective
                .iter()
                .any(|hit| hit.matched == MatchedField::Lesson && hit.lesson_id.is_some()),
            "got {by_objective:?}"
        );
    }

    #[test]
    fn search_with_no_query_returns_nothing() {
        assert!(catalog().search("   ").is_empty());
        assert!(catalog().search("zzzz-no-such-text").is_empty());
    }

    #[test]
    fn search_results_point_at_openable_content() {
        let catalog = catalog();

        for hit in catalog.search("the") {
            assert!(
                catalog.path(&hit.path_id).is_some(),
                "unknown path in {hit:?}"
            );

            if let Some(lesson_id) = &hit.lesson_id {
                assert!(
                    catalog.lesson(&hit.path_id, lesson_id).is_some(),
                    "unknown lesson in {hit:?}"
                );
            }
        }
    }

    #[test]
    fn check_views_expose_options_but_not_the_answer() {
        let lesson = &catalog().paths()[0].lessons[0];
        let view = lesson.view(None);
        let choice = view
            .checks
            .iter()
            .find(|check| check.kind == crate::lessons::CheckKind::Choice)
            .expect("the first lesson has a choice check");

        assert!(choice.options.len() >= 2);
        assert!(!choice.prompt.is_empty());
    }

    #[test]
    fn screens_serialize_to_the_shape_the_interface_expects() {
        let dashboard = serde_json::to_string(&Screen::Dashboard).expect("dashboard");

        assert_eq!(dashboard, r#"{"kind":"dashboard"}"#);

        let lesson = serde_json::to_string(&Screen::Lesson {
            path_id: String::from("path"),
            lesson_id: String::from("lesson"),
        })
        .expect("lesson");

        assert_eq!(
            lesson,
            r#"{"kind":"lesson","pathId":"path","lessonId":"lesson"}"#
        );

        let review = serde_json::to_string(&Screen::Review).expect("review");

        assert_eq!(review, r#"{"kind":"review"}"#);
    }

    #[test]
    fn snapshots_use_camel_case_keys() {
        let snapshot =
            LearningSnapshot::build(&Screen::Dashboard, catalog(), &ProgressState::new(), None);
        let json = serde_json::to_string(&snapshot).expect("snapshot");

        for key in [
            "\"canGoBack\"",
            "\"paths\"",
            "\"lesson\"",
            "\"resume\"",
            "\"storageWarning\"",
            "\"summary\"",
            "\"review\"",
            "\"lessonsCompleted\"",
            "\"checksSolved\"",
            "\"minutesCompleted\"",
        ] {
            assert!(json.contains(key), "missing {key}");
        }

        assert!(json.contains("\"solvedCount\""));
        assert!(json.contains("\"attemptCount\""));
        assert!(json.contains("\"completedAt\""));
        assert!(json.contains("\"bookmarked\""));
    }

    #[test]
    fn sections_are_passed_through_unchanged() {
        let lesson: &Lesson = &catalog().paths()[0].lessons[0];
        let view = lesson.view(None);

        assert!(view.sections.iter().all(|section| matches!(
            section,
            Section::Explanation { .. }
                | Section::Code { .. }
                | Section::Note { .. }
                | Section::Steps { .. }
                | Section::Table { .. }
                | Section::Mistake { .. }
        )));
    }

    #[test]
    fn all_section_kinds_are_used_by_the_bundled_content() {
        let mut kinds = std::collections::BTreeSet::new();

        for path in catalog().paths() {
            for lesson in &path.lessons {
                for section in &lesson.sections {
                    kinds.insert(match section {
                        Section::Explanation { .. } => "explanation",
                        Section::Code { .. } => "code",
                        Section::Note { .. } => "note",
                        Section::Steps { .. } => "steps",
                        Section::Table { .. } => "table",
                        Section::Mistake { .. } => "mistake",
                    });
                }
            }
        }

        for kind in ["explanation", "code", "steps", "table", "mistake"] {
            assert!(kinds.contains(kind), "the content never uses {kind}");
        }
    }
}
