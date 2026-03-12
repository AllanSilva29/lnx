<script>
  export let result = null;
  export let query = '';
  export let onClose = () => {};

  let pageContent = '';
  let loading = false;
  let error = null;

  $: if (result && result.currentBlock) {
    loadFullPageContent();
  }

  async function loadFullPageContent() {
    if (!result || !result.currentBlock) return;
    
    loading = true;
    error = null;
    
    try {
      // This would need to be implemented in the backend
      // For now, we'll simulate with the snippet content
      const allSnippets = result.currentBlock.matches
        .map(match => match.snippet)
        .join('\n...\n');
      
      pageContent = allSnippets;
    } catch (e) {
      error = `Erro ao carregar conteúdo: ${e.message}`;
    } finally {
      loading = false;
    }
  }

  function highlightText(text, searchTerm) {
    if (!text || !searchTerm) return text;
    
    const regex = new RegExp(`(${searchTerm})`, 'gi');
    return text.replace(regex, '<mark>$1</mark>');
  }

  function getContextAroundMatch(fullText, matchPosition, contextLines = 3) {
    const lines = fullText.split('\n');
    const matchLine = Math.floor(matchPosition);
    
    const start = Math.max(0, matchLine - contextLines);
    const end = Math.min(lines.length, matchLine + contextLines + 1);
    
    return lines.slice(start, end).join('\n');
  }
</script>

{#if result}
  <div class="page-viewer-overlay" on:click={onClose}>
    <div class="page-viewer-content" on:click|stopPropagation>
      <div class="page-viewer-header">
        <div class="file-info">
          <h2>{result.filename}</h2>
          <div class="location">
            {#if result.currentBlock.page !== null && result.currentBlock.page !== undefined}
              Página {result.currentBlock.page}
            {:else if result.currentBlock.section !== null && result.currentBlock.section !== undefined}
              Seção {result.currentBlock.section}
            {:else}
              Conteúdo
            {/if}
          </div>
        </div>
        <button class="close-btn" on:click={onClose}>✕</button>
      </div>

      <div class="page-viewer-body">
        {#if loading}
          <div class="loading-content">
            <div class="spinner"></div>
            <p>Carregando conteúdo completo...</p>
          </div>
        {:else if error}
          <div class="error-content">
            <p>{error}</p>
          </div>
        {:else}
          <div class="content-container">
            <div class="occurrences-summary">
              <span class="total-occurrences">
                {result.currentBlock.occurrences} ocorrência{result.currentBlock.occurrences !== 1 ? 's' : ''} nesta página
              </span>
              <span class="file-total">
                {result.total_occurrences} total no arquivo
              </span>
            </div>

            <div class="page-content">
              {#each result.currentBlock.matches as match, index}
                <div class="match-section">
                  <div class="match-header">
                    <span class="match-index">Ocorrência {index + 1}</span>
                    <span class="match-location">
                      {match.field} • Linha {match.line}:{match.char}
                    </span>
                  </div>
                  
                  <div class="context-block">
                    <div class="context-content">
                      {@html highlightText(match.snippet, query)}
                    </div>
                  </div>
                </div>
              {/each}
            </div>

            {#if result.allBlocks && result.allBlocks.length > 1}
              <div class="other-blocks">
                <h3>Outras páginas/seções com ocorrências:</h3>
                <div class="blocks-list">
                  {#each result.allBlocks as block}
                    {#if block !== result.currentBlock}
                      <div class="block-item">
                        <div class="block-info">
                          {#if block.page !== null && block.page !== undefined}
                            Página {block.page}
                          {:else if block.section !== null && block.section !== undefined}
                            Seção {block.section}
                          {:else}
                            Conteúdo
                          {/if}
                          <span class="block-occurrences">({block.occurrences} ocorrências)</span>
                        </div>
                        <div class="block-preview">
                          {#each block.matches.slice(0, 1) as match}
                            <div class="mini-snippet">
                              {@html highlightText(match.snippet, query)}
                            </div>
                          {/each}
                        </div>
                      </div>
                    {/if}
                  {/each}
                </div>
              </div>
            {/if}
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .page-viewer-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.8);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 20px;
  }

  .page-viewer-content {
    background: #1a1a1a;
    border-radius: 12px;
    max-width: 900px;
    max-height: 90vh;
    width: 100%;
    display: flex;
    flex-direction: column;
    border: 1px solid #333;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
  }

  .page-viewer-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px 25px;
    border-bottom: 2px solid #333;
    background: #0f0f0f;
    border-radius: 12px 12px 0 0;
  }

  .file-info h2 {
    margin: 0 0 5px 0;
    color: #ff6b6b;
    font-size: 1.2rem;
    font-weight: 600;
  }

  .location {
    color: #4ade80;
    font-size: 0.9rem;
    background: #1a1a1a;
    padding: 4px 10px;
    border-radius: 4px;
    display: inline-block;
  }

  .close-btn {
    background: #333;
    color: #fff;
    border: none;
    border-radius: 50%;
    width: 36px;
    height: 36px;
    font-size: 18px;
    cursor: pointer;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .close-btn:hover {
    background: #ff6b6b;
    transform: scale(1.1);
  }

  .page-viewer-body {
    flex: 1;
    overflow-y: auto;
    padding: 25px;
  }

  .loading-content {
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

  .error-content {
    text-align: center;
    padding: 40px;
    color: #f87171;
  }

  .content-container {
    display: flex;
    flex-direction: column;
    gap: 25px;
  }

  .occurrences-summary {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 15px 20px;
    background: #0f0f0f;
    border-radius: 8px;
    border: 1px solid #333;
  }

  .total-occurrences {
    color: #ff6b6b;
    font-weight: 600;
    font-size: 1rem;
  }

  .file-total {
    color: #888;
    font-size: 0.85rem;
  }

  .page-content {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .match-section {
    background: #0f0f0f;
    border: 1px solid #333;
    border-radius: 8px;
    overflow: hidden;
  }

  .match-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 15px;
    background: #1a1a1a;
    border-bottom: 1px solid #333;
  }

  .match-index {
    color: #4ade80;
    font-weight: 600;
    font-size: 0.9rem;
  }

  .match-location {
    color: #888;
    font-size: 0.8rem;
  }

  .context-block {
    padding: 20px;
  }

  .context-content {
    font-family: 'Courier New', monospace;
    font-size: 0.9rem;
    line-height: 1.6;
    color: #e0e0e0;
    white-space: pre-wrap;
    word-break: break-word;
    background: #1a1a1a;
    padding: 15px;
    border-radius: 6px;
    border-left: 4px solid #ff6b6b;
  }

  .context-content :global(mark) {
    background: #ff6b6b;
    color: white;
    padding: 2px 4px;
    border-radius: 3px;
    font-weight: bold;
  }

  .other-blocks {
    margin-top: 20px;
    padding-top: 20px;
    border-top: 2px solid #333;
  }

  .other-blocks h3 {
    margin: 0 0 15px 0;
    color: #4ade80;
    font-size: 1rem;
  }

  .blocks-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .block-item {
    background: #0f0f0f;
    border: 1px solid #333;
    border-radius: 6px;
    padding: 12px 15px;
    transition: all 0.2s;
  }

  .block-item:hover {
    border-color: #4ade80;
  }

  .block-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
    color: #4ade80;
    font-weight: 600;
    font-size: 0.85rem;
  }

  .block-occurrences {
    color: #ff6b6b;
    font-size: 0.75rem;
  }

  .block-preview {
    font-family: 'Courier New', monospace;
    font-size: 0.8rem;
    color: #aaa;
    line-height: 1.4;
  }

  .mini-snippet :global(mark) {
    background: #ff6b6b;
    color: white;
    padding: 1px 3px;
    border-radius: 2px;
  }
</style>
