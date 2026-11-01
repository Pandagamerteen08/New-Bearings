// The bridge between the interface and the Rust learning commands.
//
// Rust owns the catalog, the learner's position, the answers, progress and the
// review queue. This module only forwards requests and describes the results
// for TypeScript. A lesson arrives without its answer key: the backend grades
// every submitted answer itself.

import { invoke } from '@tauri-apps/api/core'

export type Difficulty = 'beginner' | 'intermediate' | 'advanced'

/** The kinds of knowledge check the backend can present. */
export type CheckKind = 'choice' | 'trueFalse' | 'shortAnswer'

export type Section =
  | { kind: 'explanation'; heading: string; body: string }
  | { kind: 'code'; heading: string | null; language: string; code: string }
  | { kind: 'note'; body: string }
  | { kind: 'steps'; heading: string; steps: string[] }
  | { kind: 'table'; heading: string; columns: string[]; rows: string[][] }
  | { kind: 'mistake'; body: string }

/**
 * A knowledge check as presented. The answer is deliberately absent, and
 * `solved` only says whether this learner already answered it correctly.
 */
export interface CheckView {
  id: string
  kind: CheckKind
  prompt: string
  /** Empty for checks that offer no options. */
  options: string[]
  solved: boolean
  attempts: number
}

export interface LessonView {
  id: string
  title: string
  summary: string
  minutes: number | null
  objectives: string[]
  sections: Section[]
  checks: CheckView[]
}

export interface LessonSummary {
  id: string
  title: string
  summary: string
  minutes: number | null
  checkCount: number
  /** Checks answered correctly. */
  solvedCount: number
  /** Every answer submitted, including wrong ones. */
  attemptCount: number
  /** Wrong answers recorded. */
  mistakes: number
  completed: boolean
  completedAt: number | null
  bookmarked: boolean
}

export interface PathView {
  id: string
  title: string
  summary: string
  subject: string
  difficulty: Difficulty
  tags: string[]
  totalMinutes: number
  completedLessons: number
  totalLessons: number
  mistakes: number
  lessons: LessonSummary[]
}

export type Screen =
  | { kind: 'dashboard' }
  | { kind: 'review' }
  | { kind: 'path'; pathId: string }
  | { kind: 'lesson'; pathId: string; lessonId: string }

export interface ResumePoint {
  pathId: string
  pathTitle: string
  lessonId: string
  lessonTitle: string
}

/** Totals across every path, derived by Rust from stored progress. */
export interface ProgressSummary {
  lessonsCompleted: number
  lessonsTotal: number
  checksSolved: number
  checksTotal: number
  mistakes: number
  bookmarks: number
  minutesCompleted: number
}

/** A lesson containing at least one question the learner answered wrongly. */
export interface ReviewItem {
  pathId: string
  pathTitle: string
  lessonId: string
  lessonTitle: string
  mistakes: number
  completed: boolean
  bookmarked: boolean
  lastAttemptAt: number
}

export interface LearningSnapshot {
  screen: Screen
  canGoBack: boolean
  paths: PathView[]
  lesson: LessonView | null
  resume: ResumePoint | null
  summary: ProgressSummary
  review: ReviewItem[]
  storageWarning: string | null
}

export interface AnswerOutcome {
  correct: boolean
  explanation: string
  lessonCompleted: boolean
  snapshot: LearningSnapshot
}

/** An answer submitted for grading. Its kind must match the check. */
export type SubmittedAnswer =
  | { kind: 'choice'; option: number }
  | { kind: 'trueFalse'; value: boolean }
  | { kind: 'shortAnswer'; text: string }

export type MatchedField = 'path' | 'lesson'

export interface SearchHit {
  pathId: string
  pathTitle: string
  lessonId: string | null
  lessonTitle: string | null
  matched: MatchedField
  /** The matching text, shortened by the backend when it is long. */
  text: string
}

/** The learning error codes the Rust backend can return. */
export type LearningErrorCode =
  | 'content_invalid'
  | 'path_not_found'
  | 'lesson_not_found'
  | 'check_not_found'
  | 'option_out_of_range'
  | 'answer_mismatch'
  | 'empty_answer'

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

export function showLearningDashboard(): Promise<LearningSnapshot> {
  return invoke<LearningSnapshot>('show_learning_dashboard')
}

export function showLearningReview(): Promise<LearningSnapshot> {
  return invoke<LearningSnapshot>('show_learning_review')
}

export function goBackInLearning(): Promise<LearningSnapshot> {
  return invoke<LearningSnapshot>('go_back_in_learning')
}

export function submitLessonAnswer(
  pathId: string,
  lessonId: string,
  checkId: string,
  answer: SubmittedAnswer,
): Promise<AnswerOutcome> {
  return invoke<AnswerOutcome>('submit_lesson_answer', {
    pathId,
    lessonId,
    checkId,
    answer,
  })
}

export function toggleLessonBookmark(
  pathId: string,
  lessonId: string,
): Promise<LearningSnapshot> {
  return invoke<LearningSnapshot>('toggle_lesson_bookmark', { pathId, lessonId })
}

export function searchLearning(query: string): Promise<SearchHit[]> {
  return invoke<SearchHit[]>('search_learning', { query })
}

const ERROR_CODES: readonly LearningErrorCode[] = [
  'content_invalid',
  'path_not_found',
  'lesson_not_found',
  'check_not_found',
  'option_out_of_range',
  'answer_mismatch',
  'empty_answer',
]

/** Recognizes a learning error code; anything else is an unexpected failure. */
export function toLearningErrorCode(error: unknown): LearningErrorCode | null {
  return ERROR_CODES.find((code) => code === error) ?? null
}
