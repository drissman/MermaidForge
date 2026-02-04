# Changelog

All notable changes to MermaidForge will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned for v1.1.0
- Live editor with Monaco
- File watcher for auto-conversion
- PDF export
- Syntax validation
- Multi-language support (EN/FR)

## [1.0.0] - 2026-02-04

### Added
- Initial release of MermaidForge
- Standalone Mermaid to SVG/PNG conversion using headless Chrome
- Batch processing with progress tracking
- Portable mode (USB/OneDrive compatible)
- Installed mode (MSI installer)
- Settings management with JSON persistence
- Orange Business branding and theme
- Drag & drop file interface
- Recent files tracking
- System info display
- Cross-platform path handling (Windows/Linux/macOS)
- Comprehensive documentation
- GitHub Desktop integration guide
- CI/CD with GitHub Actions

### Features
- Convert `.mmd` files to:
  - SVG (transparent, light, dark backgrounds)
  - PNG (2048px, 4K)
- Parallel batch conversion (up to 4 files simultaneously)
- Automatic output directory management
- Dynamic path configuration (no hardcoded paths)
- Portable mode detection via `portable.flag`
- Config storage in:
  - Portable: `./config/`
  - Installed: `%APPDATA%/MermaidForge/`
- Output directory defaults:
  - Portable: `./output/`
  - Installed: `Documents/MermaidForge/output/`

### Technical
- **Backend**: Rust 1.75+ with Tauri 2.0
- **Frontend**: React 18 + TypeScript + Vite
- **Rendering**: headless_chrome with Mermaid.js 10.9.0
- **Build targets**: Portable .exe and MSI installer
- **Size**: ~150MB (Chromium embedded)

### Documentation
- README.md with installation and usage guide
- GITHUB_GUIDE.md for Git/GitHub beginners
- Architecture documentation
- API documentation for external integration (SOLARIS)

## [0.1.0] - 2026-02-03

### Added
- Project scaffolding
- Basic Tauri setup
- Initial repository structure

---

## Version History

- **v1.0.0** (2026-02-04) - Initial Release
- **v0.1.0** (2026-02-03) - Project Start

---

## Upgrade Guide

### From v0.x to v1.0.0

First stable release - no upgrade needed, just install v1.0.0.

---

## Support

- 🐛 Report bugs: https://github.com/VOTRE_USERNAME/MermaidForge/issues
- 💡 Feature requests: https://github.com/VOTRE_USERNAME/MermaidForge/discussions
- 📧 Contact: drissman@gmail.com

---

**Maintained by Driss NAAMANE @ Orange Business**
