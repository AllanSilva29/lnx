<script>
  export let indexes = [];
  export let selectedIndex = '';
  export let searchOptions = {};
</script>

<div class="options">
  <label>
    Index:
    <select bind:value={selectedIndex}>
      {#each indexes as idx}
        <option value={idx}>{idx}</option>
      {/each}
    </select>
  </label>
  <label>
    <input type="checkbox" bind:checked={searchOptions.liveSearch} />
    Live Search
  </label>
  <label>
    <input type="checkbox" bind:checked={searchOptions.semantic} />
    Semantic Search
  </label>
  <label>
    Limit:
    <input type="number" bind:value={searchOptions.limit} min="1" max="100" />
  </label>
</div>

<style>
  .options {
    display: flex;
    gap: 20px;
    flex-wrap: wrap;
    align-items: center;
  }

  .options label {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.9rem;
    color: #888;
  }

  .options input[type="checkbox"] {
    cursor: pointer;
  }

  .options select,
  .options input[type="number"] {
    padding: 6px 10px;
    border: 1px solid #333;
    border-radius: 4px;
    background: #0f0f0f;
    color: #fff;
  }

  .options input[type="number"] {
    width: 80px;
  }
</style>
