
/// Identifies the current stage of the initial application setup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupStage {
    Welcome,
    Introduction,
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
}

/// Describes a setup operation that could not be completed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupError {
    AlreadyStarted,
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyStarted => {
                formatter.write_str("The initial setup has already started.")
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
}
