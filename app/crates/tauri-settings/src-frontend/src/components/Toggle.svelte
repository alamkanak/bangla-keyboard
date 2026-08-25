<script>
  let { checked = false, label = '', onchange } = $props();
</script>

<label class="toggle-wrapper">
  <span class="toggle-label">{label}</span>
  <button
    class="toggle"
    class:active={checked}
    role="switch"
    aria-checked={checked}
    aria-label={label}
    onclick={() => onchange(!checked)}
  >
    <span class="toggle-thumb"></span>
  </button>
</label>

<style>
  .toggle-wrapper {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-md);
    cursor: pointer;
  }

  .toggle-label {
    font-size: 14px;
    color: var(--text-primary);
  }

  .toggle {
    position: relative;
    width: 44px;
    height: 24px;
    border: none;
    border-radius: var(--radius-full);
    background: var(--surface);
    cursor: pointer;
    transition: background-color 0.2s;
    padding: 0;
  }

  .toggle.active {
    background: var(--accent);
  }

  .toggle-thumb {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: white;
    transition: transform 0.2s;
  }

  .toggle.active .toggle-thumb {
    transform: translateX(20px);
  }
</style>
