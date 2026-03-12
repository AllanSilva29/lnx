<script>
  import ResultCard from './ResultCard.svelte';
  
  export let results = [];
  export let loading = false;
  export let query = '';
  export let error = null;
  export let searchOptions = {};
</script>

<section class="results-section">
  {#if results.length > 0}
    <div class="results-header">
      <span class="results-count">{results.length} files found</span>
      {#if searchOptions.semantic}
        <span class="semantic-badge">🔮 Semantic</span>
      {/if}
    </div>
    
    <div class="results-gallery">
      {#each results as result}
        <ResultCard {result} />
      {/each}
    </div>
  {:else if loading}
    <div class="loading-state">
      <div class="spinner"></div>
      <p>Searching...</p>
    </div>
  {:else if !query}
    <div class="placeholder">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="8"></circle>
        <path d="m21 21-4.35-4.35"></path>
      </svg>
      <p>Start typing to search documents</p>
      {#if searchOptions.liveSearch}
        <p class="hint">Live search enabled - results appear as you type</p>
      {/if}
    </div>
  {:else if !error}
    <div class="no-results">
      <p>No results found for "{query}"</p>
      <p class="hint">Try different keywords or check your spelling</p>
    </div>
  {/if}

  {#if error}
    <div class="error">{error}</div>
  {/if}
</section>

<style>
  .results-section {
    background: #1a1a1a;
    padding: 20px;
    border-radius: 8px;
    min-height: 200px;
  }

  .results-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
  }

  .results-count {
    color: #888;
    font-size: 0.9rem;
  }

  .semantic-badge {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 4px 12px;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
  }

  .results-gallery {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 15px;
  }

  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px 20px;
    color: #888;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid #333;
    border-top-color: #ff6b6b;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin-bottom: 15px;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px 20px;
    color: #666;
    text-align: center;
  }

  .placeholder svg {
    margin-bottom: 20px;
    opacity: 0.5;
  }

  .placeholder p {
    margin: 5px 0;
  }

  .placeholder .hint {
    font-size: 0.85rem;
    color: #555;
  }

  .no-results {
    text-align: center;
    padding: 60px 20px;
    color: #888;
  }

  .no-results p {
    margin: 10px 0;
  }

  .no-results .hint {
    font-size: 0.85rem;
    color: #666;
  }

  .error {
    background: #2a1a1a;
    border: 1px solid #f87171;
    color: #f87171;
    padding: 12px;
    border-radius: 8px;
    margin-top: 20px;
  }
</style>
