<script>
  let { variant = 'primary', size = 'md', disabled = false, onclick, children } = $props();
</script>

<button
  class="btn btn-{variant} btn-{size}"
  {disabled}
  {onclick}
>
  {@render children()}
</button>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-sm);
    font-family: var(--font-sans);
    font-weight: 500;
    border: none;
    cursor: pointer;
    transition: background-color 0.15s, color 0.15s;
    white-space: nowrap;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-md {
    height: 36px;
    padding: 0 var(--space-lg);
    font-size: 14px;
    border-radius: var(--radius-md);
  }

  .btn-sm {
    height: 28px;
    padding: 0 var(--space-md);
    font-size: 12px;
    border-radius: var(--radius-sm);
  }

  .btn-lg {
    height: 44px;
    padding: 0 var(--space-xl);
    font-size: 16px;
    border-radius: var(--radius-md);
  }

  .btn-primary {
    background: var(--accent);
    color: var(--text-inverse);
  }
  .btn-primary:hover:not(:disabled) {
    background: var(--accent-hover);
  }
  .btn-primary:active:not(:disabled) {
    background: var(--accent-active);
  }

  .btn-secondary {
    background: var(--surface);
    color: var(--text-primary);
  }
  .btn-secondary:hover:not(:disabled) {
    background: var(--surface-hover);
  }

  .btn-ghost {
    background: transparent;
    color: var(--text-secondary);
  }
  .btn-ghost:hover:not(:disabled) {
    background: var(--surface-hover);
    color: var(--text-primary);
  }
</style>
