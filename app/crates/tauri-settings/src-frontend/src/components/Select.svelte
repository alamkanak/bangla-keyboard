<script>
  let { options = [], value = '', onchange } = $props();
</script>

<select
  class="select"
  {value}
  onchange={(e) => onchange(e.target.value)}
>
  {#each options as option}
    <option value={option.value}>{option.label}</option>
  {/each}
</select>

<style>
  .select {
    height: 36px;
    padding: 0 var(--space-lg) 0 var(--space-md);
    background: var(--surface);
    color: var(--text-primary);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    font-family: var(--font-sans);
    font-size: 14px;
    cursor: pointer;
    appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='%23A0A0AB' d='M6 8L1 3h10z'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 8px center;
    padding-right: 28px;
  }

  .select:focus {
    outline: none;
    border-color: var(--border-focus);
  }
</style>
