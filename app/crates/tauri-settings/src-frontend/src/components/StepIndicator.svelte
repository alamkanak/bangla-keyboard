<script>
  import { t } from '../lib/i18n.js';

  let { active = false, completed = false, stepNumber = 1, label = '' } = $props();
</script>

<div class="step-indicator" class:active class:completed>
  <div class="step-dot">
    {#if completed}
      <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
        <path d="M2.5 6L5 8.5L9.5 3.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    {:else}
      {stepNumber}
    {/if}
  </div>
  <span class="step-label">{label}</span>
</div>

<style>
  .step-indicator {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-sm);
    flex: 1;
  }

  .step-dot {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 13px;
    font-weight: 600;
    background: var(--surface);
    color: var(--text-tertiary);
    border: 2px solid var(--border);
    transition: all 0.2s;
  }

  .active .step-dot {
    background: var(--accent);
    color: var(--text-inverse);
    border-color: var(--accent);
  }

  .completed .step-dot {
    background: var(--success);
    color: white;
    border-color: var(--success);
  }

  .step-label {
    font-size: 11px;
    font-weight: 500;
    color: var(--text-tertiary);
    text-align: center;
    transition: color 0.2s;
  }

  .active .step-label {
    color: var(--text-primary);
  }

  .completed .step-label {
    color: var(--text-secondary);
  }
</style>
