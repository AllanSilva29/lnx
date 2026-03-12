<script>
  import api from './api.js';

  export let indexes = [];
  export let selectedIndex = '';
  export let error = null;

  let newIndexName = '';
  let newIndexFields = 'title:text,content:text';

  export async function loadIndexes() {
    try {
      indexes = await api.listIndexes();
      if (indexes.length > 0 && !indexes.includes(selectedIndex)) {
        selectedIndex = indexes[0];
      }
    } catch (e) {
      console.error('Failed to load indexes:', e);
    }
  }

  async function createIndex() {
    if (!newIndexName.trim()) return;
    
    const fields = {};
    newIndexFields.split(',').forEach(f => {
      const [name, type_] = f.trim().split(':');
      if (name) fields[name] = type_ || 'text';
    });
    
    try {
      await api.createIndex(newIndexName, fields);
      await loadIndexes();
      selectedIndex = newIndexName;
      newIndexName = '';
      error = null;
    } catch (e) {
      error = e.message;
    }
  }

  async function deleteIndex(name) {
    if (!confirm(`Delete index "${name}"? This cannot be undone.`)) return;
    
    try {
      const result = await api.deleteIndex(name);
      console.log('Delete result:', result);
      if (result.success) {
        if (selectedIndex === name) {
          selectedIndex = '';
        }
        // Remove from local array immediately for instant feedback
        indexes = indexes.filter(i => i !== name);
      } else {
        error = result.error || 'Delete failed';
      }
    } catch (e) {
      console.error('Delete error:', e);
      error = e.message;
    }
  }
</script>

<section class="index-section">
  <h3>Create New Index</h3>
  <div class="create-index">
    <input type="text" bind:value={newIndexName} placeholder="Index name" class="input" />
    <input type="text" bind:value={newIndexFields} placeholder="title:text,content:text" class="input" />
    <button on:click={createIndex} class="btn">Create Index</button>
  </div>
  <p class="hint">Format: fieldname:type (text, string, i64, f64), comma separated</p>

  <h3>Existing Indexes</h3>
  <ul class="index-list">
    {#each indexes as idx}
      <li class:selected={idx === selectedIndex}>
        <span class="idx-name">{idx}</span>
        <button class="delete-btn" on:click={() => deleteIndex(idx)}>Delete</button>
      </li>
    {:else}
      <li class="empty">No indexes yet</li>
    {/each}
  </ul>
</section>

<style>
  h3 {
    margin: 0 0 15px 0;
    color: #ff6b6b;
    font-size: 1.1rem;
  }

  .create-index {
    display: flex;
    gap: 10px;
    margin-bottom: 10px;
  }

  .hint {
    font-size: 0.8rem;
    color: #666;
    margin: 0 0 20px 0;
  }

  .index-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .index-list li {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px;
    background: #0f0f0f;
    border-radius: 4px;
    margin-bottom: 5px;
  }

  .index-list li.selected {
    border: 1px solid #ff6b6b;
  }

  .index-list li.empty {
    color: #666;
    font-style: italic;
  }

  .idx-name {
    color: #fff;
  }

  .delete-btn {
    padding: 4px 10px;
    font-size: 0.75rem;
    background: #333;
    color: #f87171;
    border: 1px solid #f87171;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .delete-btn:hover {
    background: #f87171;
    color: #fff;
  }

  .input, .btn {
    padding: 12px 16px;
    font-size: 1rem;
    border: 2px solid #333;
    border-radius: 8px;
    background: #0f0f0f;
    color: #fff;
    outline: none;
  }

  .btn {
    background: #ff6b6b;
    border: none;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn:hover {
    background: #ff5252;
  }
</style>
