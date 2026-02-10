# Contributing to KDEsktop-copycat

Thank you for your interest in contributing to KDEsktop-copycat! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or higher
- KDE Plasma 6 environment for testing
- Git
- Basic knowledge of Rust and KDE Plasma

### Development Setup

1. **Fork** the repository on GitHub
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/yourusername/KDEsktop-copycat.git
   cd KDEsktop-copycat
   ```

3. **Add** the original repository as upstream:
   ```bash
   git remote add upstream https://github.com/original-owner/KDEsktop-copycat.git
   ```

4. **Install** dependencies and build:
   ```bash
   cargo build
   cargo test
   ```

## 📋 Development Guidelines

### Code Style

We follow the standard Rust formatting guidelines:

```bash
# Format code
cargo fmt

# Run clippy for linting
cargo clippy -- -D warnings
```

### Commit Messages

We follow the conventional commit format:

- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `style:` for code style changes (formatting, etc.)
- `refactor:` for code refactoring
- `test:` for adding or modifying tests
- `chore:` for maintenance tasks

Examples:
```
feat: add support for custom configuration paths
fix: handle malformed configuration files gracefully
docs: update README with installation instructions
```

### Testing

Before submitting a pull request:

1. **Run** all tests:
   ```bash
   cargo test
   ```

2. **Test** manually with your KDE Plasma setup:
   ```bash
   cargo run -- tui
   cargo run -- scan
   cargo run -- export --out ./test-bundle
   ```

3. **Verify** the generated bundles can be restored

## 🔧 Development Areas

### Parser Improvements

The `parser.rs` module handles Plasma configuration parsing. Areas for improvement:
- Better error handling for malformed configurations
- Support for additional Plasma configuration files
- Performance optimizations for large configurations

### TUI Enhancements

The `tui.rs` module implements the terminal interface. Potential improvements:
- Better keyboard navigation
- Additional view modes
- Improved visual design
- Search/filter functionality

### Export/Import Features

The `export.rs` and `scripts.rs` modules handle bundle creation and restoration. Possible enhancements:
- Support for more restoration methods
- Better plasmoid handling
- Cross-distribution compatibility
- Incremental backup/restore

### KWin Integration

The `kwin.rs` module handles KWin configuration scanning. Areas to explore:
- Support for more KWin features
- Better effect categorization
- Integration with KWin scripts

## 🐛 Bug Reports

When reporting bugs, please include:

1. **Environment Information**:
   - KDE Plasma version
   - Rust version (`rustc --version`)
   - Operating system and distribution

2. **Steps to Reproduce**:
   - Clear, step-by-step instructions
   - Example configuration files (if applicable)
   - Expected vs. actual behavior

3. **Error Messages**:
   - Full error output
   - Debug logs (run with `RUST_LOG=debug`)

4. **Additional Context**:
   - Any unusual system configuration
   - Recent changes to your Plasma setup

## 💡 Feature Requests

When suggesting new features:

1. **Use Cases**: Describe the problem you're trying to solve
2. **Proposed Solution**: How you envision the feature working
3. **Alternatives**: Any alternative solutions you've considered
4. **Implementation**: Any thoughts on implementation (optional)

## 📝 Documentation

We welcome documentation improvements:

- **README.md**: General project information
- **Code Comments**: Inline documentation for complex logic
- **Examples**: Usage examples and tutorials
- **Wiki**: Detailed guides and advanced topics

## 🔄 Pull Request Process

1. **Create** a new branch for your feature:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make** your changes and commit them:
   ```bash
   git commit -m "feat: add your feature description"
   ```

3. **Push** to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

4. **Create** a Pull Request on GitHub with:
   - Clear title and description
   - Reference any related issues
   - Screenshots for UI changes (if applicable)
   - Testing instructions

5. **Respond** to code review feedback promptly

## 🏷️ Release Process

Releases follow semantic versioning (semver):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

Release checklist:
1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create Git tag
4. Build release binaries
5. Create GitHub release

## 🤝 Community Guidelines

### Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please:

- Be respectful and considerate
- Use inclusive language
- Focus on constructive feedback
- Help others learn and grow

### Getting Help

If you need help:

1. Check existing documentation and issues
2. Ask questions in GitHub Discussions
3. Join our community chat (if available)
4. Start a new issue with the "question" label

## 📚 Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [KDE Plasma Development](https://develop.kde.org/docs/plasma/)
- [Ratatui Documentation](https://docs.rs/ratatui/)
- [Clap Documentation](https://docs.rs/clap/)

## 🙏 Recognition

Contributors are recognized in:
- `AUTHORS.md` file
- Release notes
- GitHub contributor statistics

Thank you for contributing to KDEsktop-copycat! 🎉