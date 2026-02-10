# Build from Source

This guide covers building KDEsktop-copycat from source code.

## Prerequisites

### Required Software

- **Rust** 1.70 or higher
- **Git** for cloning the repository
- **C compiler** (gcc or clang)
- **pkg-config** (for system dependencies)

### System Dependencies

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install build-essential pkg-config libx11-dev libxtst-dev
```

#### Fedora/RHEL
```bash
sudo dnf install gcc pkg-config libX11-devel libXtst-devel
```

#### Arch Linux
```bash
sudo pacman -S base-devel pkg-config libx11 libxtst
```

#### openSUSE
```bash
sudo zypper install gcc pkg-config libX11-devel libXtst-devel
```

### Installing Rust

#### Method 1: Using rustup (Recommended)
```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts, then source the environment
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

#### Method 2: System Package Manager
```bash
# Ubuntu/Debian
sudo apt install rustc cargo

# Fedora
sudo dnf install rust cargo

# Arch Linux
sudo pacman -S rust
```

## Building KDEsktop-copycat

### Step 1: Clone the Repository
```bash
# Clone the repository
git clone https://github.com/yourusername/KDEsktop-copycat.git
cd KDEsktop-copycat

# Checkout the desired version
git checkout main  # or a specific tag like v0.1.0
```

### Step 2: Build the Project

#### Debug Build
```bash
# Build in debug mode (faster compilation, larger binary)
cargo build

# The binary will be at target/debug/kdesktop-copycat
```

#### Release Build (Recommended)
```bash
# Build in release mode (optimized, smaller binary)
cargo build --release

# The binary will be at target/release/kdesktop-copycat
```

### Step 3: Test the Build
```bash
# Run tests
cargo test

# Test the binary
./target/release/kdesktop-copycat --version
./target/release/kdesktop-copycat --help
```

## Build Options

### Custom Features
```bash
# Build with default features
cargo build --release

# Build without certain features (if available)
cargo build --release --no-default-features

# Build with specific features
cargo build --release --features "feature1,feature2"
```

### Target Specific Builds
```bash
# Build for different targets
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu

# Install target support
rustup target add x86_64-unknown-linux-musl
```

### Optimization Options
```bash
# Build with custom optimizations
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Build with LTO (Link Time Optimization)
RUSTFLAGS="-C link-arg=-fuse-ld=lld" cargo build --release
```

## Development Build

### Setting Up Development Environment
```bash
# Install development dependencies
cargo install cargo-watch cargo-expand

# Set up pre-commit hooks (if using)
pre-commit install

# Build with development features
cargo build
```

### Development Workflow
```bash
# Watch for changes and rebuild
cargo watch -x run -- tui

# Run tests automatically
cargo watch -x test

# Check formatting
cargo fmt --all

# Run linter
cargo clippy -- -D warnings
```

### Documentation Build
```bash
# Build documentation
cargo doc --no-deps

# Open documentation in browser
cargo doc --open
```

## Cross-Compilation

### Cross-Compiling for Different Architectures

#### From Linux to Windows
```bash
# Install Windows target
rustup target add x86_64-pc-windows-gnu

# Install cross-compilation tools
sudo apt install mingw-w64

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu
```

#### From Linux to macOS
```bash
# Install macOS target
rustup target add x86_64-apple-darwin aarch64-apple-darwin

# Build for macOS (requires osxcross)
cargo build --release --target x86_64-apple-darwin
```

#### Using Docker for Cross-Compilation
```bash
# Use cross-rs for easier cross-compilation
cargo install cross

# Build for different targets
cross build --release --target x86_64-unknown-linux-musl
cross build --release --target aarch64-unknown-linux-gnu
```

## Packaging

### Creating Debian Package
```bash
# Install cargo-deb
cargo install cargo-deb

# Build Debian package
cargo deb

# The .deb file will be in target/debian/
```

### Creating RPM Package
```bash
# Install cargo-generate-rpm
cargo install cargo-generate-rpm

# Build RPM package
cargo generate-rpm

# The .rpm file will be in target/generate-rpm/
```

### Creating Arch Linux Package
```bash
# Create PKGBUILD
cat > PKGBUILD << 'EOF'
pkgname=kdesktop-copycat
pkgver=0.1.0
pkgrel=1
pkgdesc="A tool for parsing and exporting KDE Plasma desktop configurations"
arch=('x86_64')
url="https://github.com/yourusername/KDEsktop-copycat"
license=('MIT')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/yourusername/KDEsktop-copycat/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "$pkgname-$pkgver"
  cargo build --release
}

package() {
  cd "$pkgname-$pkgver"
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}
EOF

# Build package
makepkg -si
```

## Troubleshooting Build Issues

### Common Build Errors

#### "Could not find X11 libraries"
```bash
# Install X11 development libraries
# Ubuntu/Debian
sudo apt install libx11-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev

# Fedora
sudo dnf install libX11-devel libXrandr-devel libXinerama-devel libXcursor-devel libXi-devel
```

#### "pkg-config not found"
```bash
# Install pkg-config
sudo apt install pkg-config  # Ubuntu/Debian
sudo dnf install pkg-config  # Fedora
sudo pacman -S pkg-config    # Arch Linux
```

#### "Rust too old"
```bash
# Update Rust
rustup update stable
rustup default stable
```

#### "Out of memory" during build
```bash
# Limit parallel jobs
cargo build --release -j 1

# Or set environment variable
export CARGO_BUILD_JOBS=1
cargo build --release
```

### Clean Build
```bash
# Clean build artifacts
cargo clean

# Remove target directory completely
rm -rf target/

# Rebuild
cargo build --release
```

### Verifying Build
```bash
# Check binary information
file target/release/kdesktop-copycat
ldd target/release/kdesktop-copycat  # Linux

# Test basic functionality
./target/release/kdesktop-copycat --version
./target/release/kdesktop-copycat --help
```

## Performance Optimization

### Build Time Optimization
```bash
# Use mold linker for faster builds (Linux)
sudo apt install mold
export RUSTFLAGS="-C link-arg=-fuse-ld=mold"
cargo build --release

# Use sccache for caching
cargo install sccache
export RUSTC_WRAPPER=sccache
cargo build --release
```

### Runtime Optimization
```bash
# Build with optimizations for your CPU
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Build with LTO
RUSTFLAGS="-C link-arg=-fuse-ld=lld" cargo build --release

# Strip debug symbols
strip target/release/kdesktop-copycat
```

## Continuous Integration

### GitHub Actions Example
```yaml
name: Build
on: [push, pull_request]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    - name: Build
      run: cargo build --release
    - name: Test
      run: cargo test
```

### Local CI Testing
```bash
# Run full CI locally
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo test
cargo build --release
```

## Contributing to the Build System

If you want to contribute to the build system:

1. **Test on multiple platforms** (Linux distributions, architectures)
2. **Update documentation** for new build options
3. **Add CI tests** for new build configurations
4. **Consider backwards compatibility** for build scripts

## Next Steps

After building from source:

1. **Install the binary** to your system PATH
2. **Test with your Plasma configuration**
3. **Read the [User Guide](user-guide.md)**
4. **Consider contributing** to the project

Happy building! 🛠️