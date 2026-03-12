const API_BASE = '/api/v0';

class LnxApi {
  constructor(baseUrl = API_BASE) {
    this.baseUrl = baseUrl;
    this.index = 'test';
  }

  async request(endpoint, options = {}) {
    const url = `${this.baseUrl}${endpoint}`;
    const config = {
      headers: {
        'Content-Type': 'application/json',
        ...options.headers,
      },
      ...options,
    };

    try {
      const response = await fetch(url, config);
      if (!response.ok) {
        let errorMessage = `HTTP ${response.status}: ${response.statusText}`;
        
        // Try to extract detailed error from response body
        try {
          const errorData = await response.json();
          if (errorData.error) {
            errorMessage = errorData.error;
          } else if (errorData.message) {
            errorMessage = errorData.message;
          } else if (typeof errorData === 'string') {
            errorMessage = errorData;
          } else {
            errorMessage = JSON.stringify(errorData);
          }
        } catch (e) {
          // If JSON parsing fails, try to get text
          try {
            const errorText = await response.text();
            if (errorText) {
              errorMessage = errorText;
            }
          } catch (textError) {
            // Keep the original HTTP error message
          }
        }
        
        throw new Error(errorMessage);
      }
      return await response.json();
    } catch (error) {
      console.error(`API Error [${endpoint}]:`, error);
      throw error;
    }
  }

  async createIndex(name, fields = {}) {
    return this.request('/index', {
      method: 'POST',
      body: JSON.stringify({ name, fields }),
    });
  }

  async listIndexes() {
    return this.request('/index');
  }

  async deleteIndex(name) {
    return this.request(`/index/${name}`, {
      method: 'DELETE',
    });
  }

  async addDocument(index, id, document, filename = null) {
    return this.request('/documents', {
      method: 'POST',
      body: JSON.stringify({ index, id, filename, document }),
    });
  }

  async search(query, options = {}) {
    console.log(`[API] Search called: query="${query}", index=${options.index}, limit=${options.limit}, semantic=${options.semantic}`);
    return this.request('/query/simple', {
      method: 'POST',
      body: JSON.stringify({
        index: options.index || this.index,
        query,
        limit: options.limit || 10,
        semantic: options.semantic === true, // Ensure it's always a boolean
      }),
    });
  }

  async getInfo() {
    return this.request('/info/summary');
  }

  async health() {
    return this.request('/health/check');
  }

  async convertDocument(index, file) {
    const formData = new FormData();
    formData.append('file', file);
    formData.append('index', index);

    const response = await fetch(`${this.baseUrl}/documents/convert`, {
      method: 'POST',
      body: formData,
    });

    if (!response.ok) {
      const text = await response.text();
      throw new Error(`HTTP ${response.status}: ${text}`);
    }

    return response.json();
  }
}

export const api = new LnxApi();
export default api;
