//! Application identity and application-level coordination.

pub const APP_NAME: &str = "New Bearings";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

mod session;

pub use session::{AnswerOutcome, Session};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn application_name_is_new_bearings() {
        assert_eq!(APP_NAME, "New Bearings");
    }

    #[test]
    fn application_version_is_not_empty() {
        assert!(!APP_VERSION.is_empty());
    }
}
