<script>
  import Sidebar from './components/Sidebar.svelte';
  import Button from './components/Button.svelte';
  import Toggle from './components/Toggle.svelte';
  import Select from './components/Select.svelte';
  import RadioCardGroup from './components/RadioCardGroup.svelte';
  import KeyboardViewer from './components/KeyboardViewer.svelte';
  import Onboarding from './components/Onboarding.svelte';
  import { t, setLocale } from './lib/i18n.js';
  import { Keyboard, Lightning, Book, Info, Palette, Moon, Sun } from 'phosphor-svelte';

  let showOnboarding = $state(true);
  let loading = $state(true);
  let activeSection = $state('layout');
  let layoutMode = $state('phonetic');
  let showKeyboard = $state(true);
  let shiftPreview = $state(false);
  let hotkeyToggle = $state('ctrl+space');
  let currentTheme = $state('dark');

  // Load preferences from backend on startup
  $effect(() => {
    loadPreferences();
  });

  async function loadPreferences() {
    try {
      if (window.__TAURI_INTERNALS__) {
        const prefs = await window.__TAURI_INTERNALS__.invoke('get_preferences');
        showOnboarding = !prefs.onboarding_complete;
        layoutMode = prefs.layout;
        setLocale(prefs.language);
        currentTheme = prefs.theme || 'dark';
        if (prefs.theme === 'light') {
          document.documentElement.setAttribute('data-theme', 'light');
        }
      } else {
        const completed = localStorage.getItem('onboarding_complete');
        showOnboarding = completed !== 'true';
      }
    } catch (e) {
      console.error('Failed to load preferences:', e);
      showOnboarding = true;
    }
    loading = false;
  }

  async function handleOnboardingComplete({ language, layout, theme }) {
    layoutMode = layout;
    try {
      if (window.__TAURI_INTERNALS__) {
        await window.__TAURI_INTERNALS__.invoke('complete_onboarding', { language, layout, theme });
      } else {
        localStorage.setItem('onboarding_complete', 'true');
      }
    } catch (e) {
      console.error('Failed to save onboarding:', e);
      localStorage.setItem('onboarding_complete', 'true');
    }
    showOnboarding = false;
  }

  async function savePreference(key, value) {
    try {
      if (window.__TAURI_INTERNALS__) {
        await window.__TAURI_INTERNALS__.invoke('update_preference', { key, value });
      }
    } catch (e) {
      console.error('Failed to save preference:', e);
    }
  }

  const sidebarItems = $derived([
    { id: 'layout', icon: Keyboard, label: t('nav.layout') },
    { id: 'hotkeys', icon: Lightning, label: t('nav.hotkeys') },
    { id: 'theme', icon: Palette, label: t('nav.theme') },
    { id: 'dictionary', icon: Book, label: t('nav.dictionary') },
    { id: 'about', icon: Info, label: t('nav.about') },
  ]);

  const layoutOptions = [
    { value: 'phonetic', label: 'Phonetic (Avro)' },
    { value: 'unibijoy', label: 'UniBijoy' },
    { value: 'national', label: 'National (Jatiya)' },
  ];

  const hotkeyOptions = [
    { value: 'ctrl+space', label: 'Ctrl + Space' },
    { value: 'cmd+space', label: 'Cmd + Space' },
    { value: 'f1', label: 'F1' },
    { value: 'f12', label: 'F12' },
  ];

  const themeOptions = $derived([
    { value: 'dark', label: t('theme.dark'), description: t('theme.dark.desc'), icon: Moon },
    { value: 'light', label: t('theme.light'), description: t('theme.light.desc'), icon: Sun },
  ]);

  function setTheme(value) {
    currentTheme = value;
    if (value === 'light') {
      document.documentElement.setAttribute('data-theme', 'light');
    } else {
      document.documentElement.removeAttribute('data-theme');
    }
    savePreference('theme', value);
  }
</script>

{#if loading}
  <div class="loading-screen">
    <div class="loading-spinner"></div>
  </div>
{:else if showOnboarding}
  <Onboarding oncomplete={handleOnboardingComplete} />
{:else}
<div class="app">
  <Sidebar
    items={sidebarItems}
    activeId={activeSection}
    onselect={(id) => activeSection = id}
  />

  <main class="content">
    {#if activeSection === 'layout'}
      <div class="page">
        <h1 class="page-title">Layout</h1>
        <p class="page-desc">Choose your preferred Bangla typing layout.</p>

        <section class="setting-group">
          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label">Active Layout</span>
              <span class="setting-hint">Switch between Phonetic and UniBijoy input methods</span>
            </div>
            <Select
              options={layoutOptions}
              value={layoutMode}
              onchange={(v) => { layoutMode = v; savePreference('layout', v); }}
            />
          </div>

          <div class="setting-row">
            <Toggle
              label="Show on-screen keyboard"
              checked={showKeyboard}
              onchange={(v) => showKeyboard = v}
            />
          </div>
        </section>

        {#if showKeyboard && (layoutMode === 'unibijoy' || layoutMode === 'national')}
          <section class="setting-group">
            <div class="setting-row">
              <Toggle
                label="Show Shift layer"
                checked={shiftPreview}
                onchange={(v) => shiftPreview = v}
              />
            </div>
            <KeyboardViewer layout={layoutMode} shift={shiftPreview} />
          </section>
        {/if}
      </div>

    {:else if activeSection === 'hotkeys'}
      <div class="page">
        <h1 class="page-title">Hotkeys</h1>
        <p class="page-desc">Configure keyboard shortcuts.</p>

        <section class="setting-group">
          <div class="setting-row">
            <div class="setting-info">
              <span class="setting-label">Toggle Bangla/English</span>
              <span class="setting-hint">Shortcut to switch between Bangla and system keyboard</span>
            </div>
            <Select
              options={hotkeyOptions}
              value={hotkeyToggle}
              onchange={(v) => hotkeyToggle = v}
            />
          </div>
        </section>
      </div>

    {:else if activeSection === 'theme'}
      <div class="page">
        <h1 class="page-title">{t('theme.title')}</h1>
        <p class="page-desc">{t('theme.desc')}</p>

        <section class="setting-group">
          <RadioCardGroup
            options={themeOptions}
            value={currentTheme}
            onselect={setTheme}
          />
        </section>
      </div>

    {:else if activeSection === 'dictionary'}
      <div class="page">
        <h1 class="page-title">{t('dictionary.title')}</h1>
        <p class="page-desc">{t('dictionary.desc')}</p>

        <section class="setting-group">
          <p class="empty-state">Custom dictionary management coming soon.</p>
        </section>
      </div>

    {:else if activeSection === 'about'}
      <div class="page">
        <h1 class="page-title">About</h1>

        <section class="setting-group">
          <div class="about-info">
            <h2 class="about-name">Bangla Keyboard</h2>
            <p class="about-version">Version 0.1.0</p>
            <p class="about-desc">
              A macOS + Windows input method supporting UniBijoy and Phonetic
              keyboard layouts for typing in Bangla.
            </p>
          </div>
        </section>
      </div>
    {/if}
  </main>
</div>
{/if}

<style>
  .app {
    display: flex;
    height: 100vh;
    width: 100vw;
  }

  .content {
    flex: 1;
    overflow-y: auto;
    background: var(--bg-secondary);
  }

  .page {
    max-width: 640px;
    padding: var(--space-3xl) var(--space-2xl);
  }

  .page-title {
    font-size: 24px;
    font-weight: 600;
    color: var(--text-primary);
    letter-spacing: -0.015em;
    margin-bottom: var(--space-xs);
  }

  .page-desc {
    font-size: 14px;
    color: var(--text-secondary);
    margin-bottom: var(--space-xl);
  }

  .setting-group {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: var(--space-lg);
    margin-bottom: var(--space-lg);
    display: flex;
    flex-direction: column;
    gap: var(--space-lg);
  }

  .setting-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .setting-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .setting-label {
    font-size: 14px;
    font-weight: 500;
    color: var(--text-primary);
  }

  .setting-hint {
    font-size: 12px;
    color: var(--text-tertiary);
  }

  .empty-state {
    text-align: center;
    padding: var(--space-2xl);
    color: var(--text-tertiary);
    font-size: 14px;
  }

  .about-info {
    text-align: center;
    padding: var(--space-xl);
  }

  .about-name {
    font-size: 20px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: var(--space-xs);
  }

  .about-version {
    font-size: 14px;
    color: var(--text-secondary);
    margin-bottom: var(--space-lg);
  }

  .about-desc {
    font-size: 14px;
    color: var(--text-secondary);
    line-height: 1.6;
  }

  .loading-screen {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
    width: 100vw;
    background: var(--bg);
  }

  .loading-spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--border);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
