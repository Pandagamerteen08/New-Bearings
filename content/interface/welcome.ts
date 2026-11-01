
export interface WelcomeContent {
  brandName: string
  accessLabel: string
  eyebrow: string
  titleLine: string
  titleAccent: string
  description: string
  getStartedButton: string
  footnote: string
  nextEyebrow: string
  nextTitleLine: string
  nextTitleAccent: string
  nextDescription: string
  backButton: string
  footerMessage: string
  backendCheckingLabel: string
  backendConnectedLabel: string
  backendFailedLabel: string
  backendErrorDetailsLabel: string
}

export const welcomeContent: WelcomeContent = {
  brandName: 'New Bearings',
  accessLabel: 'EARLY ACCESS',
  eyebrow: 'A NEW WAY TO LEARN',
  titleLine: 'Find your next',
  titleAccent: 'direction.',
  description:
    "Learning isn't one-size-fits-all. New Bearings is being built to help you discover your strengths, learn at your own pace, and make meaningful progress, one step at a time.",
  getStartedButton: 'Get Started',
  footnote: 'Your journey starts with a single step.',
  nextEyebrow: 'YOUR JOURNEY STARTS HERE',
  nextTitleLine: 'Every expert',
  nextTitleAccent: 'starts somewhere.',
  nextDescription:
    'Your learning experience is taking shape. This is the first step toward a learning companion designed around you.',
  backButton: 'Back to Welcome',
  footerMessage: 'Built for the journey ahead.',
  backendCheckingLabel: 'Connecting to the Rust backend...',
  backendConnectedLabel: 'Rust connection verified',
  backendFailedLabel: 'Unable to connect to the Rust backend',
  backendErrorDetailsLabel: 'View technical details',
}
