<script>
  import FileImport from './FileImport.svelte';
  import { handleFileImport } from './importHandler.js';

  export let indexes = [];
  export let selectedIndex = '';
  export let loading = false;
  export let error = null;
  export let api;

  let importFiles = [];
  let importResult = null;
  let importProgress = '';
  let importedDocuments = [];
  let progressPercentage = 0;
  let currentFile = '';
  let processedCount = 0;
  let totalCount = 0;

  async function doImport() {
    loading = true;
    progressPercentage = 0;
    currentFile = '';
    processedCount = 0;
    totalCount = importFiles.length;
    importProgress = `Starting import of ${totalCount} file(s)...`;
    
    try {
      const result = await handleFileImport(importFiles, selectedIndex, api, {
        onProgress: (progress) => {
          progressPercentage = progress.percentage || 0;
          currentFile = progress.currentFile || '';
          processedCount = progress.processed || 0;
          importProgress = progress.message || '';
        }
      });
      
      importResult = result.message;
      importProgress = result.progress;
      error = result.error;
      
      if (result.documents) {
        importedDocuments = [...importedDocuments, ...result.documents];
      }
      
      if (result.success) {
        importFiles = [];
        progressPercentage = 100;
        importProgress = 'Import completed successfully!';
      }
    } catch (e) {
      error = `Import failed: ${e.message}`;
      importProgress = '';
    } finally {
      loading = false;
    }
  }

  function clearImportedDocs() {
    if (confirm('Clear all imported documents from this session?')) {
      importedDocuments = [];
      importResult = null;
      progressPercentage = 0;
      currentFile = '';
      processedCount = 0;
      totalCount = 0;
      importProgress = '';
    }
  }

  function onFileChange(e) {
    importFiles = Array.from(e.target.files);
    // Reset progress state when files change
    progressPercentage = 0;
    currentFile = '';
    processedCount = 0;
    totalCount = 0;
    importProgress = '';
    importResult = null;
    error = null;
  }
</script>

<section class="import-section">
  <h3>Import Documents</h3>
  
  <div class="options">
    <label>
      Target Index:
      <select bind:value={selectedIndex}>
        {#each indexes as idx}
          <option value={idx}>{idx}</option>
        {/each}
      </select>
    </label>
  </div>

  <FileImport 
    bind:importFiles 
    {loading} 
    {importResult} 
    {importProgress} 
    {error}
    {progressPercentage}
    {currentFile}
    {processedCount}
    {totalCount}
    onImport={doImport}
    onFileChange={onFileChange}
  />

  {#if importedDocuments.length > 0}
    <div class="imported-docs">
      <div class="docs-header">
        <h4>Imported Documents in Index "{selectedIndex}":</h4>
        <button on:click={clearImportedDocs} class="clear-btn">Clear List</button>
      </div>
      <ul class="doc-list">
        {#each importedDocuments as doc}
          <li>
            <span class="doc-name">{doc.filename}</span>
            <span class="doc-info">{doc.pages} pages/sections • {doc.type.toUpperCase()}</span>
          </li>
        {/each}
      </ul>
    </div>
  {/if}
</section>

<style>
  h3 {
    margin: 0 0 15px 0;
    color: #ff6b6b;
    font-size: 1.1rem;
  }

  .options {
    display: flex;
    gap: 20px;
    flex-wrap: wrap;
    align-items: center;
    margin-bottom: 20px;
  }

  .options label {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.9rem;
    color: #888;
  }

  .options select {
    padding: 6px 10px;
    border: 1px solid #333;
    border-radius: 4px;
    background: #0f0f0f;
    color: #fff;
  }

  .imported-docs {
    margin-top: 20px;
    padding: 15px;
    background: #0f0f0f;
    border-radius: 8px;
    border: 1px solid #333;
  }

  .docs-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .imported-docs h4 {
    margin: 0 0 10px 0;
    font-size: 0.9rem;
    color: #4ade80;
  }

  .doc-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .doc-list li {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 10px;
    margin-bottom: 5px;
    background: #1a1a1a;
    border-radius: 4px;
  }

  .doc-name {
    color: #ff6b6b;
    font-weight: 500;
  }

  .doc-info {
    color: #888;
    font-size: 0.8rem;
  }

  .clear-btn {
    padding: 4px 12px;
    font-size: 0.8rem;
    background: #333;
    color: #fff;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .clear-btn:hover {
    background: #444;
  }
</style>
