//! Initial setup and onboarding state.
//!
//! Rust owns the setup stage so the same transition rules apply no matter how
//! the interface changes. The interface only renders the snapshot it receives.

use serde::{Deserialize, Serialize};

/// Identifies the current stage of the application for a learner.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupStage {
    /// The welcome screen, shown before anything else.
    #[default]
    Welcome,
    /// The introduction that explains what the application is for.
    Introduction,
    /// Setup is finished; the learner belongs in the learning screens.
    Learning,
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
    pub can_start_learning: bool,
}

/// Represents the current setup state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SetupState {
    stage: SetupStage,
}

impl SetupState {
    /// Recreates a setup state from a stage, such as one read from disk.
    pub const fn from_stage(stage: SetupStage) -> Self {
        Self { stage }
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
            can_start_learning: matches!(self.stage, SetupStage::Introduction),
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
            SetupStage::Learning => Err(SetupError::AlreadyLearning),
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
            // Setup is finished once learning has begun, so the introduction
            // is no longer a screen the learner can return to.
            SetupStage::Learning => Err(SetupError::AlreadyLearning),
        }
    }

    /// Leaves the introduction and enters the learning screens.
    pub fn start_learning(&mut self) -> Result<(), SetupError> {
        match self.stage {
            SetupStage::Welcome => Err(SetupError::NotAtIntroduction),
            SetupStage::Introduction => {
                self.stage = SetupStage::Learning;
                Ok(())
            }
            SetupStage::Learning => Err(SetupError::AlreadyLearning),
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
    AlreadyLearning,
    NotAtIntroduction,
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyStarted => formatter.write_str("The initial setup has already started."),
            Self::AlreadyAtWelcome => {
                formatter.write_str("The initial setup is already at the welcome screen.")
            }
            Self::AlreadyLearning => {
                formatter.write_str("Initial setup is finished and learning has begun.")
            }
            Self::NotAtIntroduction => {
                formatter.write_str("Learning can only begin after the introduction.")
            }
        }
    }
}

impl std::error::Error for SetupError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_setup_starts_at_welcome() {
        let state = SetupState::default();

        assert_eq!(state.stage(), SetupStage::Welcome);
    }

    #[test]
    fn begin_advances_to_introduction() {
        let mut state = SetupState::default();

        assert!(state.begin().is_ok());
        assert_eq!(state.stage(), SetupStage::Introduction);
    }

    #[test]
    fn beginning_twice_returns_an_error() {
        let mut state = SetupState::default();

        assert!(state.begin().is_ok());
        assert_eq!(state.begin(), Err(SetupError::AlreadyStarted));
    }

    #[test]
    fn beginning_after_learning_returns_an_error() {
        let mut state = SetupState::default();

        state.begin().expect("introduction");
        state.start_learning().expect("learning");

        assert_eq!(state.begin(), Err(SetupError::AlreadyLearning));
    }

    #[test]
    fn return_to_welcome_goes_back_to_the_start() {
        let mut state = SetupState::default();

        state.begin().expect("introduction");

        assert!(state.return_to_welcome().is_ok());
        assert_eq!(state.stage(), SetupStage::Welcome);
    }

    #[test]
    fn returning_from_welcome_returns_an_error() {
        let mut state = SetupState::default();

        assert_eq!(state.return_to_welcome(), Err(SetupError::AlreadyAtWelcome));
    }

    #[test]
    fn learning_cannot_go_back_to_the_introduction() {
        let mut state = SetupState::default();

        state.begin().expect("introduction");
        state.start_learning().expect("learning");

        assert_eq!(state.return_to_welcome(), Err(SetupError::AlreadyLearning));
    }

    #[test]
    fn start_learning_requires_the_introduction() {
        let mut state = SetupState::default();

        assert_eq!(state.start_learning(), Err(SetupError::NotAtIntroduction));
        assert_eq!(state.stage(), SetupStage::Welcome);

        state.begin().expect("introduction");
        state.start_learning().expect("learning");

        assert_eq!(state.stage(), SetupStage::Learning);
    }

    #[test]
    fn starting_learning_twice_returns_an_error() {
        let mut state = SetupState::default();

        state.begin().expect("introduction");
        state.start_learning().expect("learning");

        assert_eq!(state.start_learning(), Err(SetupError::AlreadyLearning));
    }

    #[test]
    fn rejected_transition_leaves_the_stage_unchanged() {
        let mut state = SetupState::default();

        state.begin().expect("introduction");
        state.start_learning().expect("learning");

        assert!(state.return_to_welcome().is_err());
        assert!(state.begin().is_err());
        assert_eq!(state.stage(), SetupStage::Learning);
    }

    #[test]
    fn snapshot_describes_the_welcome_stage() {
        let snapshot = SetupState::default().snapshot();

        assert_eq!(snapshot.stage, SetupStage::Welcome);
        assert!(snapshot.can_begin);
        assert!(!snapshot.can_return_to_welcome);
        assert!(!snapshot.can_start_learning);
    }

    #[test]
    fn snapshot_describes_the_introduction_stage() {
        let mut state = SetupState::default();
        state.begin().expect("introduction");

        let snapshot = state.snapshot();

        assert_eq!(snapshot.stage, SetupStage::Introduction);
        assert!(!snapshot.can_begin);
        assert!(snapshot.can_return_to_welcome);
        assert!(snapshot.can_start_learning);
    }

    #[test]
    fn snapshot_describes_the_learning_stage() {
        let mut state = SetupState::default();
        state.begin().expect("introduction");
        state.start_learning().expect("learning");

        let snapshot = state.snapshot();

        assert_eq!(snapshot.stage, SetupStage::Learning);
        assert!(!snapshot.can_begin);
        assert!(!snapshot.can_return_to_welcome);
        assert!(!snapshot.can_start_learning);
    }

    #[test]
    fn setup_error_serializes_to_a_lowercase_code() {
        let json =
            serde_json::to_string(&SetupError::AlreadyStarted).expect("error should serialize");

        assert_eq!(json, r#""already_started""#);
    }

    #[test]
    fn stages_round_trip_through_serialization() {
        for stage in [
            SetupStage::Welcome,
            SetupStage::Introduction,
            SetupStage::Learning,
        ] {
            let json = serde_json::to_string(&stage).expect("stage should serialize");
            let restored: SetupStage = serde_json::from_str(&json).expect("stage should parse");

            assert_eq!(restored, stage);
        }
    }
}
