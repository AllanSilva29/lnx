<script>
  import { createEventDispatcher } from 'svelte';

  export let results = [];
  export let loading = false;
  export let query = '';
  export let error = null;
  export let searchOptions = {};

  const dispatch = createEventDispatcher();

  let expandedResults = [];
  let selectedResult = null;

  $: if (results && results.length > 0) {
    expandedResults = results.flatMap(result => 
      result.page_blocks ? result.page_blocks.map((block, blockIndex) => ({
        ...result,
        currentBlock: block,
        allBlocks: result.page_blocks,
        uniqueId: crypto.randomUUID()
      })) : []
    );
  }

  function selectResult(result) {
    selectedResult = selectedResult === result ? null : result;
    dispatch('selectResult', result);
  }

  function getContextSnippet(match, contextSize = 200) {
    if (!match.snippet) return '';
    
    // This is a simplified context extraction
    // In a real implementation, you'd want to fetch more context from the server
    return match.snippet;
  }

  function highlightText(text, searchTerm) {
    if (!text || !searchTerm) return text;
    
    const regex = new RegExp(`(${searchTerm})`, 'gi');
    return text.replace(regex, '<mark>$1</mark>');
  }
</script>

<section class="expanded-gallery">
  {#if results.length > 0}
    <div class="gallery-header">
      <div class="header-info">
        <span class="results-count">
          {results.length} arquivos • {expandedResults.length} blocos com ocorrências
        </span>
        {#if searchOptions.semantic}
          <span class="semantic-badge">🔮 Semantic</span>
        {/if}
      </div>
      <div class="query-info">
        Busca: <strong>"{query}"</strong>
      </div>
    </div>

    <div class="results-container">
      {#each expandedResults as result, index (result.uniqueId)}
        <div 
            class="result-item" 
            class:selected={selectedResult === result}
            role="button"
            tabindex="0"
            on:click={() => selectResult(result)}
            on:keydown={(e) => {
              if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                selectResult(result);
              }
            }}
            aria-label={`Selecionar resultado de ${result.filename}, ${result.currentBlock.occurrences} ocorrências`}
            aria-pressed={selectedResult === result}
          >
          <div class="result-header">
            <div class="file-info">
              <span class="filename">{result.filename}</span>
              <span class="location">
                {#if result.currentBlock.page !== null && result.currentBlock.page !== undefined}
                  Página {result.currentBlock.page}
                {:else if result.currentBlock.section !== null && result.currentBlock.section !== undefined}
                  Seção {result.currentBlock.section}
                {:else}
                  Conteúdo
                {/if}
              </span>
            </div>
            <div class="occurrence-info">
              <span class="occurrence-count">
                {result.currentBlock.occurrences} ocorrência{result.currentBlock.occurrences !== 1 ? 's' : ''}
              </span>
              <span class="total-in-file">
                {result.total_occurrences} total no arquivo
              </span>
            </div>
          </div>

          <div class="matches-container">
            {#each result.currentBlock.matches as match, index (index)}
              <div class="match-item">
                <div class="match-location">
                  <span class="field-name">{match.field}</span>
                  <span class="line-info">Linha {match.line}:{match.char}</span>
                </div>
                <div class="match-content">
                  <div class="snippet">
                    {@html highlightText(match.snippet, query)}
                  </div>
                </div>
              </div>
            {/each}
          </div>

          {#if selectedResult === result}
            <div class="expanded-context">
              <div class="context-header">
                <h4>Todas as ocorrências em {result.filename}</h4>
              </div>
              <div class="all-blocks">
                {#each result.allBlocks as block, blockIndex (blockIndex)}
                  <div class="block-summary">
                    <div class="block-location">
                      {#if block.page !== null && block.page !== undefined}
                        Página {block.page}
                      {:else if block.section !== null && block.section !== undefined}
                        Seção {block.section}
                      {:else}
                        Conteúdo
                      {/if}
                      <span class="block-count">({block.occurrences} ocorrências)</span>
                    </div>
                    <div class="block-matches">
                      {#each block.matches.slice(0, 2) as match, matchIndex (blockIndex + '_' + matchIndex)}
                        <div class="mini-match">
                          {@html highlightText(match.snippet, query)}
                        </div>
                      {/each}
                      {#if block.matches.length > 2}
                        <div class="more-matches">+{block.matches.length - 2} mais</div>
                      {/if}
                    </div>
                  </div>
                {/each}
              </div>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {:else if loading}
    <div class="loading-state">
      <div class="spinner"></div>
      <p>Buscando todas as ocorrências...</p>
    </div>
  {:else if !query}
    <div class="placeholder">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="8"></circle>
        <path d="m21 21-4.35-4.35"></path>
      </svg>
      <p>Digite para buscar todas as ocorrências nos documentos</p>
      <p class="hint">Modo expandido: mostra TODAS as ocorrências encontradas</p>
    </div>
  {:else if !error}
    <div class="no-results">
      <p>Nenhuma ocorrência encontrada para "{query}"</p>
      <p class="hint">Tente outros termos ou verifique a ortografia</p>
    </div>
  {/if}

  {#if error}
    <div class="error">{error}</div>
  {/if}
</section>

<style>
  .expanded-gallery {
    background: #1a1a1a;
    padding: 20px;
    border-radius: 8px;
    min-height: 200px;
  }

  .gallery-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 25px;
    padding-bottom: 15px;
    border-bottom: 2px solid #333;
  }

  .header-info {
    display: flex;
    align-items: center;
    gap: 15px;
  }

  .results-count {
    color: #4ade80;
    font-size: 0.9rem;
    font-weight: 600;
  }

  .semantic-badge {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 4px 12px;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
  }

  .query-info {
    color: #888;
    font-size: 0.85rem;
  }

  .results-container {
    display: flex;
    flex-direction: column;
    gap: 15px;
  }

  .result-item {
    background: #0f0f0f;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 20px;
    transition: all 0.3s ease;
    cursor: pointer;
    outline: none;
  }

  .result-item:hover {
    border-color: #ff6b6b;
    box-shadow: 0 4px 12px rgba(255, 107, 107, 0.1);
  }

  .result-item.selected {
    border-color: #4ade80;
    box-shadow: 0 4px 12px rgba(74, 222, 128, 0.2);
  }

  .result-item:focus {
    border-color: #ff6b6b;
    box-shadow: 0 0 0 3px rgba(255, 107, 107, 0.3);
  }

  .result-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 15px;
    padding-bottom: 10px;
    border-bottom: 1px solid #333;
  }

  .file-info {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .filename {
    color: #ff6b6b;
    font-weight: bold;
    font-size: 1rem;
  }

  .location {
    color: #4ade80;
    font-size: 0.85rem;
    background: #1a1a1a;
    padding: 2px 8px;
    border-radius: 4px;
  }

  .occurrence-info {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 2px;
  }

  .occurrence-count {
    color: #ff6b6b;
    font-weight: 600;
    font-size: 0.9rem;
  }

  .total-in-file {
    color: #888;
    font-size: 0.75rem;
  }

  .matches-container {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .match-item {
    background: #1a1a1a;
    border: 1px solid #444;
    border-radius: 6px;
    padding: 12px;
  }

  .match-location {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }

  .field-name {
    color: #4ade80;
    font-size: 0.8rem;
    font-weight: 600;
    background: #0f0f0f;
    padding: 2px 6px;
    border-radius: 3px;
  }

  .line-info {
    color: #888;
    font-size: 0.75rem;
  }

  .match-content {
    border-left: 3px solid #ff6b6b;
    padding-left: 12px;
  }

  .snippet {
    font-family: 'Courier New', monospace;
    font-size: 0.85rem;
    color: #e0e0e0;
    line-height: 1.5;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .snippet :global(mark) {
    background: #ff6b6b;
    color: white;
    padding: 1px 3px;
    border-radius: 2px;
    font-weight: bold;
  }

  .expanded-context {
    margin-top: 20px;
    padding-top: 15px;
    border-top: 2px solid #333;
  }

  .context-header h4 {
    margin: 0 0 15px 0;
    color: #4ade80;
    font-size: 0.95rem;
  }

  .all-blocks {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .block-summary {
    background: #1a1a1a;
    border: 1px solid #444;
    border-radius: 6px;
    padding: 12px;
  }

  .block-location {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
    color: #4ade80;
    font-weight: 600;
    font-size: 0.85rem;
  }

  .block-count {
    color: #ff6b6b;
    font-size: 0.75rem;
  }

  .block-matches {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .mini-match {
    font-family: 'Courier New', monospace;
    font-size: 0.8rem;
    color: #aaa;
    line-height: 1.4;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .more-matches {
    color: #888;
    font-size: 0.75rem;
    font-style: italic;
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
