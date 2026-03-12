# DOCX to PDF Conversion Setup

To enable accurate page number extraction from DOCX files, the server needs LibreOffice installed.

## Installation

### Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install -y libreoffice-writer libreoffice-core libreoffice-common poppler-utils
```

### macOS
```bash
brew install --cask libreoffice
brew install poppler
```

### Docker
The Dockerfile already includes LibreOffice and poppler-utils. Just rebuild:
```bash
docker-compose build
```

## How It Works

1. User uploads a DOCX file through the frontend
2. Frontend sends file to `/api/v0/documents/convert` endpoint
3. Backend uses LibreOffice headless mode to convert DOCX → PDF
4. PDF is parsed page-by-page using `lopdf` library
5. Each page is indexed separately with accurate page metadata
6. Search results show exact page numbers matching the original document

## Technical Details

**Conversion Pipeline:**
- DOCX → LibreOffice (headless) → PDF
- PDF → pdftotext (page-by-page) → Text per page
- Each page → Tantivy index with page number

**Text Extraction Methods (in order of preference):**
1. `pdftotext -layout` - Best quality, preserves layout
2. `pdftotext` page-by-page - Accurate page separation
3. `pdf-extract` - Fallback if pdftotext not available

## Benefits

- **Accurate page numbers**: Match exactly what you see in LibreOffice/Word viewers
- **Accurate line numbers**: Within each page, relative to extracted text
- **Better search results**: Know exactly where to find content in the original document
- **Consistent results**: Same page numbers across different viewers

## File Support

| File Type | Page Numbers | Method |
|-----------|--------------|--------|
| PDF | ✅ Accurate | Direct extraction with lopdf |
| DOCX | ✅ Accurate | Convert to PDF first |
| DOC | ✅ Accurate | Convert to PDF first |
| TXT | ❌ Sections | No pages, grouped by lines |
| HTML | ❌ Sections | No pages, grouped by elements |
| CSV | ❌ Rows | No pages, each row is a document |

## Testing

After installation, test the conversion:

```bash
# Check LibreOffice is available
libreoffice --version

# Upload a DOCX file through the UI
# Search for a term
# Verify page numbers match your document viewer
```

## Troubleshooting

**Error: "LibreOffice conversion failed"**
- Make sure LibreOffice is installed: `which libreoffice`
- Check permissions: LibreOffice needs write access to temp directory
- Try manual conversion: `libreoffice --headless --convert-to pdf test.docx`

**Error: "PDF extraction failed"**
- The generated PDF might be corrupted
- Check LibreOffice logs
- Try opening the DOCX in LibreOffice manually first

**Page numbers still don't match**
- This can happen if the DOCX has complex formatting
- Page breaks in DOCX depend on page size, margins, fonts
- The conversion uses LibreOffice's default settings
- For best results, use simple formatting in your DOCX files

