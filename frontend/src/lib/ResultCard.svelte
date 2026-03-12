<script>
  export let result;
</script>

<div class="result-card">
  <div class="card-header">
    <span class="filename">{result.filename || 'Unknown'}</span>
    <div class="header-info">
      <span class="occurrences-badge">{result.total_occurrences} ocorrências</span>
    </div>
  </div>
  
  {#if result.page_blocks && result.page_blocks.length > 0}
    <div class="page-blocks-compact">
      {#each result.page_blocks.slice(0, 3) as block}
        <div class="page-tag">
          {#if block.page}
            Page {block.page}
          {:else if block.section}
            Sec {block.section}
          {:else}
            Content
          {/if}
          <span class="tag-count">({block.occurrences})</span>
        </div>
      {/each}
      {#if result.page_blocks.length > 3}
        <div class="page-tag more">+{result.page_blocks.length - 3}</div>
      {/if}
    </div>
    
    <div class="preview-snippet">
      {#if result.page_blocks[0]?.matches?.[0]?.snippet}
        <pre class="snippet-preview">{result.page_blocks[0].matches[0].snippet}</pre>
      {/if}
    </div>
  {/if}
</div>

<style>
  .result-card {
    background: #0f0f0f;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 15px;
    transition: all 0.2s;
    cursor: pointer;
  }

  .result-card:hover {
    border-color: #ff6b6b;
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(255, 107, 107, 0.2);
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
    padding-bottom: 10px;
    border-bottom: 1px solid #333;
  }

  .filename {
    color: #ff6b6b;
    font-weight: bold;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .header-info {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .occurrences-badge {
    background: #ff6b6b;
    color: white;
    padding: 2px 8px;
    border-radius: 10px;
    font-size: 0.75rem;
    font-weight: 600;
    flex-shrink: 0;
  }

  .page-blocks-compact {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-bottom: 10px;
  }

  .page-tag {
    background: #1a1a1a;
    border: 1px solid #4ade80;
    color: #4ade80;
    padding: 3px 8px;
    border-radius: 4px;
    font-size: 0.7rem;
    font-weight: 500;
  }

  .page-tag.more {
    border-color: #888;
    color: #888;
  }

  .tag-count {
    opacity: 0.7;
  }

  .preview-snippet {
    margin-top: 10px;
  }

  .snippet-preview {
    margin: 0;
    font-size: 0.75rem;
    color: #aaa;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-height: 40px;
    line-height: 1.4;
  }
</style>
