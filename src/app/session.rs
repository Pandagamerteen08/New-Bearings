//! The application session.
//!
//! One value holds the authoritative state for the whole application and is
//! shared with every Tauri command. Commands validate and delegate here; this
//! is the only place that changes state or writes it to disk.

use std::sync::{Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use crate::learning::{Catalog, LearningError, LearningSession, LearningSnapshot, evaluate_answer};
use crate::progress::{AnswerEffect, LessonKey, ProgressState};
use crate::setup::{SetupError, SetupSnapshot, SetupState};
use crate::storage::{AppData, AppStorage, SCHEMA_VERSION};

/// What happened when a learner answered a knowledge check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnswerOutcome {
    pub correct: bool,
    pub explanation: String,
    /// True only for the answer that finished the lesson.
    pub lesson_completed: bool,
    pub snapshot: LearningSnapshot,
}

#[derive(Debug)]
struct Inner {
    setup: SetupState,
    learning: LearningSession,
    progress: ProgressState,
    storage: AppStorage,
    /// Why saved data could not be used at startup, if it could not.
    recovery_notice: Option<String>,
    /// Set when the most recent save failed, cleared by a successful one.
    save_warning: Option<String>,
}

impl Inner {
    fn warning(&self) -> Option<String> {
        match (&self.save_warning, &self.recovery_notice) {
            (Some(save), Some(recovery)) => Some(format!("{save} {recovery}")),
            (Some(save), None) => Some(save.clone()),
            (None, Some(recovery)) => Some(recovery.clone()),
            (None, None) => None,
        }
    }

    /// Writes the current state to disk, remembering any failure so the
    /// interface can tell the learner instead of losing data silently.
    fn persist(&mut self) {
        let data = AppData {
            version: SCHEMA_VERSION,
            setup_stage: self.setup.stage(),
            progress: self.progress.clone(),
        };

        match self.storage.save(&data) {
            Ok(()) => self.save_warning = None,
            Err(error) => {
                // The technical reason belongs in the developer log, not in the
                // interface, which must not display filesystem paths.
                eprintln!(
                    "new-bearings: could not save {}: {error}",
                    self.storage.path().display()
                );
                self.save_warning = Some(String::from(
                    "New Bearings could not save your progress to this computer.",
                ));
            }
        }
    }
}

/// The shared application state.
#[derive(Debug)]
pub struct Session {
    inner: Mutex<Inner>,
}

impl Session {
    /// Loads saved data from `storage` and starts a session with it.
    ///
    /// If saved data cannot be used, the session still starts: the learner is
    /// told what happened through the warning on its snapshots.
    pub fn open(storage: AppStorage) -> Self {
        let loaded = storage.load();

        let inner = Inner {
            setup: SetupState::from_stage(loaded.data.setup_stage),
            learning: LearningSession::new(),
            progress: loaded.data.progress,
            storage,
            recovery_notice: loaded.notice,
            save_warning: None,
        };

        Self {
            inner: Mutex::new(inner),
        }
    }

    /// Describes the current setup stage.
    pub fn setup_snapshot(&self) -> SetupSnapshot {
        self.lock().setup.snapshot()
    }

    /// Advances from the welcome screen to the introduction.
    pub fn begin_setup(&self) -> Result<SetupSnapshot, SetupError> {
        let mut inner = self.lock();
        inner.setup.begin()?;
        inner.persist();

        Ok(inner.setup.snapshot())
    }

    /// Returns from the introduction to the welcome screen.
    pub fn return_to_welcome(&self) -> Result<SetupSnapshot, SetupError> {
        let mut inner = self.lock();
        inner.setup.return_to_welcome()?;
        inner.persist();

        Ok(inner.setup.snapshot())
    }

    /// Leaves the introduction and enters the learning screens.
    pub fn start_learning(&self) -> Result<SetupSnapshot, SetupError> {
        let mut inner = self.lock();
        inner.setup.start_learning()?;
        inner.learning.show_dashboard();
        inner.persist();

        Ok(inner.setup.snapshot())
    }

    /// Describes the learning screens as they currently are.
    pub fn learning_snapshot(&self) -> Result<LearningSnapshot, LearningError> {
        let inner = self.lock();

        Self::snapshot_of(&inner)
    }

    /// Opens a learning path.
    pub fn open_path(&self, path_id: &str) -> Result<LearningSnapshot, LearningError> {
        let catalog = catalog()?;
        let mut inner = self.lock();

        inner.learning.open_path(catalog, path_id)?;

        Self::snapshot_of(&inner)
    }

    /// Opens a lesson inside a learning path.
    pub fn open_lesson(
        &self,
        path_id: &str,
        lesson_id: &str,
    ) -> Result<LearningSnapshot, LearningError> {
        let catalog = catalog()?;
        let mut inner = self.lock();

        inner.learning.open_lesson(catalog, path_id, lesson_id)?;

        Self::snapshot_of(&inner)
    }

    /// Steps back one learning screen.
    pub fn go_back(&self) -> Result<LearningSnapshot, LearningError> {
        let mut inner = self.lock();

        inner.learning.go_back();

        Self::snapshot_of(&inner)
    }

    /// Judges an answer, records it when correct, and saves.
    pub fn submit_answer(
        &self,
        path_id: &str,
        lesson_id: &str,
        check_id: &str,
        option: usize,
    ) -> Result<AnswerOutcome, LearningError> {
        let catalog = catalog()?;
        let evaluated = evaluate_answer(catalog, path_id, lesson_id, check_id, option)?;
        let mut inner = self.lock();

        let mut lesson_completed = false;

        if evaluated.correct {
            let key = LessonKey::new(path_id, lesson_id);
            let effect = inner.progress.record_correct(
                &key,
                check_id,
                &evaluated.required_checks,
                unix_seconds(),
            );

            lesson_completed = effect == AnswerEffect::LessonCompleted;
            inner.persist();
        }

        let snapshot = Self::snapshot_of(&inner)?;

        Ok(AnswerOutcome {
            correct: evaluated.correct,
            explanation: evaluated.explanation,
            lesson_completed,
            snapshot,
        })
    }

    fn snapshot_of(inner: &Inner) -> Result<LearningSnapshot, LearningError> {
        let catalog = catalog()?;

        Ok(LearningSnapshot::build(
            inner.learning.screen(),
            catalog,
            &inner.progress,
            inner.warning(),
        ))
    }

    /// Locks the state, recovering from a poisoned lock.
    ///
    /// Every field is replaced as a whole, so a panic during an update cannot
    /// leave a partially updated value behind.
    fn lock(&self) -> MutexGuard<'_, Inner> {
        self.inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

fn catalog() -> Result<&'static Catalog, LearningError> {
    Catalog::embedded().map_err(|problems| {
        // Content is validated by tests, so this is a build defect rather than
        // something the learner caused. Record the details for the developer and
        // let the interface show a plain message.
        for problem in problems {
            eprintln!(
                "new-bearings: invalid content at {}: {}",
                problem.location(),
                problem.message()
            );
        }

        LearningError::ContentInvalid
    })
}

fn unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::learning::Screen;
    use crate::setup::SetupStage;

    struct TempDirectory {
        path: std::path::PathBuf,
    }

    impl TempDirectory {
        fn new(name: &str) -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|elapsed| elapsed.as_nanos())
                .unwrap_or(0);
            let path = std::env::temp_dir().join(format!(
                "new-bearings-session-{name}-{}-{unique}",
                std::process::id()
            ));

            std::fs::create_dir_all(&path).expect("test directory should be created");

            Self { path }
        }

        fn storage(&self) -> AppStorage {
            AppStorage::new(&self.path)
        }
    }

    impl Drop for TempDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }

    fn session(directory: &TempDirectory) -> Session {
        Session::open(directory.storage())
    }

    fn first_path_and_lesson() -> (&'static str, &'static str) {
        let catalog = Catalog::embedded().expect("content should load");
        let path = &catalog.paths()[0];

        (&path.id, &path.lessons[0].id)
    }

    #[test]
    fn a_new_session_starts_at_the_welcome_screen() {
        let directory = TempDirectory::new("welcome");
        let session = session(&directory);

        assert_eq!(session.setup_snapshot().stage, SetupStage::Welcome);
        assert!(session.setup_snapshot().can_begin);
    }

    #[test]
    fn setup_transitions_are_applied_and_reported() {
        let directory = TempDirectory::new("transitions");
        let session = session(&directory);

        let introduction = session.begin_setup().expect("introduction");
        assert_eq!(introduction.stage, SetupStage::Introduction);
        assert!(introduction.can_start_learning);

        let learning = session.start_learning().expect("learning");
        assert_eq!(learning.stage, SetupStage::Learning);
        assert!(!learning.can_start_learning);
    }

    #[test]
    fn reaching_learning_opens_the_dashboard() {
        let directory = TempDirectory::new("dashboard");
        let session = session(&directory);

        session.begin_setup().expect("introduction");
        session.start_learning().expect("learning");

        let snapshot = session.learning_snapshot().expect("snapshot");

        assert_eq!(snapshot.screen, Screen::Dashboard);
        assert!(!snapshot.can_go_back);
        assert!(snapshot.resume.is_some());
    }

    #[test]
    fn setup_progress_survives_a_restart() {
        let directory = TempDirectory::new("restart");

        {
            let session = session(&directory);
            session.begin_setup().expect("introduction");
            session.start_learning().expect("learning");
        }

        let reopened = session(&directory);

        assert_eq!(reopened.setup_snapshot().stage, SetupStage::Learning);
    }

    #[test]
    fn opening_a_lesson_is_reflected_in_the_snapshot() {
        let directory = TempDirectory::new("open-lesson");
        let session = session(&directory);
        let (path_id, lesson_id) = first_path_and_lesson();

        session
            .open_lesson(path_id, lesson_id)
            .expect("lesson opens");

        let snapshot = session.learning_snapshot().expect("snapshot");

        assert_eq!(
            snapshot.screen,
            Screen::Lesson {
                path_id: path_id.to_owned(),
                lesson_id: lesson_id.to_owned()
            }
        );
        assert!(snapshot.lesson.is_some());
        assert!(snapshot.can_go_back);
    }

    #[test]
    fn a_wrong_answer_is_not_recorded() {
        let directory = TempDirectory::new("wrong");
        let session = session(&directory);
        let (path_id, lesson_id) = first_path_and_lesson();
        let catalog = Catalog::embedded().expect("content");
        let check = &catalog.lesson(path_id, lesson_id).expect("lesson").checks[0];
        let wrong = (check.answer() + 1) % check.options.len();

        let outcome = session
            .submit_answer(path_id, lesson_id, &check.id, wrong)
            .expect("answer is accepted");

        assert!(!outcome.correct);
        assert!(!outcome.lesson_completed);
        assert!(!outcome.explanation.is_empty());
        assert_eq!(
            outcome.snapshot.paths[0].lessons[0].answered_count, 0,
            "an incorrect answer must not count as progress"
        );
    }

    #[test]
    fn correct_answers_complete_a_lesson_and_persist() {
        let directory = TempDirectory::new("complete");
        let (path_id, lesson_id) = first_path_and_lesson();
        let catalog = Catalog::embedded().expect("content");
        let lesson = catalog.lesson(path_id, lesson_id).expect("lesson").clone();

        {
            let session = session(&directory);
            session
                .open_lesson(path_id, lesson_id)
                .expect("lesson opens");

            for (index, check) in lesson.checks.iter().enumerate() {
                let outcome = session
                    .submit_answer(path_id, lesson_id, &check.id, check.answer())
                    .expect("answer is accepted");

                assert!(outcome.correct);
                assert_eq!(
                    outcome.lesson_completed,
                    index + 1 == lesson.checks.len(),
                    "only the last answer finishes the lesson"
                );
            }
        }

        let reopened = session(&directory);
        let snapshot = reopened.learning_snapshot().expect("snapshot");

        assert!(snapshot.paths[0].lessons[0].completed);
        assert_eq!(snapshot.paths[0].completed_lessons, 1);
        assert!(snapshot.storage_warning.is_none());
    }

    #[test]
    fn going_back_walks_out_of_a_lesson() {
        let directory = TempDirectory::new("back");
        let session = session(&directory);
        let (path_id, lesson_id) = first_path_and_lesson();

        session
            .open_lesson(path_id, lesson_id)
            .expect("lesson opens");
        session.go_back().expect("back to path");
        assert_eq!(
            session.learning_snapshot().expect("snapshot").screen,
            Screen::Path {
                path_id: path_id.to_owned()
            }
        );

        session.go_back().expect("back to dashboard");
        assert_eq!(
            session.learning_snapshot().expect("snapshot").screen,
            Screen::Dashboard
        );
    }

    #[test]
    fn unknown_lessons_are_rejected() {
        let directory = TempDirectory::new("unknown");
        let session = session(&directory);
        let (path_id, _) = first_path_and_lesson();

        assert_eq!(
            session.open_lesson(path_id, "does-not-exist"),
            Err(LearningError::LessonNotFound)
        );
    }

    #[test]
    fn progress_that_cannot_be_saved_is_reported() {
        let directory = TempDirectory::new("unsaved");
        let blocker = directory.path.join("blocker");
        std::fs::write(&blocker, b"not a directory").expect("blocker should be written");

        // Pointing storage below a file makes every save fail.
        let session = Session::open(AppStorage::new(blocker.join("data")));
        let (path_id, lesson_id) = first_path_and_lesson();
        let catalog = Catalog::embedded().expect("content");
        let check = &catalog.lesson(path_id, lesson_id).expect("lesson").checks[0];

        let outcome = session
            .submit_answer(path_id, lesson_id, &check.id, check.answer())
            .expect("answer is accepted");

        assert!(
            outcome.snapshot.storage_warning.is_some(),
            "the learner must be told that progress was not saved"
        );
    }

    #[test]
    fn a_recovered_file_is_reported_once() {
        let directory = TempDirectory::new("recovered");
        let storage = directory.storage();
        std::fs::write(storage.path(), b"not json").expect("bad file should be written");

        let session = Session::open(storage);
        let snapshot = session.learning_snapshot().expect("snapshot");

        assert!(snapshot.storage_warning.is_some());
    }
}
