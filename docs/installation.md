# Installation Guide

This guide covers various ways to install KDEsktop-copycat on your system.

## System Requirements

### Minimum Requirements
- **Operating System**: Linux (any distribution)
- **Desktop Environment**: KDE Plasma 6
- **Rust**: 1.70 or higher (for building from source)
- **Memory**: 64MB RAM minimum
- **Storage**: 50MB disk space

### Recommended Requirements
- **Operating System**: Arch Linux, Fedora, Ubuntu 22.04+ or similar
- **Desktop Environment**: KDE Plasma 6.1+
- **Memory**: 256MB RAM
- **Storage**: 100MB disk space

## Installation Methods

### Method 1: Install from Cargo (Recommended)

The easiest method if you have Rust installed:

```bash
# Install from crates.io
cargo install kdesktop-copycat

# The binary will be available as ~/.cargo/bin/kdesktop-copycat
# Make sure ~/.cargo/bin is in your PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Method 2: Build from Source

#### Prerequisites
Install Rust if not already installed:

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

#### Build Steps
```bash
# Clone the repository
git clone https://github.com/yourusername/KDEsktop-copycat.git
cd KDEsktop-copycat

# Build in release mode (faster and smaller binary)
cargo build --release

# Install system-wide (optional)
sudo cp target/release/kdesktop-copycat /usr/local/bin/

# Or install to user directory
mkdir -p ~/.local/bin
cp target/release/kdesktop-copycat ~/.local/bin/
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

### Method 3: Download Pre-built Binary

Download the appropriate binary for your system:

#### Linux (x86_64)
```bash
# Download latest release
wget https://github.com/yourusername/KDEsktop-copycat/releases/latest/download/kdesktop-copycat-linux-x86_64

# Make executable
chmod +x kdesktop-copycat-linux-x86_64

# Move to system PATH
sudo mv kdesktop-copycat-linux-x86_64 /usr/local/bin/kdesktop-copycat
```

#### Linux (ARM64)
```bash
# Download latest release
wget https://github.com/yourusername/KDEsktop-copycat/releases/latest/download/kdesktop-copycat-linux-arm64

# Make executable
chmod +x kdesktop-copycat-linux-arm64

# Move to system PATH
sudo mv kdesktop-copycat-linux-arm64 /usr/local/bin/kdesktop-copycat
```

### Method 4: Install via Package Manager

#### Arch Linux (AUR)
```bash
# Using yay
yay -S kdesktop-copycat

# Using paru
paru -S kdesktop-copycat

# Manually from AUR
git clone https://aur.archlinux.org/kdesktop-copycat.git
cd kdesktop-copycat
makepkg -si
```

#### Nix/NixOS
```bash
# Install from nixpkgs
nix-env -iA nixpkgs.kdesktop-copycat

# Or using flakes
nix profile install github:yourusername/KDEsktop-copycat
```

#### Homebrew (Linux)
```bash
# Add repository (if not already added)
brew tap yourusername/tap

# Install
brew install kdesktop-copycat
```

### Method 5: Docker

#### Using Docker Hub
```bash
# Pull the image
docker pull yourusername/kdesktop-copycat:latest

# Run with host Plasma configuration
docker run --rm -it \
  -v ~/.config:/root/.config:ro \
  yourusername/kdesktop-copycat:latest tui
```

#### Build from Source
```bash
git clone https://github.com/yourusername/KDEsktop-copycat.git
cd KDEsktop-copycat

# Build Docker image
docker build -t kdesktop-copycat .

# Run
docker run --rm -it \
  -v ~/.config:/root/.config:ro \
  kdesktop-copycat tui
```

### Method 6: Install from Distribution Package

#### Ubuntu/Debian
```bash
# Add repository (if available)
wget -qO- https://apt.example.com/kdesktop-copycat.gpg | sudo apt-key add -
echo "deb https://apt.example.com/ stable main" | sudo tee /etc/apt/sources.list.d/kdesktop-copycat.list

# Install
sudo apt update
sudo apt install kdesktop-copycat
```

#### Fedora
```bash
# Add COPR repository (if available)
sudo dnf copr enable yourusername/kdesktop-copycat

# Install
sudo dnf install kdesktop-copycat
```

## Post-Installation Setup

### Verify Installation
```bash
# Check if the binary is available
kdesktop-copycat --version

# Test basic functionality
kdesktop-copycat --help

# Test with your Plasma configuration
kdesktop-copycat scan
```

### Create Configuration Directory
```bash
# Create config directory
mkdir -p ~/.config/kdesktop-copycat

# Create default configuration
cat > ~/.config/kdesktop-copycat/config.toml << 'EOF'
[kdesktop-copycat]
plasma_config = "~/.config/plasma-org.kde.plasma.desktop-appletsrc"
kwin_config = "~/.config/kwinrc"
kwin_rules = "~/.config/kwinrulesrc"

[export]
include_snapshot = false
include_plasmoids = false
output_dir = "~/.local/share/kdesktop-copycat/bundles"
EOF
```

### Set Up Logging
```bash
# Create log directory
mkdir -p ~/.local/share/kdesktop-copycat/logs

# Configure logging
cat >> ~/.config/kdesktop-copycat/config.toml << 'EOF'

[logging]
level = "info"
file = "~/.local/share/kdesktop-copycat/logs/kdesktop-copycat.log"
EOF
```

## Troubleshooting Installation

### Common Issues

#### "command not found: kdesktop-copycat"
```bash
# Check if binary exists
which kdesktop-copycat

# If not found, check your PATH
echo $PATH

# Add cargo bin to PATH (if installed via cargo)
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### "Permission denied"
```bash
# Make binary executable
chmod +x /path/to/kdesktop-copycat

# Or install to user directory instead of system
mkdir -p ~/.local/bin
cp kdesktop-copycat ~/.local/bin/
```

#### "No such file or directory: plasma-appletsrc"
```bash
# Check if Plasma is installed and running
echo $XDG_CURRENT_DESKTOP

# Find your Plasma configuration
find ~/.config -name "*plasma*appletsrc*"

# Specify custom path
kdesktop-copycat --file /path/to/plasma-appletsrc scan
```

#### Build Errors
```bash
# Update Rust
rustup update stable

# Clean build cache
cargo clean

# Update dependencies
cargo update

# Try with specific toolchain
rustup default stable
cargo build --release
```

### Getting Help

If you encounter issues during installation:

1. **Check the [Issues](https://github.com/yourusername/KDEsktop-copycat/issues)** page
2. **Create a new issue** with your installation details
3. **Join our community** (Discord/Matrix if available)

When reporting installation issues, please include:

- Your operating system and version
- Installation method used
- Error messages (full output)
- Rust version (if building from source)
- Plasma version

## Uninstallation

### Remove from Cargo
```bash
cargo uninstall kdesktop-copycat
```

### Remove System Binary
```bash
sudo rm /usr/local/bin/kdesktop-copycat
```

### Remove User Files
```bash
# Remove configuration
rm -rf ~/.config/kdesktop-copycat

# Remove data files
rm -rf ~/.local/share/kdesktop-copycat

# Remove from PATH (edit ~/.bashrc or ~/.zshrc)
```

### Remove Package (if installed via package manager)
```bash
# Arch Linux
sudo pacman -R kdesktop-copycat

# Ubuntu/Debian
sudo apt remove kdesktop-copycat

# Fedora
sudo dnf remove kdesktop-copycat
```

## Next Steps

After successful installation:

1. **Read the [User Guide](https://yourusername.github.io/KDEsktop-copycat/)**
2. **Try the basic commands** in the [Examples](https://yourusername.github.io/KDEsktop-copycat/examples/)
3. **Check out the [Configuration Guide](configuration.md)**
4. **Join the community** for updates and support

Happy Plasma configuration management! 🎉