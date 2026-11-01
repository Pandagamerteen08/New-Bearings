
<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { welcomeContent } from '../../content/interface/welcome'
import { setupContent } from '../../content/interface/setup'
import { learningContent } from '../../content/interface/learning'
import {
  beginSetup,
  getSetupState,
  returnToWelcome,
  startLearning,
  toSetupErrorCode,
  type SetupErrorCode,
  type SetupSnapshot,
} from './backend/setup'
import { getLearningState, type LearningSnapshot } from './backend/learning'
import LearningView from './components/LearningView.vue'

// Rust owns the setup stage. This placeholder mirrors the backend's starting
// stage so the welcome screen renders before the first snapshot arrives, and
// keeps the application usable if the backend cannot be reached at all.
const setup = ref<SetupSnapshot>({
  stage: 'welcome',
  canBegin: true,
  canReturnToWelcome: false,
  canStartLearning: false,
})

// The learning screens are rendered from this snapshot, which Rust builds.
const learning = ref<LearningSnapshot | null>(null)
const learningError = ref('')
const learningErrorDetails = ref('')
const isLoadingLearning = ref(false)

const appInfo = ref('')
const backendError = ref('')
const isCheckingBackend = ref(true)
const setupError = ref('')
const setupErrorDetails = ref('')
const isTransitioning = ref(false)

function setupErrorMessage(code: SetupErrorCode | null): string | null {
  switch (code) {
    case 'already_started':
      return setupContent.errorAlreadyStarted
    case 'already_at_welcome':
      return setupContent.errorAlreadyAtWelcome
    case 'already_learning':
      return setupContent.errorAlreadyLearning
    case 'not_at_introduction':
      return setupContent.errorNotAtIntroduction
    default:
      return null
  }
}

function showSetupError(error: unknown, fallbackMessage: string) {
  setupError.value = setupErrorMessage(toSetupErrorCode(error)) ?? fallbackMessage
  setupErrorDetails.value = String(error)
}

async function loadAppInfo() {
  try {
    appInfo.value = await invoke<string>('get_app_info')
  } catch (error) {
    backendError.value = String(error)
  } finally {
    isCheckingBackend.value = false
  }
}

async function loadLearningState() {
  isLoadingLearning.value = true
  learningError.value = ''
  learningErrorDetails.value = ''

  try {
    learning.value = await getLearningState()
  } catch (error) {
    learningError.value = learningContent.errorLearningUnavailable
    learningErrorDetails.value = String(error)
  } finally {
    isLoadingLearning.value = false
  }
}

async function loadSetupState() {
  try {
    setup.value = await getSetupState()

    // Rust decides whether onboarding is finished; a returning learner goes
    // straight to the learning screens.
    if (setup.value.stage === 'learning') {
      await loadLearningState()
    }
  } catch (error) {
    showSetupError(error, setupContent.errorUnavailable)
  }
}

async function enterLearning() {
  if (isTransitioning.value) {
    return
  }

  isTransitioning.value = true
  setupError.value = ''
  setupErrorDetails.value = ''

  try {
    setup.value = await startLearning()
    await loadLearningState()
  } catch (error) {
    showSetupError(error, setupContent.errorUnexpected)
  } finally {
    isTransitioning.value = false
  }
}

function updateLearning(snapshot: LearningSnapshot) {
  learning.value = snapshot
  learningError.value = ''
  learningErrorDetails.value = ''
}

async function runSetupCommand(
  command: () => Promise<SetupSnapshot>,
  failedMessage: string,
) {
  if (isTransitioning.value) {
    return
  }

  isTransitioning.value = true
  setupError.value = ''
  setupErrorDetails.value = ''

  try {
    setup.value = await command()
  } catch (error) {
    showSetupError(error, failedMessage)
  } finally {
    isTransitioning.value = false
  }
}

function startSetup() {
  void runSetupCommand(beginSetup, setupContent.errorUnexpected)
}

function goBackToWelcome() {
  void runSetupCommand(returnToWelcome, setupContent.errorUnexpected)
}

onMounted(() => {
  void loadAppInfo()
  void loadSetupState()
})
</script>

<template>
  <main class="welcome">
    <header class="topbar">
      <a class="brand" href="#" aria-label="New Bearings home">
        <span class="brand-mark" aria-hidden="true">N</span>
        <span>{{ welcomeContent.brandName }}</span>
      </a>

      <span class="version-label">{{ welcomeContent.accessLabel }}</span>
    </header>

    <section v-if="setup.stage === 'welcome'" class="hero">
      <div class="eyebrow">
        <span class="status-dot"></span>
        {{ welcomeContent.eyebrow }}
      </div>

      <h1>
        {{ welcomeContent.titleLine }}
        <span class="accent-text">{{ welcomeContent.titleAccent }}</span>
      </h1>

      <p class="description">
        {{ welcomeContent.description }}
      </p>

      <button
        class="primary-button"
        :disabled="isTransitioning || !setup.canBegin"
        :aria-busy="isTransitioning"
        @click="startSetup"
      >
        {{ welcomeContent.getStartedButton }}
        <span aria-hidden="true">→</span>
      </button>

      <div v-if="setupError" class="setup-error" role="alert">
        <p>{{ setupError }}</p>
        <details class="error-details">
          <summary>{{ setupContent.errorDetailsLabel }}</summary>
          <pre>{{ setupErrorDetails }}</pre>
        </details>
      </div>

      <p class="footnote">
        {{ welcomeContent.footnote }}
      </p>
    </section>

    <section v-else-if="setup.stage === 'introduction'" class="hero">
      <div class="eyebrow">
        <span class="status-dot"></span>
        {{ welcomeContent.nextEyebrow }}
      </div>

      <h1>
        {{ welcomeContent.nextTitleLine }}
        <span class="accent-text">{{ welcomeContent.nextTitleAccent }}</span>
      </h1>

      <p class="description">
        {{ welcomeContent.nextDescription }}
      </p>

      <div class="button-row">
        <button
          class="primary-button"
          :disabled="isTransitioning || !setup.canStartLearning"
          :aria-busy="isTransitioning"
          @click="enterLearning"
        >
          {{ learningContent.startLearningButton }}
          <span aria-hidden="true">→</span>
        </button>

        <button
          class="secondary-button"
          :disabled="isTransitioning || !setup.canReturnToWelcome"
          @click="goBackToWelcome"
        >
          <span aria-hidden="true">←</span>
          {{ welcomeContent.backButton }}
        </button>
      </div>
    </section>

    <div v-else class="learning-shell">
      <p v-if="isLoadingLearning" class="learning-status" role="status">
        {{ learningContent.loadingLabel }}
      </p>

      <div v-else-if="learningError" class="setup-error" role="alert">
        <p>{{ learningError }}</p>
        <details class="error-details">
          <summary>{{ learningContent.errorDetailsLabel }}</summary>
          <pre>{{ learningErrorDetails }}</pre>
        </details>
      </div>

      <LearningView
        v-else-if="learning"
        :snapshot="learning"
        @snapshot="updateLearning"
      />
    </div>

    <footer class="footer">
      <span>{{ welcomeContent.footerMessage }}</span>
      <span>{{ welcomeContent.brandName }}</span>
    </footer>

    <aside class="backend-status" role="status" aria-live="polite">
      <template v-if="isCheckingBackend">
        <span class="connection-indicator checking"></span>
        <span>{{ welcomeContent.backendCheckingLabel }}</span>
      </template>

      <template v-else-if="appInfo">
        <span class="connection-indicator connected"></span>
        <span>{{ welcomeContent.backendConnectedLabel }}: {{ appInfo }}</span>
      </template>

      <template v-else>
        <span class="connection-indicator failed"></span>
        <span>{{ welcomeContent.backendFailedLabel }}</span>
        <details class="error-details">
          <summary>{{ welcomeContent.backendErrorDetailsLabel }}</summary>
          <pre>{{ backendError }}</pre>
        </details>
      </template>
    </aside>
  </main>
</template>

<style>
:root {
  font-family: Inter, "Segoe UI", sans-serif;
  color: #f4f5fb;
  background: #090b12;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  font-optical-sizing: auto;
  font-weight: 400;
}

* {
  box-sizing: border-box;
}

body {
  min-width: 320px;
  min-height: 100vh;
  margin: 0;
}

button,
a {
  font: inherit;
}

button:focus-visible,
a:focus-visible,
summary:focus-visible {
  outline: 2px solid #a5a2ff;
  outline-offset: 4px;
}

.welcome {
  position: relative;
  display: flex;
  min-height: 100vh;
  flex-direction: column;
  overflow: hidden;
  isolation: isolate;
  padding: 30px clamp(24px, 7vw, 100px);
  background:
    radial-gradient(ellipse at 75% 35%, rgb(85 70 180 / 16%), transparent 48%),
    radial-gradient(ellipse at 20% 90%, rgb(38 110 160 / 10%), transparent 42%),
    #090b12;
}

.welcome::before {
  position: absolute;
  z-index: -1;
  inset: 0;
  background-image:
    linear-gradient(rgb(255 255 255 / 2.5%) 1px, transparent 1px),
    linear-gradient(90deg, rgb(255 255 255 / 2.5%) 1px, transparent 1px);
  background-size: 52px 52px;
  content: "";
  mask-image: linear-gradient(to bottom, black, transparent 85%);
  pointer-events: none;
}

.topbar,
.brand,
.footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.topbar {
  gap: 20px;
}

.brand {
  justify-content: flex-start;
  gap: 12px;
  color: #f4f5fb;
  font-size: 16px;
  font-weight: 650;
  letter-spacing: -0.4px;
  text-decoration: none;
}

.brand-mark {
  display: grid;
  width: 36px;
  height: 36px;
  place-items: center;
  border: 1px solid rgb(173 165 255 / 40%);
  border-radius: 11px;
  background: linear-gradient(145deg, #7167db, #39316e);
  box-shadow: 0 0 28px rgb(113 103 219 / 22%);
  font-size: 19px;
  font-weight: 750;
}

.version-label {
  color: #a6a9bd;
  font-size: 10px;
  font-weight: 650;
  letter-spacing: 1.8px;
}

.hero {
  width: min(100%, 690px);
  margin: auto 0;
  padding: 90px 0;
  animation: appear 350ms ease-out both;
}

.eyebrow {
  display: flex;
  align-items: center;
  gap: 10px;
  color: #b4b0ff;
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 2px;
}

.status-dot {
  width: 7px;
  height: 7px;
  flex: 0 0 auto;
  border-radius: 50%;
  background: #a29aff;
  box-shadow: 0 0 12px rgb(162 154 255 / 70%);
}

h1 {
  max-width: 680px;
  margin: 25px 0;
  font-size: clamp(46px, 7.4vw, 82px);
  font-weight: 650;
  letter-spacing: -5px;
  line-height: 1.04;
}

.accent-text {
  display: block;
  color: #aaa4ff;
}

.description {
  max-width: 530px;
  margin: 0 0 34px;
  color: #a6a9bb;
  font-size: 16px;
  line-height: 1.85;
}

.primary-button,
.secondary-button {
  display: inline-flex;
  min-height: 50px;
  align-items: center;
  justify-content: center;
  gap: 22px;
  padding: 0 23px;
  border: 1px solid transparent;
  border-radius: 9px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 650;
  transition:
    transform 160ms ease,
    background 160ms ease,
    border-color 160ms ease;
}

.primary-button {
  color: white;
  background: linear-gradient(110deg, #7067db, #5b54b7);
  box-shadow: 0 7px 30px rgb(90 80 190 / 20%);
}

.primary-button:hover {
  transform: translateY(-2px);
  background: linear-gradient(110deg, #8178eb, #6a62ce);
}

.primary-button:disabled,
.secondary-button:disabled {
  cursor: progress;
  opacity: 0.6;
  transform: none;
}

.primary-button span,
.secondary-button span {
  font-size: 19px;
}

.secondary-button {
  color: #eeedff;
  border-color: #37394d;
  background: #151722;
}

.secondary-button:hover {
  border-color: #7770ca;
  background: #1b1d2b;
}

.footnote {
  margin-top: 20px;
  color: #777b90;
  font-size: 12px;
}

.button-row {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 14px;
}

.learning-shell {
  display: flex;
  flex: 1 1 auto;
  flex-direction: column;
  justify-content: center;
}

.learning-status {
  margin: 0;
  color: #a6a9bb;
  font-size: 14px;
}

.setup-error {
  max-width: 530px;
  margin-top: 22px;
  color: #ffb4b4;
  font-size: 13px;
  line-height: 1.7;
}

.setup-error p {
  margin: 0;
}

.setup-error .error-details {
  margin-left: 0;
}

.footer {
  gap: 20px;
  padding-top: 22px;
  border-top: 1px solid rgb(255 255 255 / 8%);
  color: #777b90;
  font-size: 11px;
}

.backend-status {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 9px;
  margin-top: 18px;
  color: #9296ac;
  font-size: 11px;
  line-height: 1.6;
}

.connection-indicator {
  width: 7px;
  height: 7px;
  flex: 0 0 auto;
  border-radius: 50%;
}

.connection-indicator.checking {
  background: #c6b77a;
}

.connection-indicator.connected {
  background: #72d6a0;
  box-shadow: 0 0 10px rgb(114 214 160 / 30%);
}

.connection-indicator.failed {
  background: #ff8585;
}

.error-details {
  flex-basis: 100%;
  margin-left: 16px;
  color: #c2a0a0;
}

.error-details summary {
  width: fit-content;
  cursor: pointer;
}

.error-details pre {
  overflow-wrap: anywhere;
  white-space: pre-wrap;
  color: #d0b6b6;
  font-family: Consolas, monospace;
  font-size: 11px;
}

@keyframes appear {
  from {
    opacity: 0;
    transform: translateY(8px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    scroll-behavior: auto !important;
    transition-duration: 0.01ms !important;
  }
}

@media (max-width: 600px) {
  .welcome {
    padding: 22px;
  }

  .version-label {
    font-size: 9px;
    letter-spacing: 1px;
  }

  .hero {
    padding: 65px 0;
  }

  h1 {
    letter-spacing: -2.5px;
  }

  .description {
    font-size: 15px;
  }

  .footer {
    flex-wrap: wrap;
  }
}
</style>
