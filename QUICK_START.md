# Quick Start - DOCX Accurate Page Numbers

## Installation

```bash
# Install LibreOffice
sudo apt-get update
sudo apt-get install -y libreoffice-writer libreoffice-core libreoffice-common

# Build and run
cargo build --release
cargo run --release
```

## Usage

1. Open http://localhost:4202
2. Create index: `content:text`
3. Import DOCX file (will auto-convert to PDF)
4. Search for terms
5. See accurate page numbers!

## What Changed

- DOCX files now convert to PDF server-side using LibreOffice
- PDF extracted page-by-page with lopdf
- Page numbers match your document viewer exactly
- Line numbers are relative to each page

## Files Modified

- `lnx-server/src/apis/document.rs` - Added conversion endpoint
- `lnx-server/Cargo.toml` - Added lopdf, tempfile
- `frontend/src/App.svelte` - Use server conversion for DOCX
- `Dockerfile` - Added LibreOffice

Done! Your DOCX page numbers will now be accurate.
