<script>
  let { items = [], activeId = '', onselect } = $props();
</script>

<nav class="sidebar">
  <div class="sidebar-header">
    <span class="sidebar-title">Settings</span>
  </div>
  <ul class="sidebar-nav">
    {#each items as item}
      <li>
        <button
          class="sidebar-item"
          class:active={activeId === item.id}
          onclick={() => onselect(item.id)}
        >
          <span class="sidebar-icon">{item.icon}</span>
          <span class="sidebar-label">{item.label}</span>
        </button>
      </li>
    {/each}
  </ul>
</nav>

<style>
  .sidebar {
    width: 240px;
    min-width: 240px;
    background: var(--bg);
    border-right: 1px solid var(--separator);
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .sidebar-header {
    padding: var(--space-xl) var(--space-lg);
    border-bottom: 1px solid var(--separator);
  }

  .sidebar-title {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .sidebar-nav {
    list-style: none;
    padding: var(--space-sm);
    flex: 1;
    overflow-y: auto;
  }

  .sidebar-item {
    display: flex;
    align-items: center;
    gap: var(--space-md);
    width: 100%;
    padding: var(--space-sm) var(--space-md);
    border: none;
    background: transparent;
    color: var(--text-secondary);
    font-family: var(--font-sans);
    font-size: 14px;
    font-weight: 400;
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: background-color 0.15s, color 0.15s;
    text-align: left;
  }

  .sidebar-item:hover {
    background: var(--surface-hover);
    color: var(--text-primary);
  }

  .sidebar-item.active {
    background: var(--accent-subtle);
    color: var(--text-primary);
  }

  .sidebar-icon {
    font-size: 18px;
    width: 24px;
    text-align: center;
  }
</style>
