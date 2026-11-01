// The bridge between the interface and the Rust learning commands.
//
// Rust owns the catalog, the learner's position, the answers and the progress.
// This module only forwards requests and describes the results for TypeScript.
// Note that a lesson arrives without its answer key: the backend judges every
// submitted answer itself.

import { invoke } from '@tauri-apps/api/core'

export type Difficulty = 'beginner' | 'intermediate' | 'advanced'

export type Section =
  | { kind: 'explanation'; heading: string; body: string }
  | { kind: 'code'; heading: string | null; language: string; code: string }
  | { kind: 'note'; body: string }

/** A knowledge check as presented: the answer is deliberately absent. */
export interface CheckView {
  id: string
  prompt: string
  options: string[]
}

export interface LessonView {
  id: string
  title: string
  summary: string
  minutes: number | null
  sections: Section[]
  checks: CheckView[]
}

export interface LessonSummary {
  id: string
  title: string
  summary: string
  minutes: number | null
  checkCount: number
  answeredCount: number
  completed: boolean
  completedAt: number | null
}

export interface PathView {
  id: string
  title: string
  summary: string
  subject: string
  difficulty: Difficulty
  totalMinutes: number
  completedLessons: number
  totalLessons: number
  lessons: LessonSummary[]
}

export type Screen =
  | { kind: 'dashboard' }
  | { kind: 'path'; pathId: string }
  | { kind: 'lesson'; pathId: string; lessonId: string }

export interface ResumePoint {
  pathId: string
  pathTitle: string
  lessonId: string
  lessonTitle: string
}

export interface LearningSnapshot {
  screen: Screen
  canGoBack: boolean
  paths: PathView[]
  lesson: LessonView | null
  resume: ResumePoint | null
  storageWarning: string | null
}

export interface AnswerOutcome {
  correct: boolean
  explanation: string
  lessonCompleted: boolean
  snapshot: LearningSnapshot
}

/** The learning error codes the Rust backend can return. */
export type LearningErrorCode =
  | 'content_invalid'
  | 'path_not_found'
  | 'lesson_not_found'
  | 'check_not_found'
  | 'option_out_of_range'

export function getLearningState(): Promise<LearningSnapshot> {
  return invoke<LearningSnapshot>('get_learning_state')
}

export function openLearningPath(pathId: string): Promise<LearningSnapshot> {
  return invoke<LearningSnapshot>('open_learning_path', { pathId })
}

export function openLearningLesson(
  pathId: string,
  lessonId: string,
): Promise<LearningSnapshot> {
  return invoke<LearningSnapshot>('open_learning_lesson', { pathId, lessonId })
}

export function goBackInLearning(): Promise<LearningSnapshot> {
  return invoke<LearningSnapshot>('go_back_in_learning')
}

export function submitLessonAnswer(
  pathId: string,
  lessonId: string,
  checkId: string,
  option: number,
): Promise<AnswerOutcome> {
  return invoke<AnswerOutcome>('submit_lesson_answer', {
    pathId,
    lessonId,
    checkId,
    option,
  })
}

const ERROR_CODES: readonly LearningErrorCode[] = [
  'content_invalid',
  'path_not_found',
  'lesson_not_found',
  'check_not_found',
  'option_out_of_range',
]

/** Recognizes a learning error code; anything else is an unexpected failure. */
export function toLearningErrorCode(error: unknown): LearningErrorCode | null {
  return ERROR_CODES.find((code) => code === error) ?? null
}
