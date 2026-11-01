<script setup lang="ts">
import { computed, ref } from 'vue'
import { learningContent as text } from '../../../content/interface/learning'
import {
  submitLessonAnswer,
  toggleLessonBookmark,
  toLearningErrorCode,
  type CheckView,
  type LearningSnapshot,
  type LessonView,
  type SubmittedAnswer,
} from '../backend/learning'

const props = defineProps<{
  snapshot: LearningSnapshot
  pathId: string
  lessonId: string
  lesson: LessonView
  nextLessonId: string | null
  nextLessonTitle: string | null
}>()

const emit = defineEmits<{
  (event: 'snapshot', snapshot: LearningSnapshot): void
  (event: 'open-lesson', lessonId: string): void
  (event: 'back'): void
}>()

interface CheckResult {
  correct: boolean
  explanation: string
}

// One selection per interaction style, so an answer keeps its own type until it
// is packed into the shape Rust grades.
const choiceSelection = ref<Record<string, number>>({})
const boolSelection = ref<Record<string, boolean>>({})
const textSelection = ref<Record<string, string>>({})

const results = ref<Record<string, CheckResult>>({})
const pendingCheck = ref<string | null>(null)
const isBookmarking = ref(false)
const error = ref('')
const errorDetails = ref('')

const summary = computed(() => {
  const path = props.snapshot.paths.find((candidate) => candidate.id === props.pathId)

  return path?.lessons.find((candidate) => candidate.id === props.lessonId)
})

const complete = computed(() => summary.value?.completed ?? false)
const solvedCount = computed(() => summary.value?.solvedCount ?? 0)
const mistakes = computed(() => summary.value?.mistakes ?? 0)
const bookmarked = computed(() => summary.value?.bookmarked ?? false)

// A check is solved when it was answered correctly earlier in another session,
// or just now in this one.
function isSolved(check: CheckView): boolean {
  return results.value[check.id]?.correct === true || check.solved
}

function answerFor(check: CheckView): SubmittedAnswer | null {
  switch (check.kind) {
    case 'choice': {
      const option = choiceSelection.value[check.id]

      return option === undefined ? null : { kind: 'choice', option }
    }
    case 'trueFalse': {
      const value = boolSelection.value[check.id]

      return value === undefined ? null : { kind: 'trueFalse', value }
    }
    case 'shortAnswer': {
      const text = (textSelection.value[check.id] ?? '').trim()

      return text.length === 0 ? null : { kind: 'shortAnswer', text }
    }
    default:
      return null
  }
}

function canSubmit(check: CheckView): boolean {
  return answerFor(check) !== null && !isSolved(check) && pendingCheck.value === null
}

function messageFor(failure: unknown): string {
  switch (toLearningErrorCode(failure)) {
    case 'empty_answer':
      return text.errorEmptyAnswer
    case 'answer_mismatch':
      return text.errorAnswerMismatch
    case 'check_not_found':
      return text.errorCheckNotFound
    case 'option_out_of_range':
      return text.errorOptionOutOfRange
    default:
      return text.errorUnexpected
  }
}

async function checkAnswer(check: CheckView) {
  const answer = answerFor(check)

  if (!answer || pendingCheck.value !== null || isSolved(check)) {
    return
  }

  pendingCheck.value = check.id
  error.value = ''
  errorDetails.value = ''

  try {
    const outcome = await submitLessonAnswer(
      props.pathId,
      props.lessonId,
      check.id,
      answer,
    )

    results.value = {
      ...results.value,
      [check.id]: { correct: outcome.correct, explanation: outcome.explanation },
    }

    emit('snapshot', outcome.snapshot)
  } catch (failure) {
    error.value = messageFor(failure)
    errorDetails.value = String(failure)
  } finally {
    pendingCheck.value = null
  }
}

async function toggleBookmark() {
  if (isBookmarking.value) {
    return
  }

  isBookmarking.value = true
  error.value = ''
  errorDetails.value = ''

  try {
    emit('snapshot', await toggleLessonBookmark(props.pathId, props.lessonId))
  } catch (failure) {
    error.value = text.errorUnexpected
    errorDetails.value = String(failure)
  } finally {
    isBookmarking.value = false
  }
}
</script>

<template>
  <article class="lesson">
    <header class="lesson-header">
      <div class="lesson-topline">
        <div class="lesson-meta">
          <span v-if="lesson.minutes">{{ text.lessonMinutesLabel(lesson.minutes) }}</span>
          <span>{{ text.lessonChecksLabel(lesson.checks.length) }}</span>
          <span v-if="summary && summary.attemptCount > 0">
            {{ text.checkAttemptsLabel(summary.attemptCount) }}
          </span>
        </div>

        <button
          type="button"
          class="bookmark"
          :class="{ on: bookmarked }"
          :disabled="isBookmarking"
          :aria-pressed="bookmarked"
          :aria-label="bookmarked ? text.bookmarkRemoveLabel : text.bookmarkAddLabel"
          @click="toggleBookmark"
        >
          <span aria-hidden="true">{{ bookmarked ? '★' : '☆' }}</span>
          <span class="bookmark-text">
            {{ bookmarked ? text.bookmarkRemoveLabel : text.bookmarkAddLabel }}
          </span>
        </button>
      </div>

      <h1 class="lesson-title">{{ lesson.title }}</h1>
      <p class="lesson-summary">{{ lesson.summary }}</p>

      <section v-if="lesson.objectives.length" class="objectives">
        <h2>{{ text.objectivesHeading }}</h2>
        <ul>
          <li v-for="objective in lesson.objectives" :key="objective">{{ objective }}</li>
        </ul>
      </section>

      <p class="lesson-progress">
        {{ text.lessonProgressLabel(solvedCount, lesson.checks.length) }}
        <span v-if="complete" class="complete-badge">{{ text.lessonCompletedLabel }}</span>
        <span v-if="mistakes > 0" class="mistake-badge">
          {{ text.lessonMistakesLabel(mistakes) }}
        </span>
      </p>
    </header>

    <div class="lesson-body">
      <template v-for="(section, index) in lesson.sections" :key="index">
        <section v-if="section.kind === 'explanation'" class="block">
          <h2>{{ section.heading }}</h2>
          <p>{{ section.body }}</p>
        </section>

        <section v-else-if="section.kind === 'code'" class="block">
          <h2 v-if="section.heading">{{ section.heading }}</h2>
          <pre class="code"><code>{{ section.code }}</code></pre>
        </section>

        <aside v-else-if="section.kind === 'note'" class="block note">
          <p>{{ section.body }}</p>
        </aside>

        <section v-else-if="section.kind === 'steps'" class="block">
          <h2>{{ section.heading }}</h2>
          <ol class="steps">
            <li v-for="step in section.steps" :key="step">{{ step }}</li>
          </ol>
        </section>

        <section v-else-if="section.kind === 'table'" class="block">
          <h2>{{ section.heading }}</h2>
          <div class="table-scroll">
            <table>
              <thead>
                <tr>
                  <th v-for="column in section.columns" :key="column" scope="col">
                    {{ column }}
                  </th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(row, rowIndex) in section.rows" :key="rowIndex">
                  <td v-for="(cell, cellIndex) in row" :key="cellIndex">{{ cell }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>

        <aside v-else class="block mistake">
          <p class="mistake-label">{{ text.commonMistakeLabel }}</p>
          <p>{{ section.body }}</p>
        </aside>
      </template>

      <section class="checks" aria-labelledby="checks-heading">
        <h2 id="checks-heading">{{ text.checkHeading }}</h2>

        <div
          v-for="check in lesson.checks"
          :key="check.id"
          class="check"
          :class="{
            correct: isSolved(check),
            incorrect: results[check.id]?.correct === false,
          }"
        >
          <fieldset :disabled="isSolved(check) || pendingCheck !== null">
            <legend>{{ check.prompt }}</legend>

            <template v-if="check.kind === 'choice'">
              <label v-for="(option, index) in check.options" :key="index" class="option">
                <input
                  v-model="choiceSelection[check.id]"
                  type="radio"
                  :name="`check-${check.id}`"
                  :value="index"
                />
                <span>{{ option }}</span>
              </label>
            </template>

            <template v-else-if="check.kind === 'trueFalse'">
              <label class="option">
                <input
                  v-model="boolSelection[check.id]"
                  type="radio"
                  :name="`check-${check.id}`"
                  :value="true"
                />
                <span>{{ text.trueLabel }}</span>
              </label>

              <label class="option">
                <input
                  v-model="boolSelection[check.id]"
                  type="radio"
                  :name="`check-${check.id}`"
                  :value="false"
                />
                <span>{{ text.falseLabel }}</span>
              </label>
            </template>

            <label v-else class="answer-field">
              <span class="answer-label">{{ text.shortAnswerLabel }}</span>
              <input
                v-model="textSelection[check.id]"
                type="text"
                :placeholder="text.shortAnswerPlaceholder"
                autocomplete="off"
                spellcheck="false"
              />
            </label>
          </fieldset>

          <div class="check-actions">
            <button
              v-if="!isSolved(check)"
              type="button"
              class="check-button"
              :disabled="!canSubmit(check)"
              :aria-busy="pendingCheck === check.id"
              @click="checkAnswer(check)"
            >
              {{ text.checkSubmitLabel }}
            </button>

            <p v-if="check.solved && !results[check.id]" class="solved-note">
              {{ text.checkSolvedPreviouslyLabel }}
            </p>
          </div>

          <p
            v-if="
              !isSolved(check) &&
              check.kind !== 'shortAnswer' &&
              answerFor(check) === null &&
              pendingCheck === null
            "
            class="hint"
          >
            {{ text.checkChooseOptionHint }}
          </p>

          <p
            v-if="!isSolved(check) && check.kind === 'shortAnswer' && answerFor(check) === null"
            class="hint"
          >
            {{ text.checkTypeAnswerHint }}
          </p>

          <div v-if="results[check.id]" class="result" role="status">
            <p class="result-label">
              {{
                results[check.id].correct
                  ? text.checkCorrectLabel
                  : text.checkIncorrectLabel
              }}
            </p>
            <p class="result-body">
              <strong>{{ text.checkExplanationHeading }}:</strong>
              {{ results[check.id].explanation }}
            </p>
            <p v-if="!results[check.id].correct" class="result-retry">
              Choose another answer and check again.
            </p>
          </div>
        </div>
      </section>

      <div v-if="error" class="error" role="alert">
        <p>{{ error }}</p>
        <details>
          <summary>{{ text.errorDetailsLabel }}</summary>
          <pre>{{ errorDetails }}</pre>
        </details>
      </div>

      <section v-if="complete" class="completion" role="status">
        <h2>{{ text.lessonCompleteTitle }}</h2>
        <p>{{ text.lessonCompleteDescription }}</p>

        <button
          v-if="nextLessonId"
          type="button"
          class="primary"
          @click="emit('open-lesson', nextLessonId)"
        >
          {{ text.nextLessonButton }}: {{ nextLessonTitle }}
        </button>

        <button v-else type="button" class="secondary" @click="emit('back')">
          {{ text.returnToPathButton }}
        </button>
      </section>
    </div>
  </article>
</template>

<style scoped>
.lesson {
  width: min(100%, 760px);
}

.lesson-header {
  padding-bottom: 22px;
  border-bottom: 1px solid rgb(255 255 255 / 8%);
}

.lesson-topline {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.lesson-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 14px;
  color: #8d91a8;
  font-size: 11px;
  font-weight: 650;
  letter-spacing: 1.4px;
  text-transform: uppercase;
}

.bookmark {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 7px 12px;
  border: 1px solid #37394d;
  border-radius: 9px;
  background: #151722;
  color: #9d9fc0;
  cursor: pointer;
  font-size: 11px;
  font-weight: 650;
}

.bookmark.on {
  border-color: rgb(198 183 122 / 45%);
  color: #e0d3a4;
}

.bookmark:disabled {
  cursor: progress;
  opacity: 0.6;
}

.lesson-title {
  margin: 16px 0 12px;
  font-size: clamp(28px, 3.8vw, 40px);
  font-weight: 650;
  letter-spacing: -1.6px;
  line-height: 1.1;
}

.lesson-summary {
  max-width: 60ch;
  margin: 0;
  color: #a6a9bb;
  font-size: 15px;
  line-height: 1.8;
}

.objectives {
  margin-top: 20px;
  padding: 16px 18px;
  border: 1px solid #2b2d3d;
  border-radius: 11px;
  background: #10121b;
}

.objectives h2 {
  margin: 0 0 10px;
  color: #b4b0ff;
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 1.8px;
  text-transform: uppercase;
}

.objectives ul {
  margin: 0;
  padding-left: 18px;
  color: #b6b9c9;
  font-size: 14px;
  line-height: 1.8;
}

.lesson-progress {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 10px;
  margin: 18px 0 0;
  color: #8d91a8;
  font-size: 12px;
}

.complete-badge {
  padding: 3px 10px;
  border: 1px solid rgb(114 214 160 / 35%);
  border-radius: 999px;
  color: #8fe3b6;
  font-size: 11px;
  font-weight: 650;
  letter-spacing: 0.6px;
}

.mistake-badge {
  padding: 3px 10px;
  border: 1px solid rgb(198 183 122 / 35%);
  border-radius: 999px;
  color: #e0d3a4;
  font-size: 11px;
  font-weight: 650;
}

.lesson-body {
  padding-top: 8px;
}

.block {
  margin: 26px 0;
}

.block h2 {
  margin: 0 0 10px;
  font-size: 18px;
  font-weight: 650;
  letter-spacing: -0.4px;
}

.block p {
  margin: 0;
  color: #b6b9c9;
  font-size: 15px;
  line-height: 1.85;
}

.note {
  padding: 15px 17px;
  border-left: 2px solid #7167db;
  border-radius: 0 8px 8px 0;
  background: rgb(113 103 219 / 8%);
}

.note p {
  color: #c3c2e6;
  font-size: 14px;
}

.mistake {
  padding: 15px 17px;
  border-left: 2px solid #c6b77a;
  border-radius: 0 8px 8px 0;
  background: rgb(198 183 122 / 7%);
}

.mistake .mistake-label {
  margin: 0 0 6px;
  color: #e0d3a4;
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 1.6px;
  text-transform: uppercase;
}

.mistake p {
  color: #d3cdb8;
  font-size: 14px;
}

.steps {
  margin: 0;
  padding-left: 20px;
  color: #b6b9c9;
  font-size: 15px;
  line-height: 1.85;
}

.steps li {
  margin-bottom: 6px;
}

.table-scroll {
  overflow-x: auto;
  border: 1px solid #2b2d3d;
  border-radius: 10px;
}

table {
  width: 100%;
  border-collapse: collapse;
  font-size: 14px;
}

th,
td {
  padding: 11px 14px;
  border-bottom: 1px solid #232637;
  text-align: left;
  vertical-align: top;
}

th {
  background: #151722;
  color: #b4b0ff;
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 1.2px;
  text-transform: uppercase;
}

td {
  color: #b6b9c9;
  line-height: 1.65;
}

tbody tr:last-child td {
  border-bottom: 0;
}

.code {
  margin: 0;
  padding: 17px 19px;
  overflow-x: auto;
  border: 1px solid #2b2d3d;
  border-radius: 10px;
  background: #10121b;
  color: #e2e0ff;
  font-family: Consolas, "SFMono-Regular", monospace;
  font-size: 13.5px;
  line-height: 1.7;
}

.checks {
  margin-top: 44px;
}

.checks h2 {
  margin: 0 0 18px;
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 2px;
  text-transform: uppercase;
  color: #b4b0ff;
}

.check {
  margin-bottom: 16px;
  padding: 19px 21px;
  border: 1px solid #2b2d3d;
  border-radius: 12px;
  background: #12141e;
}

.check.correct {
  border-color: rgb(114 214 160 / 35%);
}

.check.incorrect {
  border-color: rgb(255 133 133 / 30%);
}

fieldset {
  margin: 0;
  padding: 0;
  border: 0;
}

legend {
  padding: 0;
  margin-bottom: 14px;
  color: #f4f5fb;
  font-size: 15px;
  font-weight: 650;
  line-height: 1.5;
}

.option {
  display: flex;
  align-items: flex-start;
  gap: 11px;
  margin-bottom: 9px;
  padding: 10px 12px;
  border: 1px solid #2b2d3d;
  border-radius: 9px;
  cursor: pointer;
  font-size: 14px;
  line-height: 1.5;
}

.option:hover {
  border-color: #4d4a86;
  background: #171a26;
}

.option input {
  margin: 2px 0 0;
  accent-color: #7d74e6;
}

fieldset:disabled .option {
  cursor: default;
  opacity: 0.75;
}

.answer-field {
  display: block;
}

.answer-label {
  display: block;
  margin-bottom: 7px;
  color: #8d91a8;
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 1.4px;
  text-transform: uppercase;
}

.answer-field input {
  width: min(100%, 380px);
  min-height: 42px;
  padding: 0 13px;
  border: 1px solid #37394d;
  border-radius: 9px;
  background: #10121b;
  color: #f4f5fb;
  font: inherit;
  font-size: 14px;
}

.answer-field input:focus-visible {
  border-color: #7770ca;
  outline: 2px solid #a5a2ff;
  outline-offset: 2px;
}

fieldset:disabled .answer-field input {
  opacity: 0.7;
}

.check-actions {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 12px;
  margin-top: 12px;
}

.check-button {
  min-height: 40px;
  padding: 0 18px;
  border: 1px solid transparent;
  border-radius: 9px;
  background: linear-gradient(110deg, #7067db, #5b54b7);
  color: white;
  cursor: pointer;
  font-size: 13px;
  font-weight: 650;
}

.check-button:disabled {
  cursor: not-allowed;
  opacity: 0.45;
}

.solved-note {
  margin: 0;
  color: #8fe3b6;
  font-size: 12px;
}

.hint {
  margin: 10px 0 0;
  color: #8d91a8;
  font-size: 12px;
}

.result {
  margin-top: 15px;
  padding-top: 13px;
  border-top: 1px solid rgb(255 255 255 / 7%);
}

.result-label {
  margin: 0 0 6px;
  color: #d9d8ff;
  font-size: 13px;
  font-weight: 700;
  letter-spacing: 0.4px;
}

.check.correct .result-label {
  color: #8fe3b6;
}

.check.incorrect .result-label {
  color: #ffb4b4;
}

.result-body {
  margin: 0;
  color: #b6b9c9;
  font-size: 14px;
  line-height: 1.75;
}

.result-body strong {
  color: #d9d8ff;
}

.result-retry {
  margin: 8px 0 0;
  color: #8d91a8;
  font-size: 12px;
}

.completion {
  margin: 30px 0 10px;
  padding: 20px;
  border: 1px solid rgb(114 214 160 / 30%);
  border-radius: 12px;
  background: rgb(114 214 160 / 6%);
}

.completion h2 {
  margin: 0 0 8px;
  font-size: 18px;
  font-weight: 650;
}

.completion p {
  margin: 0 0 16px;
  color: #b6b9c9;
  font-size: 14px;
  line-height: 1.7;
}

.primary,
.secondary {
  min-height: 44px;
  padding: 0 20px;
  border-radius: 9px;
  cursor: pointer;
  font-size: 13px;
  font-weight: 650;
}

.primary {
  border: 1px solid transparent;
  background: linear-gradient(110deg, #7067db, #5b54b7);
  color: white;
}

.secondary {
  border: 1px solid #37394d;
  background: #151722;
  color: #eeedff;
}

.error {
  margin-top: 20px;
  color: #ffb4b4;
  font-size: 13px;
  line-height: 1.7;
}

.error p {
  margin: 0;
}

.error pre {
  overflow-wrap: anywhere;
  white-space: pre-wrap;
  color: #d0b6b6;
  font-family: Consolas, monospace;
  font-size: 11px;
}

@media (max-width: 600px) {
  .check {
    padding: 15px;
  }

  .bookmark-text {
    display: none;
  }
}
</style>
