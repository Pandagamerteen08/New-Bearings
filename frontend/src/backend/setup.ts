// The minimal bridge between the interface and the Rust setup commands.
//
// Rust decides the setup stage and the allowed transitions; this module only
// forwards requests and describes the results for TypeScript.

import { invoke } from '@tauri-apps/api/core'

/** The setup stages the Rust backend can return. */
export type SetupStage = 'welcome' | 'introduction'

/** The rejected-transition codes the Rust backend can return. */
export type SetupErrorCode = 'already_started' | 'already_at_welcome'

/** The setup state Rust reports after every command. */
export interface SetupSnapshot {
  stage: SetupStage
  canBegin: boolean
  canReturnToWelcome: boolean
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

/** Recognizes a setup error code; anything else is an unexpected failure. */
export function toSetupErrorCode(error: unknown): SetupErrorCode | null {
  if (error === 'already_started' || error === 'already_at_welcome') {
    return error
  }

  return null
}
