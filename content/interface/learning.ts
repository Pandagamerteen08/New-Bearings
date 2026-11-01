// The editable wording for the learning screens.
//
// Application rules live in Rust; this file only holds the text the interface
// displays, so the wording can change without touching any logic.

import type { Difficulty } from '../../frontend/src/backend/learning'

export interface LearningContent {
  startLearningButton: string
  loadingLabel: string
  dashboardEyebrow: string
  welcomeTitleLine: string
  welcomeTitleAccent: string
  welcomeDescription: string
  returningTitleLine: string
  returningTitleAccent: string
  returningDescription: string
  resumeEyebrow: string
  resumeButton: string
  resumeLabel: (lessonTitle: string, pathTitle: string) => string
  pathsHeading: string
  pathProgressLabel: (completed: number, total: number) => string
  pathMinutesLabel: (minutes: number) => string
  pathLessonsLabel: (total: number) => string
  pathBackLabel: string
  lessonsHeading: string
  lessonMinutesLabel: (minutes: number) => string
  lessonChecksLabel: (total: number) => string
  lessonCompletedLabel: string
  lessonStartedLabel: string
  lessonBackLabel: string
  lessonProgressLabel: (answered: number, total: number) => string
  checkHeading: string
  checkSubmitLabel: string
  checkCorrectLabel: string
  checkIncorrectLabel: string
  checkExplanationHeading: string
  checkChooseOptionHint: string
  lessonCompleteTitle: string
  lessonCompleteDescription: string
  nextLessonButton: string
  returnToPathButton: string
  courseCompleteLabel: string
  difficultyLabels: Record<Difficulty, string>
  emptyStateLabel: string
  errorLearningUnavailable: string
  errorContentInvalid: string
  errorPathNotFound: string
  errorLessonNotFound: string
  errorCheckNotFound: string
  errorOptionOutOfRange: string
  errorUnexpected: string
  errorDetailsLabel: string
}

export const learningContent: LearningContent = {
  startLearningButton: 'Start learning',
  loadingLabel: 'Loading your learning paths...',
  dashboardEyebrow: 'YOUR LEARNING',
  welcomeTitleLine: 'Choose what to',
  welcomeTitleAccent: 'learn next.',
  welcomeDescription:
    'Each path is a short sequence of lessons with a few questions to check that the ideas have landed. Your progress is saved on this computer.',
  returningTitleLine: 'Welcome',
  returningTitleAccent: 'back.',
  returningDescription:
    'Your progress is here. Continue where you stopped, or open a path to look through its lessons.',
  resumeEyebrow: 'PICK UP WHERE YOU LEFT OFF',
  resumeButton: 'Continue learning',
  resumeLabel: (lessonTitle, pathTitle) => `${lessonTitle} · ${pathTitle}`,
  pathsHeading: 'Learning paths',
  pathProgressLabel: (completed, total) => `${completed} of ${total} lessons complete`,
  pathMinutesLabel: (minutes) => `about ${minutes} minutes`,
  pathLessonsLabel: (total) => (total === 1 ? '1 lesson' : `${total} lessons`),
  pathBackLabel: 'All paths',
  lessonsHeading: 'Lessons',
  lessonMinutesLabel: (minutes) => `about ${minutes} minutes`,
  lessonChecksLabel: (total) =>
    total === 1 ? '1 knowledge check' : `${total} knowledge checks`,
  lessonCompletedLabel: 'Complete',
  lessonStartedLabel: 'In progress',
  lessonBackLabel: 'Back to lessons',
  lessonProgressLabel: (answered, total) => `${answered} of ${total} checks answered`,
  checkHeading: 'Check your understanding',
  checkSubmitLabel: 'Check answer',
  checkCorrectLabel: 'Correct',
  checkIncorrectLabel: 'Not quite',
  checkExplanationHeading: 'Why',
  checkChooseOptionHint: 'Choose an answer first.',
  lessonCompleteTitle: 'Lesson complete',
  lessonCompleteDescription:
    'Every knowledge check in this lesson has been answered correctly.',
  nextLessonButton: 'Next lesson',
  returnToPathButton: 'Back to lessons',
  courseCompleteLabel: 'Every lesson in this path is complete.',
  difficultyLabels: {
    beginner: 'Beginner',
    intermediate: 'Intermediate',
    advanced: 'Advanced',
  },
  emptyStateLabel: 'No learning paths are installed in this build.',
  errorLearningUnavailable:
    'New Bearings could not load your learning paths. Please try again.',
  errorContentInvalid:
    'The learning content could not be read. This is a problem with this build, not with your progress.',
  errorPathNotFound: 'That learning path is no longer available.',
  errorLessonNotFound: 'That lesson is no longer available.',
  errorCheckNotFound: 'That knowledge check is no longer available.',
  errorOptionOutOfRange: 'That answer is not one of the available options.',
  errorUnexpected: 'New Bearings could not complete that step. Please try again.',
  errorDetailsLabel: 'View technical details',
}
