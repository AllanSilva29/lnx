<script>
  export let importFiles = [];
  export let loading = false;
  export let importResult = null;
  export let importProgress = '';
  export let error = null;
  export let onImport;
  export let onFileChange;
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
        <li>{file.name} ({(file.size / 1024).toFixed(1)} KB)</li>
      {/each}
    </ul>
  </div>
  
  <button on:click={onImport} disabled={loading} class="import-btn">
    {loading ? 'Importing...' : `Import ${importFiles.length} File(s)`}
  </button>
{/if}

{#if importResult}
  <div class="success">{importResult}</div>
{/if}

{#if importProgress}
  <div class="progress">{importProgress}</div>
{/if}

{#if error}
  <div class="error">{error}</div>
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
    padding: 5px 0;
    color: #aaa;
    font-size: 0.85rem;
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

  .success {
    background: #1a2a1a;
    border: 1px solid #4ade80;
    color: #4ade80;
    padding: 12px;
    border-radius: 8px;
    margin-top: 15px;
  }

  .progress {
    background: #1a1a2a;
    border: 1px solid #6b9bff;
    color: #6b9bff;
    padding: 12px;
    border-radius: 8px;
    margin-top: 15px;
  }

  .error {
    background: #2a1a1a;
    border: 1px solid #f87171;
    color: #f87171;
    padding: 12px;
    border-radius: 8px;
    margin-top: 15px;
  }
</style>
