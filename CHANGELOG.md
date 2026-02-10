# Changelog

## [0.1.0] - 2024-02-09

First release! This tool does what I need:

### Core features
- Parse Plasma applet configs
- Interactive TUI for browsing layouts  
- Export portable bundles
- Scan KWin settings
- Generate restoration scripts

### Tech details
- Written in Rust (1.70+)
- Uses ratatui for the terminal UI
- Serde for JSON handling
- Works with KDE Plasma 6

### Known issues
- Only works with Plasma 6 (not older versions)
- Some exotic plasmoids might not export properly
- Terminal-only interface (GUI would be nice but TUI works great)

That's it for now. Just wanted something that actually works for backing up Plasma setups.