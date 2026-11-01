//! Initial setup and onboarding state.
//!
//! Rust owns the setup stage so the same transition rules apply no matter how
//! the interface changes. The interface only renders the snapshot it receives.

use std::sync::Mutex;

use serde::Serialize;

/// Identifies the current stage of the initial application setup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupStage {
    Welcome,
    Introduction,
}

/// The setup information the interface needs in order to render a screen.
///
/// The `can_*` flags are derived from the same rules that the transitions
/// enforce, so the interface never has to decide what is allowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetupSnapshot {
    pub stage: SetupStage,
    pub can_begin: bool,
    pub can_return_to_welcome: bool,
}

/// Represents the current setup state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetupState {
    stage: SetupStage,
}

impl SetupState {
    /// Creates a new setup state at the welcome stage.
    pub const fn new() -> Self {
        Self {
            stage: SetupStage::Welcome,
        }
    }

    /// Returns the current setup stage.
    pub const fn stage(&self) -> SetupStage {
        self.stage
    }

    /// Describes the current state for the interface.
    pub const fn snapshot(&self) -> SetupSnapshot {
        SetupSnapshot {
            stage: self.stage(),
            can_begin: matches!(self.stage, SetupStage::Welcome),
            can_return_to_welcome: matches!(self.stage, SetupStage::Introduction),
        }
    }

    /// Advances from the welcome screen to the introduction.
    pub fn begin(&mut self) -> Result<(), SetupError> {
        match self.stage {
            SetupStage::Welcome => {
                self.stage = SetupStage::Introduction;
                Ok(())
            }
            SetupStage::Introduction => Err(SetupError::AlreadyStarted),
        }
    }

    /// Goes back from the introduction to the welcome screen.
    pub fn return_to_welcome(&mut self) -> Result<(), SetupError> {
        match self.stage {
            SetupStage::Welcome => Err(SetupError::AlreadyAtWelcome),
            SetupStage::Introduction => {
                self.stage = SetupStage::Welcome;
                Ok(())
            }
        }
    }
}

/// Describes a setup operation that could not be completed.
///
/// The value is sent to the interface as a short code so that user-facing
/// wording stays in editable content files. [`Display`](std::fmt::Display)
/// provides the developer-facing description.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupError {
    AlreadyStarted,
    AlreadyAtWelcome,
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyStarted => {
                formatter.write_str("The initial setup has already started.")
            }
            Self::AlreadyAtWelcome => {
                formatter.write_str("The initial setup is already at the welcome screen.")
            }
        }
    }
}

impl std::error::Error for SetupError {}

/// Holds the setup state for the whole application.
///
/// Tauri shares one instance of this value with every command, so the stage
/// survives between invocations instead of resetting on each call.
#[derive(Debug)]
pub struct SetupStore {
    state: Mutex<SetupState>,
}

impl SetupStore {
    /// Creates a store that starts at the beginning of the setup flow.
    pub fn new() -> Self {
        Self {
            state: Mutex::new(SetupState::new()),
        }
    }

    /// Returns the current snapshot without changing the state.
    pub fn snapshot(&self) -> SetupSnapshot {
        self.with_state(|state| state.snapshot())
    }

    /// Advances the setup and returns the resulting snapshot.
    pub fn begin(&self) -> Result<SetupSnapshot, SetupError> {
        self.with_state(|state| {
            state.begin()?;
            Ok(state.snapshot())
        })
    }

    /// Returns to the welcome screen and returns the resulting snapshot.
    pub fn return_to_welcome(&self) -> Result<SetupSnapshot, SetupError> {
        self.with_state(|state| {
            state.return_to_welcome()?;
            Ok(state.snapshot())
        })
    }

    /// Runs `action` while holding the lock, which is released immediately after.
    ///
    /// The state is a plain `Copy` value, so recovering a poisoned lock cannot
    /// expose a half-written state.
    fn with_state<T>(&self, action: impl FnOnce(&mut SetupState) -> T) -> T {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        action(&mut state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_setup_starts_at_welcome() {
        let state = SetupState::new();

        assert_eq!(state.stage(), SetupStage::Welcome);
    }

    #[test]
    fn begin_advances_to_introduction() {
        let mut state = SetupState::new();

        assert!(state.begin().is_ok());
        assert_eq!(state.stage(), SetupStage::Introduction);
    }

    #[test]
    fn beginning_twice_returns_an_error() {
        let mut state = SetupState::new();

        assert!(state.begin().is_ok());
        assert_eq!(state.begin(), Err(SetupError::AlreadyStarted));
    }

    #[test]
    fn return_to_welcome_goes_back_to_the_start() {
        let mut state = SetupState::new();

        assert!(state.begin().is_ok());
        assert!(state.return_to_welcome().is_ok());
        assert_eq!(state.stage(), SetupStage::Welcome);
    }

    #[test]
    fn returning_from_welcome_returns_an_error() {
        let mut state = SetupState::new();

        assert_eq!(state.return_to_welcome(), Err(SetupError::AlreadyAtWelcome));
        assert_eq!(state.stage(), SetupStage::Welcome);
    }

    #[test]
    fn rejected_transition_leaves_the_stage_unchanged() {
        let mut state = SetupState::new();

        assert!(state.begin().is_ok());
        assert_eq!(state.begin(), Err(SetupError::AlreadyStarted));
        assert_eq!(state.stage(), SetupStage::Introduction);
    }

    #[test]
    fn snapshot_describes_the_welcome_stage() {
        let snapshot = SetupState::new().snapshot();

        assert_eq!(snapshot.stage, SetupStage::Welcome);
        assert!(snapshot.can_begin);
        assert!(!snapshot.can_return_to_welcome);
    }

    #[test]
    fn snapshot_describes_the_introduction_stage() {
        let mut state = SetupState::new();

        assert!(state.begin().is_ok());

        let snapshot = state.snapshot();

        assert_eq!(snapshot.stage, SetupStage::Introduction);
        assert!(!snapshot.can_begin);
        assert!(snapshot.can_return_to_welcome);
    }

    #[test]
    fn store_keeps_the_stage_between_calls() {
        let store = SetupStore::new();

        let advanced = store.begin().expect("the first begin should succeed");
        assert_eq!(advanced.stage, SetupStage::Introduction);

        let unchanged = store.snapshot();
        assert_eq!(unchanged.stage, SetupStage::Introduction);

        let repeated = store.begin();
        assert_eq!(repeated, Err(SetupError::AlreadyStarted));
    }

    #[test]
    fn store_can_return_to_the_welcome_screen() {
        let store = SetupStore::new();

        assert!(store.begin().is_ok());

        let restored = store
            .return_to_welcome()
            .expect("returning to welcome should succeed");

        assert_eq!(restored.stage, SetupStage::Welcome);
        assert!(restored.can_begin);
    }

    #[test]
    fn setup_stage_serializes_to_a_lowercase_code() {
        let snapshot = SetupState::new().snapshot();
        let json = serde_json::to_string(&snapshot).expect("snapshot should serialize");

        assert_eq!(
            json,
            r#"{"stage":"welcome","canBegin":true,"canReturnToWelcome":false}"#
        );
    }

    #[test]
    fn setup_error_serializes_to_a_lowercase_code() {
        let json = serde_json::to_string(&SetupError::AlreadyStarted)
            .expect("error should serialize");

        assert_eq!(json, r#""already_started""#);
    }
}
