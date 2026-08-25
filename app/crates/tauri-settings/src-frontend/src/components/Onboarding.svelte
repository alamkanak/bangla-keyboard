<script>
  import Button from './Button.svelte';
  import StepIndicator from './StepIndicator.svelte';
  import RadioCardGroup from './RadioCardGroup.svelte';
  import KeyboardViewer from './KeyboardViewer.svelte';
  import { t, setLocale, getLocale } from '../lib/i18n.js';
  import { Moon, Sun, Lightbulb } from 'phosphor-svelte';

  let { oncomplete } = $props();

  let step = $state(0);
  let language = $state('en');
  let layout = $state('phonetic');
  let theme = $state('dark');
  let imeEnabled = $state(false);

  const totalSteps = 4;

  const steps = $derived([
    t('onboarding.step.language'),
    t('onboarding.step.enable'),
    t('onboarding.step.layout'),
    t('onboarding.step.theme'),
  ]);

  const languageOptions = [
    { value: 'en', label: 'English', description: 'Use English for the app interface' },
    { value: 'bn', label: 'বাংলা', description: 'অ্যাপ ইন্টারফেসের জন্য বাংলা ব্যবহার করুন' },
  ];

  const layoutOptions = $derived([
    { value: 'phonetic', label: t('onboarding.layout.phonetic'), description: t('onboarding.layout.phonetic.desc') },
    { value: 'unibijoy', label: t('onboarding.layout.unibijoy'), description: t('onboarding.layout.unibijoy.desc') },
    { value: 'national', label: t('onboarding.layout.national'), description: t('onboarding.layout.national.desc') },
  ]);

  const themeOptions = $derived([
    { value: 'dark', label: t('onboarding.theme.dark'), description: t('onboarding.theme.dark.desc'), icon: Moon },
    { value: 'light', label: t('onboarding.theme.light'), description: t('onboarding.theme.light.desc'), icon: Sun },
  ]);

  const isMac = $derived(navigator.platform?.toLowerCase().includes('mac') ?? false);

  function onLanguageSelect(value) {
    language = value;
    setLocale(value);
  }

  function onThemeSelect(value) {
    theme = value;
    if (value === 'light') {
      document.documentElement.setAttribute('data-theme', 'light');
    } else {
      document.documentElement.removeAttribute('data-theme');
    }
  }

  async function enableIME() {
    try {
      if (window.__TAURI_INTERNALS__) {
        await window.__TAURI_INTERNALS__.invoke('enable_ime');
      }
      imeEnabled = true;
    } catch (e) {
      console.error('Failed to enable IME:', e);
      imeEnabled = true; // Mark as done so user can proceed
    }
  }

  function next() {
    if (step < totalSteps - 1) {
      step++;
    } else {
      oncomplete({ language, layout, theme });
    }
  }

  function back() {
    if (step > 0) step--;
  }
</script>

<div class="onboarding">
  <div class="onboarding-header">
    <h1 class="onboarding-title">{t('onboarding.title')}</h1>
    <p class="onboarding-subtitle">{t('onboarding.subtitle')}</p>
  </div>

  <div class="steps-bar">
    {#each steps as label, i}
      {#if i > 0}
        <div class="step-connector" class:active={i <= step}></div>
      {/if}
      <StepIndicator
        stepNumber={i + 1}
        {label}
        active={i === step}
        completed={i < step}
      />
    {/each}
  </div>

  <div class="step-content">
    {#if step === 0}
      <div class="step-page">
        <h2 class="step-title">{t('onboarding.language.title')}</h2>
        <p class="step-desc">{t('onboarding.language.desc')}</p>
        <RadioCardGroup
          options={languageOptions}
          value={language}
          onselect={onLanguageSelect}
        />
      </div>

    {:else if step === 1}
      <div class="step-page">
        <h2 class="step-title">{t('onboarding.enable.title')}</h2>
        <p class="step-desc">{t('onboarding.enable.desc')}</p>

        <div class="enable-action">
          <Button variant={imeEnabled ? 'secondary' : 'primary'} size="lg" onclick={enableIME}>
            {imeEnabled ? t('onboarding.enable.enabled') : t('onboarding.enable.button')}
          </Button>
        </div>

        <details class="manual-instructions">
          <summary class="manual-summary">{t('onboarding.enable.manual')}</summary>
          <div class="instruction-card">
            {#if isMac}
              <ol class="instruction-list">
                <li>{t('onboarding.enable.mac.step1')}</li>
                <li>{t('onboarding.enable.mac.step2')}</li>
                <li>{t('onboarding.enable.mac.step3')}</li>
                <li>{t('onboarding.enable.mac.step4')}</li>
              </ol>
            {:else}
              <ol class="instruction-list">
                <li>{t('onboarding.enable.win.step1')}</li>
                <li>{t('onboarding.enable.win.step2')}</li>
                <li>{t('onboarding.enable.win.step3')}</li>
                <li>{t('onboarding.enable.win.step4')}</li>
              </ol>
            {/if}
          </div>
        </details>

        <div class="switch-hint">
          <div class="hint-icon"><Lightbulb size={18} weight="fill" /></div>
          <p class="hint-text">{t('onboarding.enable.switchHint')}</p>
        </div>
      </div>

    {:else if step === 2}
      <div class="step-page">
        <h2 class="step-title">{t('onboarding.layout.title')}</h2>
        <p class="step-desc">{t('onboarding.layout.desc')}</p>
        <RadioCardGroup
          options={layoutOptions}
          value={layout}
          onselect={(v) => layout = v}
        />
        {#if layout === 'unibijoy' || layout === 'national'}
          <div class="layout-preview">
            <KeyboardViewer layout={layout} shift={false} />
          </div>
        {/if}
      </div>

    {:else if step === 3}
      <div class="step-page">
        <h2 class="step-title">{t('onboarding.theme.title')}</h2>
        <p class="step-desc">{t('onboarding.theme.desc')}</p>
        <RadioCardGroup
          options={themeOptions}
          value={theme}
          onselect={onThemeSelect}
        />
      </div>
    {/if}
  </div>

  <div class="onboarding-footer">
    {#if step > 0}
      <Button variant="ghost" onclick={back}>
        {t('onboarding.back')}
      </Button>
    {:else}
      <div></div>
    {/if}

    <Button variant="primary" size="lg" onclick={next}>
      {step < totalSteps - 1 ? t('onboarding.next') : t('onboarding.finish')}
    </Button>
  </div>
</div>

<style>
  .onboarding {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
    background: var(--bg);
    overflow: hidden;
  }

  .onboarding-header {
    text-align: center;
    padding: var(--space-xl) var(--space-2xl) var(--space-md);
  }

  .onboarding-title {
    font-size: 24px;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.02em;
    margin-bottom: var(--space-xs);
  }

  .onboarding-subtitle {
    font-size: 14px;
    color: var(--text-secondary);
  }

  .steps-bar {
    display: flex;
    align-items: flex-start;
    justify-content: center;
    gap: 0;
    padding: 0 var(--space-3xl) var(--space-lg);
    max-width: 520px;
    margin: 0 auto;
    width: 100%;
  }

  .step-connector {
    width: 40px;
    height: 2px;
    background: var(--border);
    margin-top: 15px;
    transition: background-color 0.2s;
  }

  .step-connector.active {
    background: var(--success);
  }

  .step-content {
    flex: 1;
    overflow-y: auto;
    display: flex;
    justify-content: center;
    padding-top: var(--space-sm);
  }

  .step-page {
    max-width: 520px;
    width: 100%;
    padding: 0 var(--space-xl) var(--space-2xl);
  }

  .step-title {
    font-size: 20px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: var(--space-xs);
  }

  .step-desc {
    font-size: 14px;
    color: var(--text-secondary);
    margin-bottom: var(--space-xl);
    line-height: 1.5;
  }

  .enable-action {
    display: flex;
    justify-content: center;
    margin-bottom: var(--space-xl);
  }

  .manual-instructions {
    margin-bottom: var(--space-lg);
  }

  .manual-summary {
    font-size: 13px;
    color: var(--text-tertiary);
    cursor: pointer;
    margin-bottom: var(--space-sm);
    user-select: none;
  }

  .manual-summary:hover {
    color: var(--text-secondary);
  }

  .instruction-card {
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: var(--space-xl);
    margin-top: var(--space-sm);
  }

  .instruction-list {
    list-style: none;
    counter-reset: step;
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
    padding: 0;
  }

  .instruction-list li {
    counter-increment: step;
    display: flex;
    align-items: flex-start;
    gap: var(--space-md);
    font-size: 14px;
    color: var(--text-primary);
    line-height: 1.5;
  }

  .instruction-list li::before {
    content: counter(step);
    min-width: 24px;
    height: 24px;
    border-radius: 50%;
    background: var(--accent-subtle);
    color: var(--accent);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    font-weight: 600;
  }

  .switch-hint {
    display: flex;
    align-items: flex-start;
    gap: var(--space-md);
    padding: var(--space-lg);
    background: var(--info-subtle);
    border-radius: var(--radius-md);
    border: 1px solid rgba(86, 194, 255, 0.2);
  }

  .hint-icon {
    display: flex;
    align-items: center;
    color: var(--info);
    line-height: 1;
  }

  .hint-text {
    font-size: 13px;
    color: var(--text-secondary);
    line-height: 1.5;
  }

  .layout-preview {
    margin-top: var(--space-lg);
  }

  .onboarding-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-lg) var(--space-2xl);
    border-top: 1px solid var(--separator);
  }
</style>
