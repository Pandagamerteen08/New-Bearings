// The editable wording for the learning screens.
//
// Application rules live in Rust; this file only holds the text the interface
// displays, so the wording can change without touching any logic.

import type { Difficulty, MatchedField } from '../../frontend/src/backend/learning'

export interface LearningContent {
  // Navigation
  dashboardTabLabel: string
  reviewTabLabel: (count: number) => string
  loadingLabel: string

  // Dashboard
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
  pathMistakesLabel: (count: number) => string

  // Progress summary
  statsLessonsHeading: string
  statsChecksHeading: string
  statsMinutesHeading: string
  statsBookmarksHeading: string
  statsLessonsLabel: (completed: number, total: number) => string
  statsChecksLabel: (solved: number, total: number) => string
  statsMinutesLabel: (minutes: number) => string
  statsBookmarksLabel: (count: number) => string

  // Search
  searchLabel: string
  searchPlaceholder: string
  searchButton: string
  searchClearButton: string
  searchResultsHeading: (count: number) => string
  searchNoResults: string
  searchMatchLabel: (field: MatchedField) => string

  // Path view
  pathBackLabel: string
  lessonsHeading: string

  // Lesson
  lessonMinutesLabel: (minutes: number) => string
  lessonChecksLabel: (total: number) => string
  lessonCompletedLabel: string
  lessonStartedLabel: string
  lessonBackLabel: string
  lessonProgressLabel: (answered: number, total: number) => string
  lessonMistakesLabel: (count: number) => string
  objectivesHeading: string
  commonMistakeLabel: string

  // Checks
  checkHeading: string
  checkSubmitLabel: string
  checkCorrectLabel: string
  checkIncorrectLabel: string
  checkExplanationHeading: string
  checkChooseOptionHint: string
  checkTypeAnswerHint: string
  checkSolvedPreviouslyLabel: string
  checkAttemptsLabel: (count: number) => string
  trueLabel: string
  falseLabel: string
  shortAnswerLabel: string
  shortAnswerPlaceholder: string

  // Completion
  lessonCompleteTitle: string
  lessonCompleteDescription: string
  nextLessonButton: string
  returnToPathButton: string
  courseCompleteLabel: string

  // Review
  reviewEyebrow: string
  reviewTitleLine: string
  reviewTitleAccent: string
  reviewDescription: string
  reviewEmptyLabel: string
  reviewMistakesLabel: (count: number) => string
  reviewCompletedHint: string
  reviewButton: string
  reviewBackLabel: string

  // Bookmarks
  bookmarkAddLabel: string
  bookmarkRemoveLabel: string

  // Difficulty
  difficultyLabels: Record<Difficulty, string>

  // States and errors
  emptyStateLabel: string
  startLearningButton: string
  errorLearningUnavailable: string
  errorContentInvalid: string
  errorPathNotFound: string
  errorLessonNotFound: string
  errorCheckNotFound: string
  errorOptionOutOfRange: string
  errorAnswerMismatch: string
  errorEmptyAnswer: string
  errorSearchUnavailable: string
  errorUnexpected: string
  errorDetailsLabel: string
}

export const learningContent: LearningContent = {
  dashboardTabLabel: 'Learning paths',
  reviewTabLabel: (count) => (count === 0 ? 'Review' : `Review (${count})`),
  loadingLabel: 'Loading your learning paths...',

  dashboardEyebrow: 'YOUR LEARNING',
  welcomeTitleLine: 'Choose what to',
  welcomeTitleAccent: 'learn next.',
  welcomeDescription:
    'Each path is a short sequence of lessons with questions to check that the ideas have landed. Three kinds of question appear as you go: pick an answer, decide true or false, or type it yourself. Your progress is saved on this computer.',
  returningTitleLine: 'Welcome',
  returningTitleAccent: 'back.',
  returningDescription:
    'Your progress is here. Continue where you stopped, look through a path, or revisit the questions that caught you out.',
  resumeEyebrow: 'PICK UP WHERE YOU LEFT OFF',
  resumeButton: 'Continue learning',
  resumeLabel: (lessonTitle, pathTitle) => `${lessonTitle} · ${pathTitle}`,
  pathsHeading: 'Learning paths',
  pathProgressLabel: (completed, total) => `${completed} of ${total} lessons complete`,
  pathMinutesLabel: (minutes) => `about ${minutes} minutes`,
  pathLessonsLabel: (total) => (total === 1 ? '1 lesson' : `${total} lessons`),
  pathMistakesLabel: (count) =>
    count === 1 ? '1 question to review' : `${count} questions to review`,

  statsLessonsHeading: 'Lessons',
  statsChecksHeading: 'Questions',
  statsMinutesHeading: 'Time',
  statsBookmarksHeading: 'Saved',
  statsLessonsLabel: (completed, total) => `${completed} of ${total} lessons`,
  statsChecksLabel: (solved, total) => `${solved} of ${total} questions`,
  statsMinutesLabel: (minutes) => `${minutes} minutes learned`,
  statsBookmarksLabel: (count) => (count === 1 ? '1 saved lesson' : `${count} saved lessons`),

  searchLabel: 'Search lessons',
  searchPlaceholder: 'Search titles, summaries and objectives',
  searchButton: 'Search',
  searchClearButton: 'Clear search',
  searchResultsHeading: (count) => (count === 1 ? '1 result' : `${count} results`),
  searchNoResults: 'Nothing matches that search yet.',
  searchMatchLabel: (field) => (field === 'path' ? 'Path' : 'Lesson'),

  pathBackLabel: 'All paths',
  lessonsHeading: 'Lessons',

  lessonMinutesLabel: (minutes) => `about ${minutes} minutes`,
  lessonChecksLabel: (total) =>
    total === 1 ? '1 knowledge check' : `${total} knowledge checks`,
  lessonCompletedLabel: 'Complete',
  lessonStartedLabel: 'In progress',
  lessonBackLabel: 'Back to lessons',
  lessonProgressLabel: (answered, total) => `${answered} of ${total} checks answered`,
  lessonMistakesLabel: (count) =>
    count === 1 ? '1 wrong answer so far' : `${count} wrong answers so far`,
  objectivesHeading: 'What you will learn',
  commonMistakeLabel: 'Common mistake',

  checkHeading: 'Check your understanding',
  checkSubmitLabel: 'Check answer',
  checkCorrectLabel: 'Correct',
  checkIncorrectLabel: 'Not quite',
  checkExplanationHeading: 'Why',
  checkChooseOptionHint: 'Choose an answer first.',
  checkTypeAnswerHint: 'Type an answer first.',
  checkSolvedPreviouslyLabel: 'You answered this correctly earlier.',
  checkAttemptsLabel: (count) => (count === 1 ? '1 attempt' : `${count} attempts`),
  trueLabel: 'True',
  falseLabel: 'False',
  shortAnswerLabel: 'Your answer',
  shortAnswerPlaceholder: 'Type your answer',

  lessonCompleteTitle: 'Lesson complete',
  lessonCompleteDescription:
    'Every knowledge check in this lesson has been answered correctly.',
  nextLessonButton: 'Next lesson',
  returnToPathButton: 'Back to lessons',
  courseCompleteLabel: 'Every lesson in this path is complete.',

  reviewEyebrow: 'REVIEW',
  reviewTitleLine: 'Give it another',
  reviewTitleAccent: 'look.',
  reviewDescription:
    'These lessons contain a question you answered wrongly. Reopening a lesson keeps everything you have already answered correctly, so you only need to finish what is left.',
  reviewEmptyLabel: 'No wrong answers to revisit. Questions you miss will appear here.',
  reviewMistakesLabel: (count) => (count === 1 ? '1 wrong answer' : `${count} wrong answers`),
  reviewCompletedHint: 'Lesson complete, but worth another look',
  reviewButton: 'Open lesson',
  reviewBackLabel: 'All paths',

  bookmarkAddLabel: 'Save this lesson',
  bookmarkRemoveLabel: 'Remove from saved lessons',

  difficultyLabels: {
    beginner: 'Beginner',
    intermediate: 'Intermediate',
    advanced: 'Advanced',
  },

  emptyStateLabel: 'No learning paths are installed in this build.',
  startLearningButton: 'Start learning',
  errorLearningUnavailable:
    'New Bearings could not load your learning paths. Please try again.',
  errorContentInvalid:
    'The learning content could not be read. This is a problem with this build, not with your progress.',
  errorPathNotFound: 'That learning path is no longer available.',
  errorLessonNotFound: 'That lesson is no longer available.',
  errorCheckNotFound: 'That knowledge check is no longer available.',
  errorOptionOutOfRange: 'That answer is not one of the available options.',
  errorAnswerMismatch: 'That answer did not match the kind of question asked.',
  errorEmptyAnswer: 'Type an answer before checking it.',
  errorSearchUnavailable: 'New Bearings could not search your lessons. Please try again.',
  errorUnexpected: 'New Bearings could not complete that step. Please try again.',
  errorDetailsLabel: 'View technical details',
}
