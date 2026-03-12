<script>
  import { onMount } from 'svelte';
  import api from './lib/api.js';
  import SearchBar from './lib/SearchBar.svelte';
  import SearchOptions from './lib/SearchOptions.svelte';
  import ExpandedGallery from './lib/ExpandedGallery.svelte';
  import PageViewer from './lib/PageViewer.svelte';
  import IndexesTab from './lib/IndexesTab.svelte';
  import ImportTab from './lib/ImportTab.svelte';

  let activeTab = 'search';
  let query = '';
  let results = [];
  let loading = false;
  let error = null;
  let status = 'connecting';
  let searchTimeout = null;
let lastSearchQuery = '';
let lastSearchTime = 0;
  
  let indexes = [];
  let selectedIndex = 'test';
  
  let searchOptions = {
    liveSearch: true,
    semantic: false,
    limit: 50
  };
  
  let selectedResult = null;

  onMount(async () => {
    try {
      await api.health();
      status = 'connected';
      await loadIndexes();
    } catch (e) {
      status = 'disconnected';
    }
  });

  async function loadIndexes() {
    try {
      indexes = await api.listIndexes();
      if (indexes.length > 0 && !indexes.includes(selectedIndex)) {
        selectedIndex = indexes[0];
      }
    } catch (e) {
      console.error('Failed to load indexes:', e);
    }
  }

  async function handleSearch() {
    if (!query.trim()) {
      results = [];
      return;
    }
    
    // Prevent duplicate searches
    const now = Date.now();
    if (query === lastSearchQuery && (now - lastSearchTime) < 1000) {
      return;
    }
    
    lastSearchQuery = query;
    lastSearchTime = now;
    loading = true;
    error = null;
    
    try {
      const response = await api.search(query, { 
        index: selectedIndex, 
        limit: searchOptions.limit,
        semantic: searchOptions.semantic,
        allOccurrences: true
      });
      results = response.results || [];
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  }

  function handleQueryChange() {
    if (!searchOptions.liveSearch) return;
    
    if (searchTimeout) clearTimeout(searchTimeout);
    searchTimeout = setTimeout(() => handleSearch(), 500);
  }

  function handleKeydown(e) {
    if (e.key === 'Enter') {
      if (searchTimeout) clearTimeout(searchTimeout);
      handleSearch();
    }
  }

  function openPageViewer(result) {
    selectedResult = result;
  }

  function closePageViewer() {
    selectedResult = null;
  }
</script>

<main>
  <header>
    <div class="header-content">
      <h1>lnx Search</h1>
      <div class="status" class:connected={status === 'connected'} class:disconnected={status === 'disconnected'}>
        {status === 'connected' ? '● Connected' : status === 'connecting' ? '○ Connecting...' : '○ Disconnected'}
      </div>
    </div>
  </header>

  <nav class="tabs">
    <button class:active={activeTab === 'search'} on:click={() => activeTab = 'search'}>Search</button>
    <button class:active={activeTab === 'indexes'} on:click={() => activeTab = 'indexes'}>Indexes</button>
    <button class:active={activeTab === 'import'} on:click={() => activeTab = 'import'}>Import</button>
  </nav>

  {#if activeTab === 'search'}
    <section class="search-section">
      <SearchBar 
        bind:query 
        {loading} 
        onSearch={handleSearch}
        onQueryChange={handleQueryChange}
        onKeydown={handleKeydown}
      />
      <SearchOptions bind:indexes bind:selectedIndex bind:searchOptions />
    </section>

    <ExpandedGallery {results} {loading} {query} {error} {searchOptions} on:selectResult={(e) => openPageViewer(e.detail)} />
  {/if}

  {#if activeTab === 'indexes'}
    <IndexesTab 
      bind:indexes 
      bind:selectedIndex 
      bind:error
    />
  {/if}

  {#if activeTab === 'import'}
    <ImportTab 
      bind:indexes 
      bind:selectedIndex 
      bind:loading
      bind:error
      {api}
    />
  {/if}
</main>

{#if selectedResult}
  <PageViewer result={selectedResult} query={query} onClose={closePageViewer} />
{/if}

<style>
  :global(body) {
    margin: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
    background: #0f0f0f;
    color: #e0e0e0;
  }

  main {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
  }

  header {
    margin-bottom: 20px;
  }

  .header-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  h1 {
    margin: 0;
    font-size: 1.8rem;
    color: #ff6b6b;
  }

  .status {
    font-size: 0.85rem;
    padding: 4px 12px;
    border-radius: 12px;
    background: #333;
  }

  .status.connected {
    color: #4ade80;
  }

  .status.disconnected {
    color: #f87171;
  }

  .tabs {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
  }

  .tabs button {
    padding: 10px 20px;
    background: #1a1a1a;
    border: none;
    border-radius: 8px 8px 0 0;
    color: #888;
    cursor: pointer;
    transition: all 0.2s;
  }

  .tabs button.active {
    background: #1a1a1a;
    color: #ff6b6b;
    border-bottom: 2px solid #ff6b6b;
  }

  .search-section {
    background: #1a1a1a;
    padding: 20px;
    border-radius: 0 8px 8px 8px;
    margin-bottom: 20px;
  }
</style>
