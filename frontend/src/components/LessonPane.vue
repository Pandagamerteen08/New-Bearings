<script setup lang="ts">
import { computed, ref } from 'vue'
import { learningContent as text } from '../../../content/interface/learning'
import {
  submitLessonAnswer,
  toLearningErrorCode,
  type LearningSnapshot,
  type LessonView,
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

const selected = ref<Record<string, number>>({})
const results = ref<Record<string, CheckResult>>({})
const pendingCheck = ref<string | null>(null)
const error = ref('')
const errorDetails = ref('')

const summary = computed(() => {
  const path = props.snapshot.paths.find((candidate) => candidate.id === props.pathId)

  return path?.lessons.find((candidate) => candidate.id === props.lessonId)
})

const complete = computed(() => summary.value?.completed ?? false)
const answeredCount = computed(() => summary.value?.answeredCount ?? 0)

// Only a correct answer closes a check. An incorrect one keeps the choices
// open so the learner can try again, which is why nothing is recorded for it.
function isSolved(checkId: string): boolean {
  return results.value[checkId]?.correct === true
}

function canSubmit(checkId: string): boolean {
  return (
    selected.value[checkId] !== undefined &&
    !isSolved(checkId) &&
    pendingCheck.value === null
  )
}

async function checkAnswer(checkId: string) {
  const option = selected.value[checkId]

  if (option === undefined || pendingCheck.value !== null) {
    return
  }

  pendingCheck.value = checkId
  error.value = ''
  errorDetails.value = ''

  try {
    const outcome = await submitLessonAnswer(
      props.pathId,
      props.lessonId,
      checkId,
      option,
    )

    results.value = {
      ...results.value,
      [checkId]: { correct: outcome.correct, explanation: outcome.explanation },
    }

    emit('snapshot', outcome.snapshot)
  } catch (failure) {
    if (toLearningErrorCode(failure) === null) {
      error.value = text.errorUnexpected
    } else {
      error.value = text.errorCheckNotFound
    }

    errorDetails.value = String(failure)
  } finally {
    pendingCheck.value = null
  }
}
</script>

<template>
  <article class="lesson">
    <header class="lesson-header">
      <div class="lesson-meta">
        <span v-if="lesson.minutes">{{ text.lessonMinutesLabel(lesson.minutes) }}</span>
        <span>{{ text.lessonChecksLabel(lesson.checks.length) }}</span>
      </div>

      <h1 class="lesson-title">{{ lesson.title }}</h1>
      <p class="lesson-summary">{{ lesson.summary }}</p>

      <p class="lesson-progress">
        {{ text.lessonProgressLabel(answeredCount, lesson.checks.length) }}
        <span v-if="complete" class="complete-badge">{{ text.lessonCompletedLabel }}</span>
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

        <aside v-else class="block note">
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
            correct: results[check.id]?.correct === true,
            incorrect: results[check.id]?.correct === false,
          }"
        >
          <fieldset :disabled="isSolved(check.id) || pendingCheck !== null">
            <legend>{{ check.prompt }}</legend>

            <label
              v-for="(option, index) in check.options"
              :key="index"
              class="option"
            >
              <input
                v-model="selected[check.id]"
                type="radio"
                :name="`check-${check.id}`"
                :value="index"
              />
              <span>{{ option }}</span>
            </label>
          </fieldset>

          <button
            v-if="!isSolved(check.id)"
            type="button"
            class="check-button"
            :disabled="!canSubmit(check.id)"
            :aria-busy="pendingCheck === check.id"
            @click="checkAnswer(check.id)"
          >
            {{ text.checkSubmitLabel }}
          </button>

          <p
            v-if="!isSolved(check.id) && selected[check.id] === undefined"
            class="hint"
          >
            {{ text.checkChooseOptionHint }}
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
  padding-bottom: 26px;
  border-bottom: 1px solid rgb(255 255 255 / 8%);
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

.lesson-title {
  margin: 16px 0 12px;
  font-size: clamp(30px, 4vw, 42px);
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

.lesson-progress {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 12px;
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

.lesson-body {
  padding-top: 8px;
}

.block {
  margin: 28px 0;
}

.block h2 {
  margin: 0 0 10px;
  font-size: 19px;
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
  padding: 16px 18px;
  border-left: 2px solid #7167db;
  border-radius: 0 8px 8px 0;
  background: rgb(113 103 219 / 8%);
}

.note p {
  color: #c3c2e6;
  font-size: 14px;
}

.code {
  margin: 0;
  padding: 18px 20px;
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
  margin-top: 46px;
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
  margin-bottom: 18px;
  padding: 20px 22px;
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

.check-button {
  margin-top: 12px;
  min-height: 42px;
  padding: 0 20px;
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

.hint {
  margin: 12px 0 0;
  color: #8d91a8;
  font-size: 12px;
}

.result {
  margin-top: 16px;
  padding-top: 14px;
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
  margin: 34px 0 10px;
  padding: 22px;
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
  margin-top: 22px;
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
    padding: 16px;
  }
}
</style>
