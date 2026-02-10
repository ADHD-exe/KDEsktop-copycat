# Examples

This directory contains example usage and configurations for KDEsktop-copycat.

## Example Bundles

### Basic Layout Export
```bash
# Export current Plasma layout
kdesktop-copycat export --out ~/plasma-backup

# Export with snapshot and plasmoids
kdesktop-copycat export --out ~/plasma-backup --snapshot --plasmoids

# Export with verbose output
kdesktop-copycat export --out ~/plasma-backup --verbose
```

### Custom Configuration Paths
```bash
# Export with custom paths
kdesktop-copycat export \
  --out ./my-bundle \
  --file ~/.config/plasma-org.kde.plasma.desktop-appletsrc \
  --kwinrc ~/.config/kwinrc
```

### Interactive Exploration
```bash
# Launch TUI to explore current layout
kdesktop-copycat tui

# Parse and output JSON
kdesktop-copycat scan > my-layout.json
```

## Example Output

### JSON Structure
```json
{
  "layout": {
    "containments": [
      {
        "id": "1",
        "plugin": "org.kde.desktopcontainment",
        "formfactor": 0,
        "location": 0,
        "applets": [
          {
            "id": "2",
            "plugin": "org.kde.plasma.digitalclock",
            "config": {
              "time_format": "24h"
            }
          }
        ]
      }
    ],
    "kwin": {
      "effects": {
        "blur": true,
        "desktop_grid": true
      },
      "window_rules": [
        {
          "description": "Terminal settings",
          "windowclass": "konsole"
        }
      ]
    }
  }
}
```

### Bundle Structure
```
plasma-layout-bundle-2024-02-09_15-30-45/
├── layout.json                    # Parsed configuration data
├── scripts/                       # Restoration scripts
│   ├── restore-layout.js         # JavaScript restoration via D-Bus
│   ├── restore-portable.sh       # Portable restoration script
│   └── restore-snapshot.sh       # Snapshot restoration script
├── snapshot/                      # Optional configuration snapshot
│   └── plasma-org.kde.plasma.desktop-appletsrc
└── plasmoids/                     # Optional bundled plasmoids
    └── [plasmoid-id]/
```

## Restoration Examples

### Automatic Restoration
```bash
# Navigate to extracted bundle
cd plasma-layout-bundle-2024-02-09_15-30-45

# Run restoration script
./scripts/restore-portable.sh

# Confirm Plasma restart when prompted
```

### Manual Restoration Steps
```bash
# 1. Install any bundled plasmoids
cd plasmoids/
for plasmoid in */; do
  kpackagetool6 -i "$plasmoid"
done

# 2. Apply layout via JavaScript
qmljs ../scripts/restore-layout.js

# 3. Restart Plasma shell
kquitapp6 plasmashell && plasmashell &
```

## Advanced Usage

### Debug Mode
```bash
# Enable debug logging
RUST_LOG=debug kdesktop-copycat tui
RUST_LOG=debug kdesktop-copycat export --verbose
```

### Script Integration
```bash
#!/bin/bash
# Backup Plasma layout automatically

BUNDLE_DIR="$HOME/plasma-backups/$(date +%Y-%m-%d_%H-%M-%S)"
mkdir -p "$BUNDLE_DIR"

kdesktop-copycat export --out "$BUNDLE_DIR" --snapshot --plasmoids

echo "Plasma layout backed up to: $BUNDLE_DIR"
```

### Multiple Profile Management
```bash
# Save different desktop setups
kdesktop-copycat export --out ~/.config/plasma-profiles/work
kdesktop-copycat export --out ~/.config/plasma-profiles/gaming
kdesktop-copycat export --out ~/.config/plasma-profiles/minimal

# Restore a profile
cd ~/.config/plasma-profiles/work
./scripts/restore-portable.sh
```

## Configuration Examples

### Typical Plasma Configuration Files
- **Main Config**: `~/.config/plasma-org.kde.plasma.desktop-appletsrc`
- **KWin Config**: `~/.config/kwinrc`
- **KWin Rules**: `~/.config/kwinrulesrc`

### Common Layout Patterns

#### Minimal Desktop
- Single panel with application launcher
- Digital clock
- Show Desktop widget
- No additional applets

#### Productivity Setup
- Top panel with task manager
- Bottom panel with system tray
- Folder view on desktop
- Widget for notes/tasks
- Window tiling KWin effects

#### Gaming Setup
- Minimal panel for essential apps
- Game launchers on desktop
- Performance-optimized KWin settings
- Resource monitor widgets

## Troubleshooting Examples

### Common Issues and Solutions

#### Missing Configuration Files
```bash
# Find your Plasma configuration
find ~/.config -name "*plasma*appletsrc*"

# Use custom path if needed
kdesktop-copycat scan --file ~/.kde4/share/config/plasma-appletsrc
```

#### Permission Issues
```bash
# Check file permissions
ls -la ~/.config/plasma-org.kde.plasma.desktop-appletsrc

# Fix ownership if needed
chown $USER:$USER ~/.config/plasma-org.kde.plasma.desktop-appletsrc
```

#### TUI Display Issues
```bash
# Ensure terminal supports UTF-8
export LANG=en_US.UTF-8

# Use larger terminal window
kdesktop-copycat tui
```

## Integration Examples

### With Backup Tools
```bash
#!/bin/bash
# Integrate with restic backup system

# Export Plasma layout
LAYOUT_BUNDLE="/tmp/plasma-layout-$(date +%s)"
kdesktop-copycat export --out "$LAYOUT_BUNDLE"

# Backup with restic
restic backup "$LAYOUT_BUNDLE" --tag plasma-layout

# Cleanup
rm -rf "$LAYOUT_BUNDLE"
```

### With Version Control
```bash
#!/bin/bash
# Track Plasma configuration changes in Git

cd ~/.config/plasma-configs

# Export current layout
kdesktop-copycat export --out "./current"

# Commit changes
git add .
git commit -m "Plasma layout update $(date)"
git push origin main
```

### With System Automation
```bash
# Systemd user service for automatic backups

[Unit]
Description=Plasma Layout Backup
Wants=network-online.target

[Service]
Type=oneshot
ExecStart=/usr/local/bin/kdesktop-copycat export --out %h/.config/plasma-backups/$(date +%%Y-%%m-%%d_%%H-%%M-%%S)

[Install]
WantedBy=timers.target
```

---

For more examples and use cases, see the [main README](../README.md) or open an issue to request specific examples.