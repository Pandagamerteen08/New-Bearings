<script setup lang="ts">
import { computed, ref } from 'vue'
import { learningContent as text } from '../../../content/interface/learning'
import {
  goBackInLearning,
  openLearningLesson,
  openLearningPath,
  toLearningErrorCode,
  type LearningSnapshot,
} from '../backend/learning'
import LessonPane from './LessonPane.vue'

const props = defineProps<{ snapshot: LearningSnapshot }>()

const emit = defineEmits<{
  (event: 'snapshot', snapshot: LearningSnapshot): void
}>()

const busy = ref(false)
const error = ref('')
const errorDetails = ref('')

const screen = computed(() => props.snapshot.screen)

const hasProgress = computed(() =>
  props.snapshot.paths.some((path) =>
    path.lessons.some((lesson) => lesson.completed || lesson.answeredCount > 0),
  ),
)

const openPathView = computed(() => {
  const current = screen.value

  if (current.kind !== 'path') {
    return undefined
  }

  return props.snapshot.paths.find((path) => path.id === current.pathId)
})

const openLessonContext = computed(() => {
  const current = screen.value

  if (current.kind !== 'lesson') {
    return undefined
  }

  const path = props.snapshot.paths.find((candidate) => candidate.id === current.pathId)

  if (!path) {
    return undefined
  }

  const index = path.lessons.findIndex((lesson) => lesson.id === current.lessonId)
  const next = index >= 0 ? path.lessons[index + 1] : undefined

  return {
    pathId: current.pathId,
    lessonId: current.lessonId,
    nextLessonId: next?.id ?? null,
    nextLessonTitle: next?.title ?? null,
  }
})

function messageFor(failure: unknown, fallback: string): string {
  switch (toLearningErrorCode(failure)) {
    case 'content_invalid':
      return text.errorContentInvalid
    case 'path_not_found':
      return text.errorPathNotFound
    case 'lesson_not_found':
      return text.errorLessonNotFound
    case 'check_not_found':
      return text.errorCheckNotFound
    case 'option_out_of_range':
      return text.errorOptionOutOfRange
    default:
      return fallback
  }
}

async function run(command: () => Promise<LearningSnapshot>, fallback: string) {
  if (busy.value) {
    return
  }

  busy.value = true
  error.value = ''
  errorDetails.value = ''

  try {
    emit('snapshot', await command())
  } catch (failure) {
    error.value = messageFor(failure, fallback)
    errorDetails.value = String(failure)
  } finally {
    busy.value = false
  }
}

function openPath(pathId: string) {
  void run(() => openLearningPath(pathId), text.errorUnexpected)
}

function openLesson(pathId: string, lessonId: string) {
  void run(() => openLearningLesson(pathId, lessonId), text.errorUnexpected)
}

function goBack() {
  void run(goBackInLearning, text.errorUnexpected)
}

function continueLearning() {
  const resume = props.snapshot.resume

  if (resume) {
    openLesson(resume.pathId, resume.lessonId)
  }
}

function openLessonInPath(lessonId: string) {
  const context = openLessonContext.value

  if (context) {
    openLesson(context.pathId, lessonId)
  }
}

function forwardSnapshot(snapshot: LearningSnapshot) {
  error.value = ''
  errorDetails.value = ''
  emit('snapshot', snapshot)
}
</script>

<template>
  <div class="learning">
    <p v-if="snapshot.storageWarning" class="warning" role="status">
      {{ snapshot.storageWarning }}
    </p>

    <div v-if="error" class="error" role="alert">
      <p>{{ error }}</p>
      <details>
        <summary>{{ text.errorDetailsLabel }}</summary>
        <pre>{{ errorDetails }}</pre>
      </details>
    </div>

    <template v-if="screen.kind === 'lesson' && snapshot.lesson && openLessonContext">
      <button type="button" class="back" :disabled="busy" @click="goBack">
        <span aria-hidden="true">←</span>
        {{ text.lessonBackLabel }}
      </button>

      <LessonPane
        :snapshot="snapshot"
        :path-id="openLessonContext.pathId"
        :lesson-id="openLessonContext.lessonId"
        :lesson="snapshot.lesson"
        :next-lesson-id="openLessonContext.nextLessonId"
        :next-lesson-title="openLessonContext.nextLessonTitle"
        @snapshot="forwardSnapshot"
        @open-lesson="openLessonInPath"
        @back="goBack"
      />
    </template>

    <template v-else-if="screen.kind === 'path'">
      <button type="button" class="back" :disabled="busy" @click="goBack">
        <span aria-hidden="true">←</span>
        {{ text.pathBackLabel }}
      </button>

      <template v-if="openPathView">
        <header class="page-header">
          <p class="eyebrow">{{ openPathView.subject }}</p>
          <h1 class="page-title">{{ openPathView.title }}</h1>
          <p class="page-description">{{ openPathView.summary }}</p>

          <p class="page-meta">
            <span>{{ text.difficultyLabels[openPathView.difficulty] }}</span>
            <span>{{ text.pathLessonsLabel(openPathView.totalLessons) }}</span>
            <span>{{ text.pathMinutesLabel(openPathView.totalMinutes) }}</span>
            <span>
              {{
                text.pathProgressLabel(
                  openPathView.completedLessons,
                  openPathView.totalLessons,
                )
              }}
            </span>
          </p>
        </header>

        <h2 class="section-heading">{{ text.lessonsHeading }}</h2>

        <ul class="lesson-list">
          <li v-for="(lesson, index) in openPathView.lessons" :key="lesson.id">
            <button
              type="button"
              class="lesson-row"
              :disabled="busy"
              @click="openLesson(openPathView.id, lesson.id)"
            >
              <span class="lesson-index" aria-hidden="true">{{ index + 1 }}</span>

              <span class="lesson-text">
                <span class="lesson-name">{{ lesson.title }}</span>
                <span class="lesson-summary">{{ lesson.summary }}</span>
              </span>

              <span class="lesson-state">
                <span v-if="lesson.completed" class="state complete">
                  {{ text.lessonCompletedLabel }}
                </span>
                <span v-else-if="lesson.answeredCount > 0" class="state started">
                  {{ text.lessonStartedLabel }}
                </span>
                <span v-else-if="lesson.minutes" class="state minutes">
                  {{ text.lessonMinutesLabel(lesson.minutes) }}
                </span>
                <span class="state-count">
                  {{ text.lessonChecksLabel(lesson.checkCount) }}
                </span>
              </span>
            </button>
          </li>
        </ul>

        <p
          v-if="openPathView.completedLessons === openPathView.totalLessons"
          class="path-complete"
          role="status"
        >
          {{ text.courseCompleteLabel }}
        </p>
      </template>
    </template>

    <template v-else>
      <header class="page-header">
        <p class="eyebrow">{{ text.dashboardEyebrow }}</p>

        <h1 class="page-title">
          {{ hasProgress ? text.returningTitleLine : text.welcomeTitleLine }}
          <span class="accent">
            {{ hasProgress ? text.returningTitleAccent : text.welcomeTitleAccent }}
          </span>
        </h1>

        <p class="page-description">
          {{ hasProgress ? text.returningDescription : text.welcomeDescription }}
        </p>
      </header>

      <section v-if="snapshot.resume && hasProgress" class="resume">
        <p class="resume-eyebrow">{{ text.resumeEyebrow }}</p>
        <p class="resume-lesson">
          {{ text.resumeLabel(snapshot.resume.lessonTitle, snapshot.resume.pathTitle) }}
        </p>
        <button type="button" class="primary" :disabled="busy" @click="continueLearning">
          {{ text.resumeButton }}
          <span aria-hidden="true">→</span>
        </button>
      </section>

      <h2 class="section-heading">{{ text.pathsHeading }}</h2>

      <p v-if="snapshot.paths.length === 0" class="empty">{{ text.emptyStateLabel }}</p>

      <ul class="path-list">
        <li v-for="path in snapshot.paths" :key="path.id">
          <button
            type="button"
            class="path-card"
            :disabled="busy"
            @click="openPath(path.id)"
          >
            <span class="path-subject">{{ path.subject }}</span>
            <span class="path-title">{{ path.title }}</span>
            <span class="path-summary">{{ path.summary }}</span>
            <span class="path-meta">
              <span>{{ text.difficultyLabels[path.difficulty] }}</span>
              <span>
                {{ text.pathProgressLabel(path.completedLessons, path.totalLessons) }}
              </span>
              <span>{{ text.pathMinutesLabel(path.totalMinutes) }}</span>
            </span>
          </button>
        </li>
      </ul>
    </template>
  </div>
</template>

<style scoped>
.learning {
  width: 100%;
  max-width: 880px;
  margin: 0 auto;
  padding: 34px 0 60px;
}

.warning {
  margin: 0 0 20px;
  padding: 12px 16px;
  border: 1px solid rgb(198 183 122 / 35%);
  border-radius: 9px;
  background: rgb(198 183 122 / 8%);
  color: #e0d3a4;
  font-size: 13px;
  line-height: 1.6;
}

.error {
  margin: 0 0 20px;
  padding: 12px 16px;
  border: 1px solid rgb(255 133 133 / 30%);
  border-radius: 9px;
  background: rgb(255 133 133 / 7%);
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

.back {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 26px;
  padding: 9px 15px;
  border: 1px solid #37394d;
  border-radius: 9px;
  background: #151722;
  color: #d9d8ff;
  cursor: pointer;
  font-size: 12px;
  font-weight: 650;
}

.back:disabled {
  cursor: progress;
  opacity: 0.6;
}

.page-header {
  padding-bottom: 26px;
}

.eyebrow {
  margin: 0;
  color: #b4b0ff;
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 2px;
  text-transform: uppercase;
}

.page-title {
  margin: 18px 0 14px;
  font-size: clamp(32px, 4.6vw, 46px);
  font-weight: 650;
  letter-spacing: -2px;
  line-height: 1.08;
}

.page-title .accent {
  display: block;
  color: #aaa4ff;
}

.page-description {
  max-width: 62ch;
  margin: 0;
  color: #a6a9bb;
  font-size: 15px;
  line-height: 1.8;
}

.page-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 8px 16px;
  margin: 20px 0 0;
  color: #8d91a8;
  font-size: 12px;
}

.section-heading {
  margin: 34px 0 16px;
  color: #8d91a8;
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 2px;
  text-transform: uppercase;
}

.resume {
  margin-top: 8px;
  padding: 22px;
  border: 1px solid #37394d;
  border-radius: 13px;
  background:
    radial-gradient(ellipse at 90% 10%, rgb(113 103 219 / 18%), transparent 60%),
    #12141e;
}

.resume-eyebrow {
  margin: 0 0 8px;
  color: #b4b0ff;
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 1.8px;
}

.resume-lesson {
  margin: 0 0 16px;
  color: #f4f5fb;
  font-size: 17px;
  font-weight: 650;
  letter-spacing: -0.4px;
  line-height: 1.4;
}

.path-list,
.lesson-list {
  margin: 0;
  padding: 0;
  list-style: none;
}

.path-card {
  display: flex;
  width: 100%;
  flex-direction: column;
  gap: 9px;
  margin-bottom: 12px;
  padding: 20px 22px;
  border: 1px solid #2b2d3d;
  border-radius: 13px;
  background: #12141e;
  color: inherit;
  cursor: pointer;
  text-align: left;
  transition:
    border-color 160ms ease,
    background 160ms ease,
    transform 160ms ease;
}

.path-card:hover {
  border-color: #4d4a86;
  background: #161924;
  transform: translateY(-1px);
}

.path-card:disabled,
.lesson-row:disabled {
  cursor: progress;
  opacity: 0.65;
  transform: none;
}

.path-subject {
  color: #8d91a8;
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 1.6px;
  text-transform: uppercase;
}

.path-title {
  color: #f4f5fb;
  font-size: 21px;
  font-weight: 650;
  letter-spacing: -0.6px;
}

.path-summary {
  max-width: 70ch;
  color: #a6a9bb;
  font-size: 14px;
  line-height: 1.7;
}

.path-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 8px 16px;
  color: #777b90;
  font-size: 12px;
}

.lesson-row {
  display: flex;
  width: 100%;
  align-items: flex-start;
  gap: 14px;
  margin-bottom: 10px;
  padding: 16px 18px;
  border: 1px solid #2b2d3d;
  border-radius: 11px;
  background: #12141e;
  color: inherit;
  cursor: pointer;
  text-align: left;
}

.lesson-row:hover {
  border-color: #4d4a86;
  background: #161924;
}

.lesson-index {
  display: grid;
  width: 26px;
  height: 26px;
  flex: 0 0 auto;
  place-items: center;
  border: 1px solid #37394d;
  border-radius: 8px;
  color: #b4b0ff;
  font-size: 12px;
  font-weight: 700;
}

.lesson-text {
  display: flex;
  flex: 1 1 auto;
  flex-direction: column;
  gap: 4px;
}

.lesson-name {
  color: #f4f5fb;
  font-size: 15px;
  font-weight: 650;
  letter-spacing: -0.2px;
}

.lesson-summary {
  color: #a6a9bb;
  font-size: 13px;
  line-height: 1.6;
}

.lesson-state {
  display: flex;
  flex: 0 0 auto;
  flex-direction: column;
  align-items: flex-end;
  gap: 5px;
  color: #777b90;
  font-size: 11px;
  text-align: right;
}

.state.complete {
  color: #8fe3b6;
  font-weight: 650;
}

.state.started {
  color: #d9d5a0;
  font-weight: 650;
}

.state-count {
  color: #6f7387;
}

.path-complete {
  margin: 18px 0 0;
  color: #8fe3b6;
  font-size: 13px;
}

.empty {
  color: #8d91a8;
  font-size: 14px;
}

.primary {
  display: inline-flex;
  min-height: 46px;
  align-items: center;
  gap: 18px;
  padding: 0 20px;
  border: 1px solid transparent;
  border-radius: 9px;
  background: linear-gradient(110deg, #7067db, #5b54b7);
  color: white;
  cursor: pointer;
  font-size: 13px;
  font-weight: 650;
}

.primary:disabled {
  cursor: progress;
  opacity: 0.6;
}

.primary span {
  font-size: 17px;
}

@media (max-width: 640px) {
  .lesson-row {
    flex-wrap: wrap;
  }

  .lesson-state {
    flex-direction: row;
    align-items: center;
    gap: 10px;
  }
}
</style>
