# KDEsktop-copycat

I got tired of losing my Plasma setup every time I reinstalled or messed up my config, so I built this tool to backup and restore KDE Plasma desktop layouts.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-KDE%20Plasma%206-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)

## What it does

- **Scans** your current Plasma panels and widgets
- **Shows** you everything in a nice TUI
- **Exports** portable bundles you can restore later
- **Backs up** KWin settings too

## Quick start

```bash
# Install
cargo install kdesktop-copycat

# Check out your current setup
kdesktop-copycat tui

# Export everything
kdesktop-copycat export --out ~/my-plasma-backup

# Just see the JSON output
kdesktop-copycat scan
```

## How it works

The tool parses your `plasma-org.kde.plasma.desktop-appletsrc` file (and optionally `kwinrc`) to understand:

- Your panels and their widgets
- Desktop folders and applets  
- Window rules and effects
- All the little settings that make your desktop yours

Then it can create restoration scripts that rebuild your layout using Plasma's JavaScript API.

## Why I built this

I switch between my desktop and laptop a lot, and I'm constantly tweaking my Plasma setup. Previously I'd just copy config files around, but that's fragile and breaks across different Plasma versions. This tool actually recreates your layout programmatically, which is way more reliable.

## Requirements

- Rust 1.70+
- KDE Plasma 6
- Linux

## Usage

```bash
# Interactive viewer - great for exploring
kdesktop-copycat tui

# Export to a bundle you can copy to another machine
kdesktop-copycat export --out ./my-setup --snapshot --bundle-plasmoids

# Just parse and show JSON
kdesktop-copycat scan

# Use custom config paths
kdesktop-copycat scan --file /path/to/appletsrc --kwinrc /path/to/kwinrc
```

## Bundles

When you export, it creates:
- `layout.json` - Your desktop layout data
- `scripts/restore-*.sh` - Restoration scripts  
- `snapshot/` - Optional config file backup
- `plasmoids/` - Your custom widgets (if any)

To restore, just run the restoration scripts from the bundle.

## Contributing

Feel free to open issues or PRs. Mostly I need:
- More robust error handling for weird configs
- Better support for exotic plasmoids
- GUI maybe? (TUI is pretty solid though)

## License

MIT - do whatever you want with this.