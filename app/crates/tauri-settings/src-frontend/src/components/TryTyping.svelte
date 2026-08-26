<script>
  import { t } from '../lib/i18n.js';
  import { Lightbulb, CheckCircle } from 'phosphor-svelte';

  let { layout = 'phonetic' } = $props();

  let inputText = $state('');
  let hasBangla = $derived(containsBangla(inputText));

  const isMac = $derived(navigator.platform?.toLowerCase().includes('mac') ?? false);

  function containsBangla(text) {
    // Bengali Unicode block: U+0980–U+09FF
    return /[\u0980-\u09FF]/.test(text);
  }
</script>

<div class="try-typing">
  <div class="try-hint">
    <div class="hint-icon"><Lightbulb size={18} weight="fill" /></div>
    <p class="hint-text">
      {isMac ? t('onboarding.try.hint.mac') : t('onboarding.try.hint.win')}
    </p>
  </div>

  <div class="try-input-wrapper">
    <textarea
      class="try-input"
      class:has-bangla={hasBangla}
      bind:value={inputText}
      placeholder={t('onboarding.try.placeholder')}
      rows="3"
    ></textarea>
    {#if hasBangla}
      <div class="try-success">
        <CheckCircle size={16} weight="fill" />
        <span>{t('onboarding.try.success')}</span>
      </div>
    {/if}
  </div>
</div>

<style>
  .try-typing {
    display: flex;
    flex-direction: column;
    gap: var(--space-lg);
  }

  .try-hint {
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

  .try-input-wrapper {
    position: relative;
  }

  .try-input {
    width: 100%;
    padding: var(--space-lg);
    font-size: 18px;
    font-family: inherit;
    color: var(--text-primary);
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    resize: none;
    outline: none;
    transition: border-color 0.2s;
  }

  .try-input:focus {
    border-color: var(--accent);
  }

  .try-input.has-bangla {
    border-color: var(--success);
  }

  .try-input::placeholder {
    color: var(--text-tertiary);
  }

  .try-success {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    margin-top: var(--space-sm);
    color: var(--success);
    font-size: 13px;
    font-weight: 500;
  }
</style>
