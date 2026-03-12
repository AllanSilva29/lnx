export async function handleFileImport(files, index, api) {
  const result = { success: false, message: '', progress: '', error: null, documents: [] };
  
  if (!files || files.length === 0) {
    result.error = 'No files selected';
    return result;
  }

  result.progress = `Processing ${files.length} file(s)...`;

  for (const file of files) {
    try {
      const filename = file.name;
      const ext = filename.split('.').pop().toLowerCase();
      
      // For DOCX/PDF, use the convert endpoint
      if (ext === 'docx' || ext === 'doc' || ext === 'pdf') {
        result.progress = `Converting ${filename}...`;
        const convertResult = await api.convertDocument(index, file);
        
        if (!convertResult.success) {
          throw new Error(convertResult.error || 'Conversion failed');
        }
        
        result.documents.push({ 
          filename, 
          pages: convertResult.pages_indexed, 
          type: ext 
        });
      } else {
        // For text files, read and index directly
        const text = await readFile(file);
        const id = crypto.randomUUID();
        const document = { content: text, type: ext };
        
        await api.addDocument(index, id, document, filename);
        result.documents.push({ filename, pages: 1, type: ext });
      }
      
      result.progress = `Processed ${result.documents.length}/${files.length} file(s)`;
    } catch (e) {
      result.error = `Failed to import ${file.name}: ${e.message}`;
    }
  }

  if (!result.error) {
    result.success = true;
    result.message = `Successfully imported ${result.documents.length} document(s)`;
  }

  return result;
}

function readFile(file) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(reader.result);
    reader.onerror = () => reject(new Error('Failed to read file'));
    reader.readAsText(file);
  });
}
