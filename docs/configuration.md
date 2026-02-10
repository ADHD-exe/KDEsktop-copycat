# KDEsktop-copycat Configuration

Configuration options for KDEsktop-copycat.

## Configuration File

KDEsktop-copycat looks for configuration in the following locations (in order):

1. `~/.config/kdesktop-copycat/config.toml`
2. `./kdesktop-copycat.toml`
3. Environment variables
4. Command-line arguments

## Default Configuration

```toml
[kdesktop-copycat]
# Default paths for Plasma configuration files
plasma_config = "~/.config/plasma-org.kde.plasma.desktop-appletsrc"
kwin_config = "~/.config/kwinrc"
kwin_rules = "~/.config/kwinrulesrc"

[export]
# Default export options
include_snapshot = false
include_plasmoids = false
output_dir = "~/.local/share/kdesktop-copycat/bundles"

[tui]
# Terminal UI preferences
theme = "default"
keybindings = "default"
max_depth = 5

[logging]
# Logging configuration
level = "info"
file = "~/.local/share/kdesktop-copycat/logs/kdesktop-copycat.log"
max_size = "10MB"
max_files = 5
```

## Environment Variables

You can override configuration using environment variables:

```bash
# Override configuration file location
export KDESKTOP_COPYCAT_CONFIG="/path/to/config.toml"

# Override specific settings
export KDESKTOP_COPYCAT_PLASMA_CONFIG="/custom/path/plasma-appletsrc"
export KDESKTOP_COPYCAT_LOG_LEVEL="debug"
export KDESKTOP_COPYCAT_EXPORT_DIR="/custom/export/path"
```

## Command Line Arguments

Command-line arguments take precedence over configuration files and environment variables:

```bash
# Use custom configuration file
kdesktop-copycat --config /path/to/config.toml tui

# Override specific options
kdesktop-copycat export \
  --out ./my-bundle \
  --file /path/to/plasma-appletsrc \
  --verbose
```

## Advanced Configuration

### Custom Themes

Create custom TUI themes in your configuration:

```toml
[tui.theme]
background = "black"
foreground = "white"
highlight = "cyan"
border = "blue"
title = "yellow"
text = "gray"
success = "green"
warning = "yellow"
error = "red"
```

### Custom Keybindings

Define custom keybindings for the TUI:

```toml
[tui.keybindings]
quit = "q"
help = "?"
expand = "enter"
collapse = "enter"
next_tab = "tab"
prev_tab = "shift+tab"
up = "up"
down = "down"
page_up = "pgup"
page_down = "pgdn"
home = "home"
end = "end"
```

### Export Options

Fine-tune export behavior:

```toml
[export]
# Include system information in exports
include_system_info = true

# Compression settings
compress_bundles = true
compression_level = 6

# File exclusions (glob patterns)
exclude_patterns = [
  "*.tmp",
  "*.log",
  "cache/*"
]

# Bundle naming format
bundle_name_format = "plasma-layout-%Y-%m-%d_%H-%M-%S"
```

### Logging Configuration

Configure detailed logging:

```toml
[logging]
# Log levels: error, warn, info, debug, trace
level = "info"

# Log to both file and console
targets = ["file", "console"]

# Rotate logs when they reach max_size
rotate = true

# Include timestamps and module information
include_timestamp = true
include_module = true

# Specific module log levels
[logging.modules]
parser = "debug"
tui = "info"
export = "info"
```

## Examples

### Development Configuration

For developers working on the project:

```toml
[kdesktop-copycat]
plasma_config = "./test-data/plasma-appletsrc"
kwin_config = "./test-data/kwinrc"

[logging]
level = "debug"
targets = ["console"]

[export]
output_dir = "./test-exports"
include_snapshot = true
```

### Minimal Configuration

For minimal resource usage:

```toml
[export]
include_snapshot = false
include_plasmoids = false
compress_bundles = false

[tui]
theme = "minimal"

[logging]
level = "warn"
```

### Production Configuration

For production/backup use:

```toml
[export]
include_snapshot = true
include_plasmoids = true
compress_bundles = true
compression_level = 9
include_system_info = true

[logging]
level = "info"
targets = ["file"]
max_size = "50MB"
max_files = 10
```

## Configuration Validation

KDEsktop-copycat validates configuration on startup and will:

1. **Warn** about deprecated settings
2. **Error** on invalid values
3. **Fall back** to defaults for missing settings
4. **Log** configuration loading details

You can validate your configuration:

```bash
kdesktop-copycat --validate-config
```

## Troubleshooting Configuration

### Common Issues

**Configuration file not found:**
```bash
# Check default locations
ls -la ~/.config/kdesktop-copycat/config.toml
ls -la ./kdesktop-copycat.toml

# Specify path explicitly
kdesktop-copycat --config /path/to/config.toml
```

**Invalid TOML syntax:**
```bash
# Validate TOML syntax
kdesktop-copycat --validate-config
# or
python3 -c "import toml; toml.load(open('config.toml'))"
```

**Permissions error:**
```bash
# Check file permissions
ls -la ~/.config/kdesktop-copycat/config.toml

# Fix permissions
chmod 644 ~/.config/kdesktop-copycat/config.toml
```

### Debug Configuration Loading

Enable debug logging to see configuration loading details:

```bash
RUST_LOG=debug kdesktop-copycat tui
```

This will show:
- Which configuration files were checked
- What settings were loaded from each source
- Final effective configuration
- Any warnings or errors

## Migration from Older Versions

If you're upgrading from an older version:

1. **Backup** your current configuration
2. **Check** the changelog for breaking changes
3. **Run** with validation: `kdesktop-copycat --validate-config`
4. **Update** deprecated settings as needed

For more help with configuration, see the [Issues](https://github.com/yourusername/KDEsktop-copycat/issues) page or create a new issue.