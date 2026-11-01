//! The learning catalog and the learner's place inside it.
//!
//! The catalog is loaded from editable content files that are compiled into the
//! application, so there is no content to fetch or download at startup. Every
//! request to open a path, open a lesson or answer a check is validated here,
//! and the interface renders the snapshot it is given.

use std::sync::OnceLock;

use serde::{Serialize, Serializer};

use crate::lessons::{self, ContentProblem, Difficulty, Lesson, LessonView, Path};
use crate::progress::{LessonKey, ProgressState};

/// The bundled content files. Adding a pack means adding a line here, and its
/// content is validated by the tests in this module.
const EMBEDDED_PACKS: &[&str] = &[include_str!(
    "../../content/programming/rust/foundations.json"
)];

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
    pub total_minutes: u32,
    pub completed_lessons: usize,
    pub total_lessons: usize,
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
    pub answered_count: usize,
    pub completed: bool,
    /// When the lesson was finished, if it has been.
    pub completed_at: Option<u64>,
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
    /// The next unfinished lesson, when there is one.
    pub resume: Option<ResumePoint>,
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

        let lesson = open_lesson_view(screen, catalog);
        let resume = resume_point(catalog, progress);

        Self {
            can_go_back: !matches!(screen, Screen::Dashboard),
            screen: screen.clone(),
            paths,
            lesson,
            resume,
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
                answered_count: record.map_or(0, |record| record.answered_count()),
                completed: progress.is_complete(&key),
                completed_at: record.and_then(crate::progress::LessonRecord::completed_at),
            }
        })
        .collect();

    PathView {
        id: path.id.clone(),
        title: path.title.clone(),
        summary: path.summary.clone(),
        subject: path.subject.clone(),
        difficulty: path.difficulty,
        total_minutes: path.total_minutes(),
        completed_lessons: lessons.iter().filter(|lesson| lesson.completed).count(),
        total_lessons: lessons.len(),
        lessons,
    }
}

fn open_lesson_view(screen: &Screen, catalog: &Catalog) -> Option<LessonView> {
    match screen {
        Screen::Lesson { path_id, lesson_id } => catalog
            .lesson(path_id, lesson_id)
            .map(lessons::Lesson::view),
        _ => None,
    }
}

fn resume_point(catalog: &Catalog, progress: &ProgressState) -> Option<ResumePoint> {
    for path in catalog.paths() {
        for lesson in &path.lessons {
            if !progress.is_complete(&LessonKey::new(&path.id, &lesson.id)) {
                return Some(ResumePoint {
                    path_id: path.id.clone(),
                    path_title: path.title.clone(),
                    lesson_id: lesson.id.clone(),
                    lesson_title: lesson.title.clone(),
                });
            }
        }
    }

    None
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
/// This is the only place an answer index is compared, so the correct option
/// never has to leave the backend.
pub fn evaluate_answer(
    catalog: &Catalog,
    path_id: &str,
    lesson_id: &str,
    check_id: &str,
    option: usize,
) -> Result<EvaluatedAnswer, LearningError> {
    let lesson = catalog
        .lesson(path_id, lesson_id)
        .ok_or(LearningError::LessonNotFound)?;
    let check = lesson.check(check_id).ok_or(LearningError::CheckNotFound)?;

    if option >= check.options.len() {
        return Err(LearningError::OptionOutOfRange);
    }

    Ok(EvaluatedAnswer {
        correct: check.is_correct(option),
        explanation: check.explanation.clone(),
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

    /// Steps back one screen: a lesson returns to its path, a path to the
    /// dashboard. The dashboard stays put.
    pub fn go_back(&mut self) {
        self.screen = match &self.screen {
            Screen::Dashboard => Screen::Dashboard,
            Screen::Path { .. } => Screen::Dashboard,
            Screen::Lesson { path_id, .. } => Screen::Path {
                path_id: path_id.clone(),
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lessons::{CheckView, Section};
    use crate::progress::AnswerEffect;

    fn catalog() -> &'static Catalog {
        Catalog::embedded().expect("the bundled content should be valid")
    }

    fn first_path_id() -> String {
        catalog().paths()[0].id.clone()
    }

    fn first_lesson_id() -> String {
        catalog().paths()[0].lessons[0].id.clone()
    }

    #[test]
    fn bundled_content_loads() {
        assert!(!catalog().paths().is_empty());
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

        let evaluated = evaluate_answer(catalog(), &path_id, &lesson_id, &check.id, check.answer())
            .expect("the answer is offered");

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
        let wrong = (check.answer() + 1) % check.options.len();

        let evaluated = evaluate_answer(catalog(), &path_id, &lesson_id, &check.id, wrong)
            .expect("the answer is offered");

        assert!(!evaluated.correct);
    }

    #[test]
    fn an_option_outside_the_offered_range_is_rejected() {
        let path_id = first_path_id();
        let lesson_id = first_lesson_id();
        let check_id = catalog()
            .lesson(&path_id, &lesson_id)
            .expect("the lesson exists")
            .checks[0]
            .id
            .clone();

        assert_eq!(
            evaluate_answer(catalog(), &path_id, &lesson_id, &check_id, 99),
            Err(LearningError::OptionOutOfRange)
        );
    }

    #[test]
    fn an_unknown_check_is_rejected() {
        assert_eq!(
            evaluate_answer(
                catalog(),
                &first_path_id(),
                &first_lesson_id(),
                "does-not-exist",
                0
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

        let first = &snapshot.paths[0];
        assert_eq!(first.completed_lessons, 0);
        assert_eq!(first.total_lessons, first.lessons.len());
        assert!(first.total_minutes > 0);
    }

    #[test]
    fn finishing_every_lesson_clears_the_resume_point() {
        let mut progress = ProgressState::new();
        let path = &catalog().paths()[0];

        for lesson in &path.lessons {
            let key = LessonKey::new(&path.id, &lesson.id);

            for check in &lesson.checks {
                progress.record_correct(&key, &check.id, &lesson.check_ids(), 1);
            }
        }

        let snapshot = LearningSnapshot::build(&Screen::Dashboard, catalog(), &progress, None);

        assert!(snapshot.resume.is_none());
        assert_eq!(
            snapshot.paths[0].completed_lessons,
            snapshot.paths[0].total_lessons
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
    fn the_resume_point_skips_finished_lessons() {
        let path = &catalog().paths()[0];
        let key = LessonKey::new(&path.id, &path.lessons[0].id);
        let mut progress = ProgressState::new();

        for check in &path.lessons[0].checks {
            progress.record_correct(&key, &check.id, &path.lessons[0].check_ids(), 1);
        }

        let snapshot = LearningSnapshot::build(&Screen::Dashboard, catalog(), &progress, None);

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
        assert!(snapshot.can_go_back);
    }

    #[test]
    fn completing_a_lesson_is_reflected_in_the_snapshot() {
        let path = &catalog().paths()[0];
        let lesson = &path.lessons[0];
        let key = LessonKey::new(&path.id, &lesson.id);
        let mut progress = ProgressState::new();

        let mut effect = AnswerEffect::Recorded;
        for check in &lesson.checks {
            effect = progress.record_correct(&key, &check.id, &lesson.check_ids(), 42);
        }

        assert_eq!(effect, AnswerEffect::LessonCompleted);

        let snapshot = LearningSnapshot::build(&Screen::Dashboard, catalog(), &progress, None);

        assert_eq!(snapshot.paths[0].completed_lessons, 1);
        assert!(snapshot.paths[0].lessons[0].completed);
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
    fn check_views_expose_options_but_not_the_answer() {
        let lesson = &catalog().paths()[0].lessons[0];
        let view: &CheckView = &lesson.view().checks[0];

        assert_eq!(view.options.len(), lesson.checks[0].options.len());
        assert!(!view.prompt.is_empty());
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
        ] {
            assert!(json.contains(key), "missing {key} in {json}");
        }

        assert!(json.contains("\"completedLessons\""));
        assert!(json.contains("\"answeredCount\""));
        assert!(json.contains("\"completedAt\""));
    }

    #[test]
    fn sections_are_passed_through_unchanged() {
        let lesson: &Lesson = &catalog().paths()[0].lessons[0];
        let view = lesson.view();

        assert!(view.sections.iter().all(|section| matches!(
            section,
            Section::Explanation { .. } | Section::Code { .. } | Section::Note { .. }
        )));
    }
}
