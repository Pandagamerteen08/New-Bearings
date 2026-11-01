//! Local persistence for the learner's setup stage, progress and bookmarks.
//!
//! The data lives in one versioned JSON file inside the application data
//! directory. Writes go to a temporary file that is then renamed over the real
//! one, so an interrupted save cannot leave a half-written file behind.
//!
//! Saved data is never deleted silently: a file written by an older format is
//! migrated in place, and a file that cannot be used at all is moved aside and
//! the learner is told what happened.

use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::progress::{LegacyLessonRecord, ProgressState};
use crate::setup::SetupStage;

/// The version of the saved file format that this build writes and reads.
pub const SCHEMA_VERSION: u32 = 2;

/// The file name used inside the application data directory.
pub const STATE_FILE_NAME: &str = "app-state.json";

/// Everything that is remembered between runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppData {
    pub version: u32,
    #[serde(default)]
    pub setup_stage: SetupStage,
    #[serde(default)]
    pub progress: ProgressState,
}

impl AppData {
    /// Creates the data a first run starts with.
    pub fn fresh() -> Self {
        Self {
            version: SCHEMA_VERSION,
            setup_stage: SetupStage::Welcome,
            progress: ProgressState::new(),
        }
    }
}

/// The result of reading saved data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadOutcome {
    /// The data to use. Saved data that could not be used is replaced by a
    /// fresh value, and the reason is reported in `notice`.
    pub data: AppData,
    /// A learner-facing explanation, set when saved data could not be used.
    pub notice: Option<String>,
}

/// Describes why saved data could not be written or read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageError {
    message: String,
}

impl StorageError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for StorageError {}

/// Just enough of the file to decide how to read the rest of it.
#[derive(Debug, Deserialize)]
struct VersionProbe {
    version: u32,
}

/// The saved file shape written by format version 1.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct V1AppData {
    #[serde(default)]
    setup_stage: SetupStage,
    #[serde(default)]
    progress: BTreeMap<String, LegacyLessonRecord>,
}

/// Reads and writes the application data file.
#[derive(Debug, Clone)]
pub struct AppStorage {
    path: PathBuf,
}

impl AppStorage {
    /// Points storage at an application data directory.
    pub fn new(directory: impl Into<PathBuf>) -> Self {
        Self {
            path: directory.into().join(STATE_FILE_NAME),
        }
    }

    /// The file that holds the learner's data.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Reads saved data, falling back to a fresh value when nothing usable is
    /// available.
    pub fn load(&self) -> LoadOutcome {
        if !self.path.exists() {
            return LoadOutcome {
                data: AppData::fresh(),
                notice: None,
            };
        }

        let raw = match fs::read_to_string(&self.path) {
            Ok(raw) => raw,
            Err(error) => {
                return LoadOutcome {
                    data: AppData::fresh(),
                    notice: Some(format!(
                        "Your saved progress could not be read ({error}). New progress will be saved from now on."
                    )),
                };
            }
        };

        if raw.trim().is_empty() {
            return LoadOutcome {
                data: AppData::fresh(),
                notice: None,
            };
        }

        let version = match serde_json::from_str::<VersionProbe>(&raw) {
            Ok(probe) => probe.version,
            Err(error) => {
                return self.recover(format!(
                    "Your saved progress could not be understood ({error}), so New Bearings started fresh."
                ));
            }
        };

        match version {
            SCHEMA_VERSION => match serde_json::from_str::<AppData>(&raw) {
                Ok(data) => LoadOutcome { data, notice: None },
                Err(error) => self.recover(format!(
                    "Your saved progress could not be understood ({error}), so New Bearings started fresh."
                )),
            },
            1 => self.migrate_from_v1(&raw),
            other => self.recover(format!(
                "Your saved progress was written by a different version of New Bearings (format {other}), so it could not be used."
            )),
        }
    }

    /// Writes the data, replacing the previous file only once the new one is
    /// complete.
    pub fn save(&self, data: &AppData) -> Result<(), StorageError> {
        let directory = self.path.parent().ok_or_else(|| {
            StorageError::new("the application data directory could not be determined")
        })?;

        fs::create_dir_all(directory).map_err(|error| {
            StorageError::new(format!(
                "the application data directory could not be created: {error}"
            ))
        })?;

        let bytes = serde_json::to_vec_pretty(data).map_err(|error| {
            StorageError::new(format!("progress could not be encoded: {error}"))
        })?;

        let temporary = self.path.with_file_name(format!("{STATE_FILE_NAME}.tmp"));

        let write_result = (|| -> std::io::Result<()> {
            let mut file = fs::File::create(&temporary)?;
            file.write_all(&bytes)?;
            file.sync_all()
        })();

        if let Err(error) = write_result {
            let _ = fs::remove_file(&temporary);
            return Err(StorageError::new(format!(
                "progress could not be written: {error}"
            )));
        }

        if let Err(error) = fs::rename(&temporary, &self.path) {
            let _ = fs::remove_file(&temporary);
            return Err(StorageError::new(format!(
                "progress could not be saved: {error}"
            )));
        }

        Ok(())
    }

    /// Converts a version 1 file to the current shape.
    ///
    /// The migrated data is written straight back, so the conversion happens
    /// once. If that write fails the data is still usable for this run and the
    /// next save retries it, which is why the failure is not reported as data
    /// loss.
    fn migrate_from_v1(&self, raw: &str) -> LoadOutcome {
        let legacy: V1AppData = match serde_json::from_str(raw) {
            Ok(legacy) => legacy,
            Err(error) => {
                return self.recover(format!(
                    "Your saved progress could not be understood ({error}), so New Bearings started fresh."
                ));
            }
        };

        let data = AppData {
            version: SCHEMA_VERSION,
            setup_stage: legacy.setup_stage,
            progress: ProgressState::from_legacy(legacy.progress),
        };

        let _ = self.save(&data);

        LoadOutcome { data, notice: None }
    }

    /// Moves an unusable file aside so nothing is destroyed, and explains why.
    fn recover(&self, reason: String) -> LoadOutcome {
        let backup = self
            .path
            .with_file_name(format!("app-state.recovered-{}.json", unix_seconds()));

        let detail = if fs::rename(&self.path, &backup).is_ok() {
            format!(
                " The previous file was kept as {}.",
                backup
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| String::from("a backup file"))
            )
        } else {
            String::new()
        };

        LoadOutcome {
            data: AppData::fresh(),
            notice: Some(format!("{reason}{detail}")),
        }
    }
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
    use crate::progress::LessonKey;

    struct TempDirectory {
        path: PathBuf,
    }

    impl TempDirectory {
        fn new(name: &str) -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|elapsed| elapsed.as_nanos())
                .unwrap_or(0);
            let path = std::env::temp_dir().join(format!(
                "new-bearings-{name}-{}-{unique}",
                std::process::id()
            ));

            fs::create_dir_all(&path).expect("test directory should be created");

            Self { path }
        }

        fn storage(&self) -> AppStorage {
            AppStorage::new(&self.path)
        }
    }

    impl Drop for TempDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn sample_data() -> AppData {
        let mut data = AppData::fresh();
        data.setup_stage = SetupStage::Learning;
        data.progress.record_attempt(
            &LessonKey::new("rust-foundations", "first-program"),
            "entry-point",
            true,
            &["entry-point".to_owned(), "semicolon".to_owned()],
            1_700_000_000,
        );
        data.progress
            .mark_opened(&LessonKey::new("rust-foundations", "variables"));
        data.progress
            .toggle_bookmark(&LessonKey::new("rust-foundations", "functions"));

        data
    }

    #[test]
    fn missing_file_yields_fresh_data_without_a_notice() {
        let directory = TempDirectory::new("missing");

        let outcome = directory.storage().load();

        assert_eq!(outcome.data, AppData::fresh());
        assert!(outcome.notice.is_none());
    }

    #[test]
    fn saved_data_round_trips() {
        let directory = TempDirectory::new("round-trip");
        let storage = directory.storage();
        let data = sample_data();

        storage.save(&data).expect("save should succeed");
        let outcome = storage.load();

        assert_eq!(outcome.data, data);
        assert!(outcome.notice.is_none());
    }

    #[test]
    fn saving_creates_missing_directories() {
        let directory = TempDirectory::new("nested");
        let storage = AppStorage::new(directory.path.join("first").join("second"));

        storage
            .save(&AppData::fresh())
            .expect("save should succeed");

        assert!(storage.path().exists());
    }

    #[test]
    fn a_successful_save_leaves_no_temporary_file() {
        let directory = TempDirectory::new("temporary");
        let storage = directory.storage();

        storage.save(&sample_data()).expect("save should succeed");

        let leftovers: Vec<_> = fs::read_dir(&directory.path)
            .expect("directory should be readable")
            .filter_map(Result::ok)
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .filter(|name| name.ends_with(".tmp"))
            .collect();

        assert!(leftovers.is_empty(), "found {leftovers:?}");
    }

    #[test]
    fn corrupt_data_is_kept_and_explained() {
        let directory = TempDirectory::new("corrupt");
        let storage = directory.storage();
        fs::write(storage.path(), b"{ this is not json").expect("bad file should be written");

        let outcome = storage.load();

        assert_eq!(outcome.data, AppData::fresh());
        assert!(outcome.notice.is_some());

        let kept: Vec<_> = fs::read_dir(&directory.path)
            .expect("directory should be readable")
            .filter_map(Result::ok)
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .filter(|name| name.starts_with("app-state.recovered-"))
            .collect();

        assert_eq!(kept.len(), 1, "the unusable file should be kept aside");
    }

    #[test]
    fn a_version_one_file_is_migrated_and_rewritten() {
        let directory = TempDirectory::new("migrate");
        let storage = directory.storage();

        fs::write(
            storage.path(),
            br#"{
                "version": 1,
                "setupStage": "learning",
                "progress": {
                    "rust-foundations/first-program": {
                        "correctChecks": ["entry-point", "semicolon"],
                        "completedAt": 1700000000
                    },
                    "rust-foundations/variables": {
                        "correctChecks": ["mut-binding"]
                    }
                }
            }"#,
        )
        .expect("version 1 file should be written");

        let outcome = storage.load();

        assert!(outcome.notice.is_none(), "migrating is not a failure");

        let complete = LessonKey::new("rust-foundations", "first-program");
        let partial = LessonKey::new("rust-foundations", "variables");

        assert!(outcome.data.progress.is_complete(&complete));
        assert!(!outcome.data.progress.is_complete(&partial));
        assert_eq!(
            outcome
                .data
                .progress
                .lesson(&complete)
                .and_then(|record| record.completed_at()),
            Some(1_700_000_000)
        );
        assert_eq!(
            outcome
                .data
                .progress
                .lesson(&partial)
                .map(|record| record.solved_count()),
            Some(1)
        );

        // The migrated file is written in the current format, so the conversion
        // happens once.
        let rewritten = fs::read_to_string(storage.path()).expect("file should be readable");
        let stored: AppData = serde_json::from_str(&rewritten).expect("file should be current");

        assert_eq!(stored.version, SCHEMA_VERSION);
        assert_eq!(stored, outcome.data);
    }

    #[test]
    fn a_newer_format_is_kept_aside_and_explained() {
        let directory = TempDirectory::new("version");
        let storage = directory.storage();
        fs::write(
            storage.path(),
            br#"{"version": 999, "setupStage": "learning"}"#,
        )
        .expect("file should be written");

        let outcome = storage.load();

        assert_eq!(outcome.data, AppData::fresh());
        assert!(
            outcome
                .notice
                .as_deref()
                .is_some_and(|notice| notice.contains("different version"))
        );
    }

    #[test]
    fn an_empty_file_is_treated_as_a_first_run() {
        let directory = TempDirectory::new("empty");
        let storage = directory.storage();
        fs::write(storage.path(), b"").expect("file should be written");

        let outcome = storage.load();

        assert_eq!(outcome.data, AppData::fresh());
        assert!(outcome.notice.is_none());
    }

    #[test]
    fn missing_optional_fields_fall_back_to_their_defaults() {
        let directory = TempDirectory::new("defaults");
        let storage = directory.storage();
        fs::write(storage.path(), br#"{"version": 2}"#).expect("file should be written");

        let outcome = storage.load();

        assert_eq!(outcome.data.setup_stage, SetupStage::Welcome);
        assert_eq!(outcome.data.progress, ProgressState::new());
        assert!(outcome.notice.is_none());
    }
}
