// The minimal bridge between the interface and the Rust setup commands.
//
// Rust decides the setup stage and the allowed transitions; this module only
// forwards requests and describes the results for TypeScript.

import { invoke } from '@tauri-apps/api/core'

/** The stages the Rust backend can return. */
export type SetupStage = 'welcome' | 'introduction' | 'learning'

/** The rejected-setup codes the Rust backend can return. */
export type SetupErrorCode =
  | 'already_started'
  | 'already_at_welcome'
  | 'already_learning'
  | 'not_at_introduction'

/** The setup state Rust reports after every command. */
export interface SetupSnapshot {
  stage: SetupStage
  canBegin: boolean
  canReturnToWelcome: boolean
  canStartLearning: boolean
}

export function getSetupState(): Promise<SetupSnapshot> {
  return invoke<SetupSnapshot>('get_setup_state')
}

export function beginSetup(): Promise<SetupSnapshot> {
  return invoke<SetupSnapshot>('begin_setup')
}

export function returnToWelcome(): Promise<SetupSnapshot> {
  return invoke<SetupSnapshot>('return_to_welcome')
}

export function startLearning(): Promise<SetupSnapshot> {
  return invoke<SetupSnapshot>('start_learning')
}

const ERROR_CODES: readonly SetupErrorCode[] = [
  'already_started',
  'already_at_welcome',
  'already_learning',
  'not_at_introduction',
]

/** Recognizes a setup error code; anything else is an unexpected failure. */
export function toSetupErrorCode(error: unknown): SetupErrorCode | null {
  return ERROR_CODES.find((code) => code === error) ?? null
}
