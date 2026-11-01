// The editable wording for the initial setup flow.
//
// Application rules live in Rust; this file only holds the text the interface
// displays, so the wording can change without touching any logic.

export interface SetupContent {
  errorAlreadyStarted: string
  errorAlreadyAtWelcome: string
  errorAlreadyLearning: string
  errorNotAtIntroduction: string
  errorUnexpected: string
  errorUnavailable: string
  errorDetailsLabel: string
}

export const setupContent: SetupContent = {
  errorAlreadyStarted: 'Your setup is already underway. Your progress has been kept.',
  errorAlreadyAtWelcome: 'You are already at the welcome screen.',
  errorAlreadyLearning: 'Setup is finished, so this step is no longer available.',
  errorNotAtIntroduction: 'Finish the introduction before starting to learn.',
  errorUnexpected: 'New Bearings could not update your setup. Please try again.',
  errorUnavailable: 'New Bearings could not load your setup. Please try again.',
  errorDetailsLabel: 'View technical details',
}
