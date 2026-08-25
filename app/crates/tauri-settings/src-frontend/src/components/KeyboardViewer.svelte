<script>
  let { layout = 'unibijoy', shift = false } = $props();

  const unibijoyNormal = [
    ['ঙ','য','ড','প','ট','চ','জ','হ','গ','ড়'],
    ['ৃ','ু','ি','া','্','ব','ক','ত','দ'],
    ['','ও','ে','র','ন','স','ম']
  ];

  const unibijoyShift = [
    ['ং','য়','ঢ','ফ','ঠ','ছ','ঝ','ঞ','ঘ','ঢ়'],
    ['','ূ','ী','অ','।','ভ','খ','থ','ধ'],
    ['','ৌ','ৈ','ল','ণ','ষ','শ']
  ];

  const qwertyRows = [
    ['Q','W','E','R','T','Y','U','I','O','P'],
    ['A','S','D','F','G','H','J','K','L'],
    ['Z','X','C','V','B','N','M']
  ];

  let displayRows = $derived(
    layout === 'unibijoy'
      ? (shift ? unibijoyShift : unibijoyNormal)
      : qwertyRows
  );
</script>

<div class="keyboard">
  {#each displayRows as row, rowIdx}
    <div class="keyboard-row" style:padding-left="{rowIdx * 20}px">
      {#each row as key, keyIdx}
        <div class="key">
          <span class="key-bangla">{key}</span>
          <span class="key-latin">{qwertyRows[rowIdx][keyIdx]}</span>
        </div>
      {/each}
    </div>
  {/each}
</div>

<style>
  .keyboard {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    padding: var(--space-lg);
    background: var(--bg-tertiary);
    border-radius: var(--radius-lg);
  }

  .keyboard-row {
    display: flex;
    gap: var(--space-xs);
  }

  .key {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    transition: background 0.1s;
  }

  .key:hover {
    background: var(--surface-hover);
  }

  .key-bangla {
    font-size: 16px;
    color: var(--text-primary);
    line-height: 1;
  }

  .key-latin {
    font-size: 10px;
    color: var(--text-tertiary);
    line-height: 1;
    margin-top: 2px;
  }
</style>
