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
  let currentTheme = $state('dark');
  const isMac = navigator.platform.toUpperCase().includes('MAC');

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
        <h1 class="page-title">{t('hotkeys.title')}</h1>
        <p class="page-desc">{t('hotkeys.desc')}</p>

        <section class="setting-group">
          <h3 class="hotkey-section-title">{t('hotkeys.switch.title')}</h3>
          <p class="hotkey-section-desc">{t('hotkeys.switch.desc')}</p>
          <div class="hotkey-list">
            {#if isMac}
              <div class="hotkey-item">
                <kbd>Globe</kbd> / <kbd>Fn</kbd>
                <span class="hotkey-item-desc">{t('hotkeys.mac.globe')}</span>
              </div>
              <div class="hotkey-item">
                <kbd>Ctrl</kbd> + <kbd>Space</kbd>
                <span class="hotkey-item-desc">{t('hotkeys.mac.ctrlspace')}</span>
              </div>
            {:else}
              <div class="hotkey-item">
                <kbd>Win</kbd> + <kbd>Space</kbd>
                <span class="hotkey-item-desc">{t('hotkeys.win.winspace')}</span>
              </div>
              <div class="hotkey-item">
                <kbd>Alt</kbd> + <kbd>Shift</kbd>
                <span class="hotkey-item-desc">{t('hotkeys.win.altshift')}</span>
              </div>
            {/if}
          </div>
        </section>

        <section class="setting-group">
          <h3 class="hotkey-section-title">{t('hotkeys.customize.title')}</h3>
          <p class="hotkey-section-desc">
            {#if isMac}
              {t('hotkeys.customize.mac')}
            {:else}
              {t('hotkeys.customize.win')}
            {/if}
          </p>
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

  .hotkey-section-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .hotkey-section-desc {
    font-size: 13px;
    color: var(--text-secondary);
    line-height: 1.5;
    margin: 0;
  }

  .hotkey-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
  }

  .hotkey-item {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    font-size: 14px;
    color: var(--text-primary);
  }

  .hotkey-item-desc {
    font-size: 13px;
    color: var(--text-tertiary);
    margin-left: var(--space-xs);
  }

  .hotkey-item kbd {
    display: inline-block;
    padding: 2px 8px;
    font-size: 12px;
    font-family: inherit;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    color: var(--text-primary);
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
