# Changelog

All notable changes to KDEsktop-copycat will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release functionality
- Plasma configuration parsing
- Interactive TUI interface
- Bundle export system
- KWin configuration scanning
- Restoration script generation

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.1.0] - 2024-02-09

### Added
- **Configuration Parser**: Parse KDE Plasma 6 configuration files (`plasma-org.kde.plasma.desktop-appletsrc`)
- **Interactive TUI**: Terminal-based interface for exploring desktop layouts
  - Plasma tab with desktop and panel browsing
  - KWin tab with effects and window rules
  - Help tab with navigation instructions
  - Arrow key navigation with expand/collapse functionality
- **Bundle Export**: Create portable desktop layout bundles
  - JSON layout data export
  - Restoration script generation (JavaScript and Bash)
  - Optional configuration snapshot inclusion
  - Optional plasmoid bundling for user-installed widgets
- **KWin Integration**: Scan and backup KWin configuration
  - Effects and scripts detection
  - Window rules parsing
  - Task switcher settings
  - Tier-1 configuration summary
- **Command-Line Interface**:
  - `scan` command for JSON output
  - `tui` command for interactive exploration
  - `export` command for bundle creation
  - Comprehensive CLI argument parsing with `clap`
- **Data Models**: Rust structs for Plasma configuration representation
  - Layout, Containment, Applet structures
  - KWinScan and KWinSummary models
  - Serde serialization support
- **Error Handling**: Comprehensive error handling with `anyhow`
- **Logging**: Debug logging support for troubleshooting
- **Documentation**: Complete inline documentation and examples

### Technical Details
- **Minimum Rust Version**: 1.70+
- **Dependencies**: anyhow, clap, crossterm, dirs, ratatui, regex, serde, serde_json, walkdir
- **Platform**: KDE Plasma 6 specific
- **License**: MIT
- **Architecture**: Modular design with separate parser, TUI, export, and KWin modules

### Known Limitations
- KDE Plasma 6 specific (not compatible with older versions)
- Requires KDE environment for full functionality
- Limited to user-installed plasmoids (system plasmoids excluded)
- Terminal-only interface (no GUI)

---

## How to Upgrade

### From Source
```bash
git pull origin main
cargo build --release
```

### From Binary (if available)
Download the latest release from the GitHub releases page.

### Configuration Migration
No configuration migration is needed for version 0.1.0, as this is the initial release.

---

## Release Policy

- **Major releases** may include breaking changes
- **Minor releases** add new features in a backward-compatible manner
- **Patch releases** contain bug fixes only

Always review the upgrade notes before updating to a new major version.