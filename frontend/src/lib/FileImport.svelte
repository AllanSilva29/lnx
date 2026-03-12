<script>
  export let importFiles = [];
  export let loading = false;
  export let importResult = null;
  export let importProgress = '';
  export let error = null;
  export let onImport;
  export let onFileChange;
  export let progressPercentage = 0;
  export let currentFile = '';
  export let processedCount = 0;
  export let totalCount = 0;
</script>

<div class="file-drop">
  <input 
    type="file" 
    accept=".json,.csv,.txt,.html,.pdf,.docx"
    on:change={onFileChange}
    id="file-input"
    multiple
  />
  <label for="file-input">
    {importFiles.length > 0 ? `${importFiles.length} file(s) selected` : 'Choose files (JSON, CSV, TXT, HTML, PDF, DOCX)'}
  </label>
</div>

{#if importFiles.length > 0}
  <div class="selected-files">
    <h4>Selected Files:</h4>
    <ul>
      {#each importFiles as file}
        <li>
          <span class="file-name">{file.name}</span>
          <span class="file-size">({(file.size / 1024).toFixed(1)} KB)</span>
        </li>
      {/each}
    </ul>
    <div class="total-size">
      Total: {(importFiles.reduce((sum, file) => sum + file.size, 0) / 1024 / 1024).toFixed(2)} MB
    </div>
  </div>
  
  <button on:click={onImport} disabled={loading} class="import-btn">
    {loading ? 'Importing...' : `Import ${importFiles.length} File(s)`}
  </button>
{/if}

{#if loading}
  <div class="loading-container">
    <div class="loading-header">
      <div class="loading-spinner"></div>
      <span class="loading-text">Processing documents...</span>
    </div>
    
    {#if currentFile}
      <div class="current-file">
        <strong>Current file:</strong> {currentFile}
      </div>
    {/if}
    
    <div class="progress-bar-container">
      <div class="progress-bar" style="width: {progressPercentage}%"></div>
      <span class="progress-text">{progressPercentage}%</span>
    </div>
    
    {#if totalCount > 0}
      <div class="progress-details">
        Processed: {processedCount} / {totalCount} files
      </div>
    {/if}
    
    {#if importProgress}
      <div class="progress-message">{importProgress}</div>
    {/if}
  </div>
{/if}

{#if importResult}
  <div class="success">
    <div class="success-header">✅ Import Complete</div>
    <div class="success-message">{importResult}</div>
  </div>
{/if}

{#if error}
  <div class="error">
    <div class="error-header">❌ Import Error</div>
    <div class="error-message">{error}</div>
  </div>
{/if}

<style>
  .file-drop {
    margin: 20px 0;
  }

  .file-drop input {
    display: none;
  }

  .file-drop label {
    display: block;
    padding: 40px;
    border: 2px dashed #333;
    border-radius: 8px;
    text-align: center;
    cursor: pointer;
    transition: border-color 0.2s;
    color: #888;
  }

  .file-drop label:hover {
    border-color: #ff6b6b;
  }

  .selected-files {
    margin: 15px 0;
    padding: 15px;
    background: #0f0f0f;
    border-radius: 8px;
  }

  .selected-files h4 {
    margin: 0 0 10px 0;
    font-size: 0.9rem;
    color: #888;
  }

  .selected-files ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .selected-files li {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 0;
    border-bottom: 1px solid #222;
  }

  .selected-files li:last-child {
    border-bottom: none;
  }

  .file-name {
    color: #aaa;
    font-size: 0.85rem;
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file-size {
    color: #666;
    font-size: 0.8rem;
    margin-left: 10px;
  }

  .total-size {
    margin-top: 10px;
    padding-top: 10px;
    border-top: 1px solid #222;
    color: #4ade80;
    font-size: 0.9rem;
    font-weight: 500;
  }

  .import-btn {
    padding: 12px 24px;
    font-size: 1rem;
    background: #ff6b6b;
    color: #fff;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: background 0.2s;
    width: 100%;
  }

  .import-btn:hover:not(:disabled) {
    background: #ff5252;
  }

  .import-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .loading-container {
    margin-top: 20px;
    padding: 20px;
    background: #1a1a2e;
    border: 1px solid #16213e;
    border-radius: 8px;
  }

  .loading-header {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 15px;
  }

  .loading-spinner {
    width: 20px;
    height: 20px;
    border: 2px solid #333;
    border-top: 2px solid #ff6b6b;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .loading-text {
    color: #ff6b6b;
    font-weight: 500;
  }

  .current-file {
    margin-bottom: 15px;
    padding: 10px;
    background: #0f3460;
    border-radius: 4px;
    color: #e94560;
    font-size: 0.9rem;
  }

  .progress-bar-container {
    position: relative;
    height: 24px;
    background: #0f0f0f;
    border: 1px solid #333;
    border-radius: 12px;
    margin-bottom: 10px;
    overflow: hidden;
  }

  .progress-bar {
    height: 100%;
    background: linear-gradient(90deg, #ff6b6b, #ff8e53);
    border-radius: 12px;
    transition: width 0.3s ease;
    position: relative;
  }

  .progress-text {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: #fff;
    font-size: 0.8rem;
    font-weight: 500;
    text-shadow: 0 1px 2px rgba(0,0,0,0.5);
  }

  .progress-details {
    text-align: center;
    color: #888;
    font-size: 0.9rem;
    margin-bottom: 10px;
  }

  .progress-message {
    padding: 10px;
    background: #1a1a1a;
    border-radius: 4px;
    color: #6b9bff;
    font-size: 0.85rem;
    text-align: center;
  }

  .success {
    background: #1a2a1a;
    border: 1px solid #4ade80;
    color: #4ade80;
    padding: 15px;
    border-radius: 8px;
    margin-top: 15px;
  }

  .success-header {
    font-weight: 600;
    margin-bottom: 8px;
    font-size: 1rem;
  }

  .success-message {
    font-size: 0.9rem;
  }

  .error {
    background: #2a1a1a;
    border: 1px solid #f87171;
    color: #f87171;
    padding: 15px;
    border-radius: 8px;
    margin-top: 15px;
  }

  .error-header {
    font-weight: 600;
    margin-bottom: 8px;
    font-size: 1rem;
  }

  .error-message {
    font-size: 0.9rem;
  }
</style>
