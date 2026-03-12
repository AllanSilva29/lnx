export async function handleFileImport(files, index, api, options = {}) {
  const result = { success: false, message: '', progress: '', error: null, documents: [] };
  const { onProgress } = options;
  
  if (!files || files.length === 0) {
    result.error = 'No files selected';
    return result;
  }

  const totalFiles = files.length;
  
  const updateProgress = (processed, currentFileName, message, subProgress = 0, delay = 0) => {
    // Calculate base progress for completed files + sub-progress for current file
    const basePercentage = (processed / totalFiles) * 100;
    const subPercentage = (subProgress / 100) * (100 / totalFiles); // subProgress is 0-100 for current file
    const percentage = Math.round(basePercentage + subPercentage);
    
    if (onProgress) {
      onProgress({
        percentage: Math.min(percentage, 100),
        currentFile: currentFileName,
        processed,
        total: totalFiles,
        message
      });
    }
    
    // Add delay for visibility if specified
    if (delay > 0) {
      return new Promise(resolve => setTimeout(resolve, delay));
    }
    return Promise.resolve();
  };

  await updateProgress(0, '', `Preparing to import ${totalFiles} file(s)...`, 0, 500);

  for (let i = 0; i < files.length; i++) {
    const file = files[i];
    const filename = file.name;
    const ext = filename.split('.').pop().toLowerCase();
    
    await updateProgress(i, filename, `Processing ${filename}...`, 0, 300);
    
    try {
      // For DOCX/PDF, use the convert endpoint
      if (ext === 'docx' || ext === 'doc' || ext === 'pdf') {
        await updateProgress(i, filename, `Converting ${filename}...`, 25, 400);
        const convertResult = await api.convertDocument(index, file);
        
        if (!convertResult.success) {
          throw new Error(convertResult.error || 'Conversion failed');
        }
        
        await updateProgress(i, filename, `Indexing converted content...`, 75, 400);
        
        result.documents.push({ 
          filename, 
          pages: convertResult.pages_indexed, 
          type: ext 
        });
        
        await updateProgress(i + 1, filename, `Successfully imported ${filename} (${convertResult.pages_indexed} pages)`, 0, 500);
      } else {
        // For text files, read and index directly
        await updateProgress(i, filename, `Reading ${filename}...`, 25, 400);
        const text = await readFile(file);
        const id = crypto.randomUUID();
        const document = { content: text, type: ext };
        
        await updateProgress(i, filename, `Indexing ${filename}...`, 75, 600);
        await api.addDocument(index, id, document, filename);
        result.documents.push({ filename, pages: 1, type: ext });
        
        await updateProgress(i + 1, filename, `Successfully imported ${filename}`, 0, 500);
      }
      
    } catch (e) {
      const errorMessage = `Failed to import ${file.name}: ${e.message}`;
      await updateProgress(i + 1, filename, errorMessage, 0, 1000);
      
      if (!result.error) {
        result.error = errorMessage;
      }
    }
  }

  if (!result.error) {
    result.success = true;
    result.message = `Successfully imported ${result.documents.length} document(s)`;
    await updateProgress(totalFiles, '', result.message, 0, 1000);
  } else {
    result.message = `Import completed with ${result.documents.length} successful import(s). Some files had errors.`;
    await updateProgress(totalFiles, '', result.message, 0, 1000);
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
