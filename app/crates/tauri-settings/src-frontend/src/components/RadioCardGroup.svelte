<script>
  let { options = [], value = '', onselect } = $props();
</script>

<div class="radio-group">
  {#each options as option}
    <button
      class="radio-card"
      class:selected={value === option.value}
      onclick={() => onselect(option.value)}
    >
      <div class="radio-dot">
        {#if value === option.value}
          <div class="radio-dot-inner"></div>
        {/if}
      </div>
      <div class="radio-content">
        <span class="radio-label">{option.label}</span>
        {#if option.description}
          <span class="radio-desc">{option.description}</span>
        {/if}
      </div>
      {#if option.icon}
        <span class="radio-icon">
          {#if typeof option.icon === 'string'}
            {option.icon}
          {:else}
            {@const Icon = option.icon}
            <Icon size={24} weight="regular" />
          {/if}
        </span>
      {/if}
    </button>
  {/each}
</div>

<style>
  .radio-group {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .radio-card {
    display: flex;
    align-items: center;
    gap: var(--space-md);
    min-height: 72px;
    padding: var(--space-lg);
    background: var(--surface);
    border: 2px solid var(--border);
    border-radius: var(--radius-lg);
    cursor: pointer;
    transition: border-color 0.15s, background-color 0.15s;
    font-family: var(--font-sans);
    text-align: left;
    width: 100%;
  }

  .radio-card:hover {
    border-color: var(--border-hover);
    background: var(--surface-hover);
  }

  .radio-card.selected {
    border-color: var(--accent);
    background: var(--accent-subtle);
  }

  .radio-dot {
    width: 20px;
    height: 20px;
    min-width: 20px;
    border-radius: 50%;
    border: 2px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: border-color 0.15s;
  }

  .selected .radio-dot {
    border-color: var(--accent);
  }

  .radio-dot-inner {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--accent);
  }

  .radio-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .radio-label {
    font-size: 14px;
    font-weight: 500;
    color: var(--text-primary);
  }

  .radio-desc {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .radio-icon {
    font-size: 24px;
    line-height: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-secondary);
  }
</style>
