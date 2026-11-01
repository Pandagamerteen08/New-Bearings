<script setup lang="ts">
import { computed, ref } from 'vue'
import { learningContent as text } from '../../../content/interface/learning'
import {
  goBackInLearning,
  openLearningLesson,
  openLearningPath,
  searchLearning,
  showLearningDashboard,
  showLearningReview,
  toggleLessonBookmark,
  toLearningErrorCode,
  type LearningSnapshot,
  type SearchHit,
} from '../backend/learning'
import LessonPane from './LessonPane.vue'

const props = defineProps<{ snapshot: LearningSnapshot }>()

const emit = defineEmits<{
  (event: 'snapshot', snapshot: LearningSnapshot): void
}>()

const busy = ref(false)
const error = ref('')
const errorDetails = ref('')

const searchTerm = ref('')
const searchHits = ref<SearchHit[] | null>(null)
const isSearching = ref(false)

const screen = computed(() => props.snapshot.screen)

// A learner who has answered something gets the returning wording, so the
// dashboard never pretends progress exists before there is any.
const hasProgress = computed(() => props.snapshot.summary.checksSolved > 0)
const reviewCount = computed(() => props.snapshot.review.length)

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
    case 'answer_mismatch':
      return text.errorAnswerMismatch
    case 'empty_answer':
      return text.errorEmptyAnswer
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
  searchHits.value = null
  void run(() => openLearningPath(pathId), text.errorUnexpected)
}

function openLesson(pathId: string, lessonId: string) {
  searchHits.value = null
  void run(() => openLearningLesson(pathId, lessonId), text.errorUnexpected)
}

function goBack() {
  void run(goBackInLearning, text.errorUnexpected)
}

function goToDashboard() {
  void run(showLearningDashboard, text.errorUnexpected)
}

function goToReview() {
  void run(showLearningReview, text.errorUnexpected)
}

function toggleBookmark(pathId: string, lessonId: string) {
  void run(() => toggleLessonBookmark(pathId, lessonId), text.errorUnexpected)
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

async function runSearch() {
  const query = searchTerm.value.trim()

  if (!query || isSearching.value) {
    if (!query) {
      searchHits.value = null
    }

    return
  }

  isSearching.value = true
  error.value = ''
  errorDetails.value = ''

  try {
    searchHits.value = await searchLearning(query)
  } catch (failure) {
    error.value = text.errorSearchUnavailable
    errorDetails.value = String(failure)
  } finally {
    isSearching.value = false
  }
}

function clearSearch() {
  searchTerm.value = ''
  searchHits.value = null
}

function openHit(hit: SearchHit) {
  if (hit.lessonId) {
    openLesson(hit.pathId, hit.lessonId)
  } else {
    openPath(hit.pathId)
  }
}

function pathPercent(completed: number, total: number): string {
  if (total === 0) {
    return '0%'
  }

  return `${Math.round((completed / total) * 100)}%`
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

    <nav v-if="screen.kind !== 'lesson'" class="tabs" aria-label="Learning">
      <button
        type="button"
        class="tab"
        :class="{ active: screen.kind === 'dashboard' || screen.kind === 'path' }"
        :aria-current="
          screen.kind === 'dashboard' || screen.kind === 'path' ? 'page' : undefined
        "
        :disabled="busy"
        @click="goToDashboard"
      >
        {{ text.dashboardTabLabel }}
      </button>

      <button
        type="button"
        class="tab"
        :class="{ active: screen.kind === 'review' }"
        :aria-current="screen.kind === 'review' ? 'page' : undefined"
        :disabled="busy"
        @click="goToReview"
      >
        {{ text.reviewTabLabel(reviewCount) }}
      </button>
    </nav>

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

          <p v-if="openPathView.tags.length" class="tag-row">
            <span v-for="tag in openPathView.tags" :key="tag" class="tag">{{ tag }}</span>
          </p>
        </header>

        <h2 class="section-heading">{{ text.lessonsHeading }}</h2>

        <ul class="lesson-list">
          <li v-for="(lesson, index) in openPathView.lessons" :key="lesson.id" class="lesson-row">
            <button
              type="button"
              class="lesson-open"
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
                <span
                  v-else-if="lesson.solvedCount > 0 || lesson.attemptCount > 0"
                  class="state started"
                >
                  {{ text.lessonStartedLabel }}
                </span>
                <span v-else-if="lesson.minutes" class="state minutes">
                  {{ text.lessonMinutesLabel(lesson.minutes) }}
                </span>

                <span v-if="lesson.mistakes > 0" class="state review">
                  {{ text.lessonMistakesLabel(lesson.mistakes) }}
                </span>

                <span class="state-count">
                  {{ text.lessonChecksLabel(lesson.checkCount) }}
                </span>
              </span>
            </button>

            <button
              type="button"
              class="bookmark"
              :class="{ on: lesson.bookmarked }"
              :disabled="busy"
              :aria-pressed="lesson.bookmarked"
              :aria-label="
                lesson.bookmarked ? text.bookmarkRemoveLabel : text.bookmarkAddLabel
              "
              @click="toggleBookmark(openPathView.id, lesson.id)"
            >
              <span aria-hidden="true">{{ lesson.bookmarked ? '★' : '☆' }}</span>
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

    <template v-else-if="screen.kind === 'review'">
      <button type="button" class="back" :disabled="busy" @click="goToDashboard">
        <span aria-hidden="true">←</span>
        {{ text.reviewBackLabel }}
      </button>

      <header class="page-header">
        <p class="eyebrow">{{ text.reviewEyebrow }}</p>

        <h1 class="page-title">
          {{ text.reviewTitleLine }}
          <span class="accent">{{ text.reviewTitleAccent }}</span>
        </h1>

        <p class="page-description">{{ text.reviewDescription }}</p>
      </header>

      <p v-if="snapshot.review.length === 0" class="empty">{{ text.reviewEmptyLabel }}</p>

      <ul v-else class="path-list">
        <li v-for="item in snapshot.review" :key="`${item.pathId}/${item.lessonId}`">
          <div class="review-row">
            <div class="review-text">
              <p class="review-path">{{ item.pathTitle }}</p>
              <p class="review-lesson">{{ item.lessonTitle }}</p>
              <p class="review-meta">
                <span class="state review">{{ text.reviewMistakesLabel(item.mistakes) }}</span>
                <span v-if="item.completed" class="state complete">
                  {{ text.reviewCompletedHint }}
                </span>
              </p>
            </div>

            <div class="review-actions">
              <button
                type="button"
                class="primary small"
                :disabled="busy"
                @click="openLesson(item.pathId, item.lessonId)"
              >
                {{ text.reviewButton }}
              </button>

              <button
                type="button"
                class="bookmark"
                :class="{ on: item.bookmarked }"
                :disabled="busy"
                :aria-pressed="item.bookmarked"
                :aria-label="
                  item.bookmarked ? text.bookmarkRemoveLabel : text.bookmarkAddLabel
                "
                @click="toggleBookmark(item.pathId, item.lessonId)"
              >
                <span aria-hidden="true">{{ item.bookmarked ? '★' : '☆' }}</span>
              </button>
            </div>
          </div>
        </li>
      </ul>
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

      <dl v-if="hasProgress" class="stats">
        <div class="stat">
          <dt>{{ text.statsLessonsHeading }}</dt>
          <dd>
            {{
              text.statsLessonsLabel(
                snapshot.summary.lessonsCompleted,
                snapshot.summary.lessonsTotal,
              )
            }}
          </dd>
        </div>

        <div class="stat">
          <dt>{{ text.statsChecksHeading }}</dt>
          <dd>
            {{
              text.statsChecksLabel(
                snapshot.summary.checksSolved,
                snapshot.summary.checksTotal,
              )
            }}
          </dd>
        </div>

        <div class="stat">
          <dt>{{ text.statsMinutesHeading }}</dt>
          <dd>{{ text.statsMinutesLabel(snapshot.summary.minutesCompleted) }}</dd>
        </div>

        <div class="stat">
          <dt>{{ text.statsBookmarksHeading }}</dt>
          <dd>{{ text.statsBookmarksLabel(snapshot.summary.bookmarks) }}</dd>
        </div>
      </dl>

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

      <section v-if="reviewCount > 0" class="review-callout">
        <div>
          <p class="review-callout-title">{{ text.reviewEyebrow }}</p>
          <p class="review-callout-body">{{ text.reviewMistakesLabel(reviewCount) }}</p>
        </div>

        <button type="button" class="secondary" :disabled="busy" @click="goToReview">
          {{ text.reviewButton }}
        </button>
      </section>

      <form class="search" role="search" @submit.prevent="runSearch">
        <label class="search-label" for="lesson-search">{{ text.searchLabel }}</label>
        <div class="search-row">
          <input
            id="lesson-search"
            v-model="searchTerm"
            type="search"
            class="search-input"
            :placeholder="text.searchPlaceholder"
            autocomplete="off"
          />
          <button type="submit" class="primary small" :disabled="isSearching">
            {{ text.searchButton }}
          </button>
          <button
            v-if="searchHits"
            type="button"
            class="secondary small"
            @click="clearSearch"
          >
            {{ text.searchClearButton }}
          </button>
        </div>
      </form>

      <section v-if="searchHits" class="search-results" aria-live="polite">
        <h2 class="section-heading">{{ text.searchResultsHeading(searchHits.length) }}</h2>

        <p v-if="searchHits.length === 0" class="empty">{{ text.searchNoResults }}</p>

        <ul v-else class="path-list">
          <li v-for="(hit, hitIndex) in searchHits" :key="`${hit.pathId}/${hit.lessonId ?? 'path'}/${hitIndex}`">
            <button type="button" class="path-card" :disabled="busy" @click="openHit(hit)">
              <span class="path-subject">{{ text.searchMatchLabel(hit.matched) }}</span>
              <span class="path-title">{{ hit.lessonTitle ?? hit.pathTitle }}</span>
              <span class="path-summary">{{ hit.text }}</span>
              <span class="path-meta">{{ hit.pathTitle }}</span>
            </button>
          </li>
        </ul>
      </section>

      <h2 class="section-heading">{{ text.pathsHeading }}</h2>

      <p v-if="snapshot.paths.length === 0" class="empty">{{ text.emptyStateLabel }}</p>

      <ul class="path-list">
        <li v-for="path in snapshot.paths" :key="path.id">
          <button type="button" class="path-card" :disabled="busy" @click="openPath(path.id)">
            <span class="path-subject">{{ path.subject }}</span>
            <span class="path-title">{{ path.title }}</span>
            <span class="path-summary">{{ path.summary }}</span>

            <span
              class="path-bar"
              role="img"
              :aria-label="text.pathProgressLabel(path.completedLessons, path.totalLessons)"
            >
              <span
                class="path-bar-fill"
                :style="{ width: pathPercent(path.completedLessons, path.totalLessons) }"
              ></span>
            </span>

            <span class="path-meta">
              <span>{{ text.difficultyLabels[path.difficulty] }}</span>
              <span>{{ text.pathProgressLabel(path.completedLessons, path.totalLessons) }}</span>
              <span>{{ text.pathMinutesLabel(path.totalMinutes) }}</span>
              <span v-if="path.mistakes > 0">{{ text.pathMistakesLabel(path.mistakes) }}</span>
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
  padding: 24px 0 60px;
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

.tabs {
  display: flex;
  gap: 8px;
  margin-bottom: 26px;
  padding-bottom: 14px;
  border-bottom: 1px solid rgb(255 255 255 / 8%);
}

.tab {
  padding: 8px 14px;
  border: 1px solid transparent;
  border-radius: 999px;
  background: transparent;
  color: #a6a9bb;
  cursor: pointer;
  font-size: 12px;
  font-weight: 650;
}

.tab:hover {
  background: #151722;
  color: #eeedff;
}

.tab.active {
  border-color: #4d4a86;
  background: #1a1c2b;
  color: #d9d8ff;
}

.tab:disabled {
  cursor: progress;
  opacity: 0.6;
}

.back {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 22px;
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
  padding-bottom: 22px;
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
  margin: 16px 0 12px;
  font-size: clamp(30px, 4.2vw, 42px);
  font-weight: 650;
  letter-spacing: -1.8px;
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
  margin: 18px 0 0;
  color: #8d91a8;
  font-size: 12px;
}

.tag-row {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin: 14px 0 0;
}

.tag {
  padding: 3px 9px;
  border: 1px solid #32344a;
  border-radius: 999px;
  color: #9d9fc0;
  font-size: 11px;
}

.stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 10px;
  margin: 6px 0 22px;
  padding: 0;
}

.stat {
  padding: 14px 16px;
  border: 1px solid #2b2d3d;
  border-radius: 11px;
  background: #12141e;
}

.stat dt {
  margin: 0 0 5px;
  color: #777b90;
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 1.4px;
  text-transform: uppercase;
}

.stat dd {
  margin: 0;
  color: #f4f5fb;
  font-size: 15px;
  font-weight: 650;
  letter-spacing: -0.2px;
}

.section-heading {
  margin: 30px 0 14px;
  color: #8d91a8;
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 2px;
  text-transform: uppercase;
}

.resume {
  margin: 8px 0 18px;
  padding: 20px 22px;
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

.review-callout {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 14px;
  margin: 0 0 18px;
  padding: 16px 20px;
  border: 1px solid rgb(198 183 122 / 30%);
  border-radius: 11px;
  background: rgb(198 183 122 / 6%);
}

.review-callout-title {
  margin: 0 0 4px;
  color: #e0d3a4;
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 1.8px;
}

.review-callout-body {
  margin: 0;
  color: #cfc9b0;
  font-size: 13px;
}

.search {
  margin: 4px 0 8px;
}

.search-label {
  display: block;
  margin-bottom: 8px;
  color: #8d91a8;
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 1.6px;
  text-transform: uppercase;
}

.search-row {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.search-input {
  min-width: 220px;
  flex: 1 1 240px;
  padding: 0 14px;
  min-height: 44px;
  border: 1px solid #37394d;
  border-radius: 9px;
  background: #10121b;
  color: #f4f5fb;
  font: inherit;
  font-size: 14px;
}

.search-input:focus-visible {
  border-color: #7770ca;
  outline: 2px solid #a5a2ff;
  outline-offset: 2px;
}

.search-results {
  margin-bottom: 8px;
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
  padding: 18px 20px;
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
.lesson-open:disabled,
.bookmark:disabled {
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
  font-size: 20px;
  font-weight: 650;
  letter-spacing: -0.6px;
}

.path-summary {
  max-width: 70ch;
  color: #a6a9bb;
  font-size: 13.5px;
  line-height: 1.7;
}

.path-bar {
  display: block;
  width: 100%;
  height: 4px;
  overflow: hidden;
  border-radius: 999px;
  background: #232637;
}

.path-bar-fill {
  display: block;
  height: 100%;
  border-radius: 999px;
  background: linear-gradient(90deg, #7067db, #8f88f0);
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
  align-items: stretch;
  gap: 8px;
  margin-bottom: 10px;
}

.lesson-open {
  display: flex;
  width: 100%;
  flex: 1 1 auto;
  align-items: flex-start;
  gap: 14px;
  padding: 15px 16px;
  border: 1px solid #2b2d3d;
  border-radius: 11px;
  background: #12141e;
  color: inherit;
  cursor: pointer;
  text-align: left;
}

.lesson-open:hover {
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

.state.review {
  color: #e0b3a4;
}

.state-count {
  color: #6f7387;
}

.bookmark {
  flex: 0 0 auto;
  width: 42px;
  border: 1px solid #2b2d3d;
  border-radius: 11px;
  background: #12141e;
  color: #6f7387;
  cursor: pointer;
  font-size: 16px;
}

.bookmark:hover {
  border-color: #4d4a86;
  color: #d9d8ff;
}

.bookmark.on {
  border-color: rgb(198 183 122 / 45%);
  color: #e0d3a4;
}

.review-row {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 14px;
  margin-bottom: 10px;
  padding: 16px 18px;
  border: 1px solid #2b2d3d;
  border-radius: 11px;
  background: #12141e;
}

.review-text {
  min-width: 220px;
  flex: 1 1 auto;
}

.review-path {
  margin: 0 0 3px;
  color: #8d91a8;
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 1.4px;
  text-transform: uppercase;
}

.review-lesson {
  margin: 0 0 6px;
  color: #f4f5fb;
  font-size: 15px;
  font-weight: 650;
}

.review-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin: 0;
  font-size: 11px;
}

.review-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.path-complete {
  margin: 18px 0 0;
  color: #8fe3b6;
  font-size: 13px;
}

.empty {
  color: #8d91a8;
  font-size: 14px;
  line-height: 1.7;
}

.primary,
.secondary {
  display: inline-flex;
  min-height: 44px;
  align-items: center;
  gap: 16px;
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

.primary.small,
.secondary.small {
  min-height: 38px;
  padding: 0 15px;
  font-size: 12px;
}

.primary:disabled,
.secondary:disabled {
  cursor: progress;
  opacity: 0.6;
}

.primary span {
  font-size: 17px;
}

@media (max-width: 640px) {
  .lesson-open {
    flex-wrap: wrap;
  }

  .lesson-state {
    flex-direction: row;
    align-items: center;
    gap: 10px;
  }
}
</style>
