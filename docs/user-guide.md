# User Guide

This comprehensive guide covers how to use KDEsktop-copycat effectively.

## Getting Started

### First Run

After installation, verify everything works:

```bash
# Check version
kdesktop-copycat --version

# Test basic functionality
kdesktop-copycat scan

# Launch the TUI
kdesktop-copycat tui
```

### Understanding the Interface

KDEsktop-copycat provides three main interfaces:

1. **Command Line** - Quick operations and scripting
2. **Interactive TUI** - Explore configurations visually
3. **Bundle System** - Export and restore desktop layouts

## Command Line Interface

### Basic Commands

#### `scan` - Parse and Display Configuration
```bash
# Basic scan (outputs JSON)
kdesktop-copycat scan

# Save to file
kdesktop-copycat scan > my-layout.json

# Custom configuration file
kdesktop-copycat scan --file /path/to/plasma-appletsrc

# Pretty-print JSON
kdesktop-copycat scan | jq '.'
```

#### `tui` - Interactive Terminal Interface
```bash
# Launch TUI
kdesktop-copycat tui

# With custom configuration
kdesktop-copycat tui --file /path/to/plasma-appletsrc

# With debug logging
RUST_LOG=debug kdesktop-copycat tui
```

#### `export` - Create Portable Bundle
```bash
# Basic export
kdesktop-copycat export --out ~/my-plasma-bundle

# Include snapshot and plasmoids
kdesktop-copycat export --out ~/backup --snapshot --plasmoids

# Verbose output
kdesktop-copycat export --out ./bundle --verbose

# Custom paths
kdesktop-copycat export \
  --out ./bundle \
  --file /path/to/appletsrc \
  --kwinrc /path/to/kwinrc
```

### Advanced Options

#### Global Options
```bash
# Custom config file
kdesktop-copycat --config ~/.config/kdesktop-copycat/custom.toml tui

# Debug mode
RUST_LOG=debug kdesktop-copycat scan

# Quiet mode
kdesktop-copycat --quiet export --out ./bundle

# Help
kdesktop-copycat --help
kdesktop-copycat export --help
```

## Interactive TUI Guide

### Navigation Basics

The TUI uses keyboard navigation:

- **↑/↓** - Navigate up/down
- **Enter** - Expand/collapse items
- **Tab/Shift+Tab** - Switch between tabs
- **q** - Quit
- **?** - Show help

### Tab Overview

#### Plasma Tab
Displays your Plasma desktop layout:

```
📁 Plasma Configuration
├── 🖥️ Desktop 1 (ID: 1)
│   ├── 📱 Folder View (org.kde.folder)
│   ├── ⏰ Digital Clock (org.kde.digitalclock)
│   └── 📋 Clipboard (org.kde.plasma.clipboard)
├── 📱 Panel (ID: 2)
│   ├── 🚀 Application Launcher (org.kde.kickoff)
│   ├── 📊 Task Manager (org.kde.taskmanager)
│   └── 🔔 System Tray (org.kde.systemtray)
└── 🖥️ Desktop 2 (ID: 3)
    └── 📱 Wallpaper (org.kde.image)
```

#### KWin Tab
Shows KWin configuration:

```
⚡ KWin Configuration
├── ✨ Effects
│   ├── Blur: Enabled
│   ├── Desktop Grid: Enabled
│   └── Present Windows: Enabled
├── 📜 Scripts
│   └── KWin Overview Script: Enabled
└── 📋 Window Rules
    └── Terminal settings: Konsole
```

#### Help Tab
Displays navigation help and keybindings.

### Advanced TUI Features

#### Search and Filter
```bash
# Launch with search mode
kdesktop-copycat tui --search-mode

# Search for specific widgets
kdesktop-copycat tui --filter "clock"

# Search by plugin ID
kdesktop-copycat tui --filter "org.kde.digitalclock"
```

#### Detailed View Mode
```bash
# Show detailed configuration
kdesktop-copycat tui --detailed

# Limit depth for large configurations
kdesktop-copycat tui --max-depth 3
```

## Bundle System

### Understanding Bundles

A bundle contains everything needed to restore your desktop:

```
plasma-layout-bundle-2024-02-09_15-30-45/
├── layout.json                    # Parsed configuration data
├── scripts/                       # Restoration scripts
│   ├── restore-layout.js         # JavaScript restoration
│   ├── restore-portable.sh       # Portable restoration
│   └── restore-snapshot.sh       # Snapshot restoration
├── snapshot/                      # Optional config snapshot
│   └── plasma-org.kde.plasma.desktop-appletsrc
└── plasmoids/                     # Optional bundled plasmoids
    └── [plasmoid-id]/
```

### Creating Bundles

#### Basic Bundle
```bash
# Simple export
kdesktop-copycat export --out ~/plasma-backup

# This creates a minimal bundle with:
# - layout.json (configuration data)
# - Basic restoration scripts
```

#### Complete Bundle
```bash
# Full backup with all features
kdesktop-copycat export \
  --out ~/complete-backup \
  --snapshot \
  --plasmoids \
  --verbose

# This includes:
# - Configuration snapshot
# - User-installed plasmoids
# - All restoration scripts
# - Detailed logging
```

#### Automated Backups
```bash
#!/bin/bash
# Script for automated daily backups

BACKUP_DIR="$HOME/plasma-backups/$(date +%Y-%m-%d_%H-%M-%S)"
mkdir -p "$BACKUP_DIR"

kdesktop-copycat export \
  --out "$BACKUP_DIR" \
  --snapshot \
  --plasmoids \
  --quiet

echo "Plasma layout backed up to: $BACKUP_DIR"

# Keep only last 7 days
find ~/plasma-backups -type d -mtime +7 -exec rm -rf {} +
```

### Restoring from Bundles

#### Automatic Restoration
```bash
# Navigate to bundle directory
cd plasma-layout-bundle-2024-02-09_15-30-45

# Run the portable restoration script
./scripts/restore-portable.sh

# Follow prompts:
# - Install missing plasmoids (if bundled)
# - Apply layout configuration
# - Restart Plasma shell
```

#### Manual Restoration
```bash
# Step 1: Install plasmoids (if bundled)
cd plasmoids/
for plasmoid in */; do
  kpackagetool6 -i "$plasmoid"
done

# Step 2: Apply layout via JavaScript
qmljs ../scripts/restore-layout.js

# Step 3: Restart Plasma shell
kquitapp6 plasmashell && plasmashell &
```

#### Restoration from Snapshot
```bash
# Use snapshot restoration (exact copy)
cd plasma-layout-bundle-2024-02-09_15-30-45

# Backup current configuration first
cp ~/.config/plasma-org.kde.plasma.desktop-appletsrc \
   ~/.config/plasma-org.kde.plasma.desktop-appletsrc.backup

# Apply snapshot
./scripts/restore-snapshot.sh

# Restart Plasma
kquitapp6 plasmashell && plasmashell &
```

## Advanced Usage

### Scripting and Automation

#### Batch Processing
```bash
#!/bin/bash
# Process multiple configurations

CONFIG_DIR="$HOME/plasma-configs"
OUTPUT_DIR="$HOME/plasma-exports"

mkdir -p "$OUTPUT_DIR"

for config in "$CONFIG_DIR"/*.appletsrc; do
  basename=$(basename "$config" .appletsrc)
  kdesktop-copycat export \
    --file "$config" \
    --out "$OUTPUT_DIR/$basename" \
    --quiet
done
```

#### Conditional Operations
```bash
#!/bin/bash
# Backup only if changes detected

LAYOUT_FILE="$HOME/.cache/current-layout.json"
BUNDLE_DIR="$HOME/plasma-backups/$(date +%Y-%m-%d_%H-%M-%S)"

# Export current layout
kdesktop-copycat scan > "$LAYOUT_FILE.tmp"

# Compare with previous (if exists)
if [[ -f "$LAYOUT_FILE" ]]; then
  if diff "$LAYOUT_FILE" "$LAYOUT_FILE.tmp" > /dev/null; then
    echo "No changes detected"
    rm "$LAYOUT_FILE.tmp"
    exit 0
  fi
fi

# Changes detected, create backup
mv "$LAYOUT_FILE.tmp" "$LAYOUT_FILE"
kdesktop-copycat export --out "$BUNDLE_DIR" --snapshot
echo "Changes detected and backed up to: $BUNDLE_DIR"
```

### Integration with Other Tools

#### Version Control
```bash
# Track Plasma configuration in Git
cd ~/.config/plasma-configs

# Export current layout
kdesktop-copycat export --out ./current

# Commit changes
git add .
git commit -m "Plasma layout update $(date)"
git push origin main
```

#### Backup Systems
```bash
# Integrate with restic
#!/bin/bash

LAYOUT_BUNDLE="/tmp/plasma-layout-$(date +%s)"
kdesktop-copycat export --out "$LAYOUT_BUNDLE"

# Backup with restic
restic backup "$LAYOUT_BUNDLE" --tag plasma-layout

# Cleanup
rm -rf "$LAYOUT_BUNDLE"
```

## Configuration and Customization

### Custom Configuration Files
```bash
# Use specific configuration files
kdesktop-copycat \
  --file ~/.config/plasma-workspace/appletsrc \
  --kwinrc ~/.config/kwin-workspace/kwinrc \
  export --out ./bundle
```

### Environment Variables
```bash
# Set configuration via environment
export KDESKTOP_COPYCAT_PLASMA_CONFIG="/path/to/appletsrc"
export KDESKTOP_COPYCAT_EXPORT_DIR="/path/to/exports"
export KDESKTOP_COPYCAT_LOG_LEVEL="debug"

kdesktop-copycat export --out ./bundle
```

### Profile Management
```bash
# Multiple profile management
alias plasma-work="kdesktop-copycat export --out ~/.config/plasma-profiles/work"
alias plasma-home="kdesktop-copycat export --out ~/.config/plasma-profiles/home"
alias plasma-minimal="kdesktop-copycat export --out ~/.config/plasma-profiles/minimal"

# Restore profiles
alias restore-work="cd ~/.config/plasma-profiles/work && ./scripts/restore-portable.sh"
alias restore-home="cd ~/.config/plasma-profiles/home && ./scripts/restore-portable.sh"
```

## Troubleshooting

### Common Issues

#### "Configuration file not found"
```bash
# Find your Plasma configuration
find ~/.config -name "*plasma*appletsrc*"

# Check if you're in a Plasma session
echo $XDG_CURRENT_DESKTOP
ps aux | grep plasmashell
```

#### "TUI display issues"
```bash
# Ensure terminal supports UTF-8
export LANG=en_US.UTF-8

# Use larger terminal window
# Minimum 80x24 recommended

# Try basic mode
kdesktop-copycat scan
```

#### "Bundle restoration fails"
```bash
# Check bundle integrity
ls -la bundle/
cat bundle/layout.json | jq .

# Check restoration scripts
cat bundle/scripts/restore-portable.sh

# Manual step-by-step restoration
cd bundle
./scripts/restore-portable.sh
```

### Debug Mode
```bash
# Enable debug logging
RUST_LOG=debug kdesktop-copycat tui
RUST_LOG=debug kdesktop-copycat export --verbose

# Check configuration loading
RUST_LOG=debug kdesktop-copycat --validate-config
```

## Best Practices

### Regular Backups
- Schedule automatic backups using cron or systemd timers
- Include snapshots for exact restoration
- Keep multiple backup versions
- Test restoration periodically

### Bundle Management
- Use descriptive names for bundles
- Include system information for compatibility
- Document any special requirements
- Clean up old bundles regularly

### Security Considerations
- Review bundles before importing from others
- Be careful with sensitive desktop configurations
- Store bundles in secure locations
- Use encryption for sensitive layouts

## Next Steps

- Explore [Advanced Configuration](configuration.md)
- Check out [Examples](../examples/)
- Join the community for tips and tricks
- Contribute to the project!

Enjoy managing your Plasma desktop layouts! 🎉