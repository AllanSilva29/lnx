<script>
  export let query = '';
  export let loading = false;
  export let onSearch;
  export let onQueryChange;
  export let onKeydown;
</script>

<div class="search-box">
  <input 
    type="text" 
    bind:value={query} 
    on:input={onQueryChange}
    on:keydown={onKeydown}
    placeholder="Search documents..."
    class="search-input"
  />
  <button on:click={onSearch} disabled={loading} class="search-btn">
    {loading ? 'Searching...' : 'Search'}
  </button>
</div>

<style>
  .search-box {
    display: flex;
    gap: 10px;
    margin-bottom: 15px;
  }

  .search-input {
    flex: 1;
    padding: 12px 16px;
    font-size: 1rem;
    border: 2px solid #333;
    border-radius: 8px;
    background: #0f0f0f;
    color: #fff;
    outline: none;
    transition: border-color 0.2s;
  }

  .search-input:focus {
    border-color: #ff6b6b;
  }

  .search-btn {
    padding: 12px 24px;
    font-size: 1rem;
    background: #ff6b6b;
    color: #fff;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: background 0.2s;
    white-space: nowrap;
  }

  .search-btn:hover:not(:disabled) {
    background: #ff5252;
  }

  .search-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
