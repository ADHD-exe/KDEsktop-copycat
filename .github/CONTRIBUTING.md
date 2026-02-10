# Contributing to KDEsktop-copycat

We welcome contributions of all types! Whether you're fixing a bug, adding a feature, improving documentation, or reporting an issue, we appreciate your help.

## 🤝 How to Contribute

### Reporting Issues

1. **Search existing issues** first to avoid duplicates
2. **Use appropriate templates** for bug reports or feature requests
3. **Provide detailed information** including environment details
4. **Be respectful** and constructive in your communications

### Contributing Code

1. **Fork the repository** to your GitHub account
2. **Create a feature branch** for your changes
3. **Make your changes** following our coding standards
4. **Test your changes** thoroughly
5. **Submit a pull request** with a clear description

### Development Setup

```bash
# Clone your fork
git clone https://github.com/yourusername/KDEsktop-copycat.git
cd KDEsktop-copycat

# Add upstream remote
git remote add upstream https://github.com/original-owner/KDEsktop-copycat.git

# Install dependencies
cargo build
cargo test

# Run with debug logging
RUST_LOG=debug cargo run -- tui
```

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run linter: `cargo clippy -- -D warnings`
- Write tests for new functionality
- Add documentation comments for public APIs

### Areas for Contribution

#### Parser Improvements
- Better error handling for malformed configurations
- Support for additional Plasma configuration files
- Performance optimizations

#### TUI Enhancements
- Better keyboard navigation
- Additional view modes
- Improved visual design
- Search/filter functionality

#### Export/Import Features
- More restoration methods
- Better plasmoid handling
- Cross-distribution compatibility
- Incremental backup/restore

#### Documentation
- Usage examples
- Advanced guides
- API documentation
- Translations

## 📝 Pull Request Process

1. **Update your fork** with latest changes:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Create feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make changes** and commit:
   ```bash
   git commit -m "feat: add your feature description"
   ```

4. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

5. **Create Pull Request**:
   - Use clear title and description
   - Reference any related issues
   - Include screenshots for UI changes
   - Add testing instructions

6. **Respond to feedback** promptly and make requested changes

## 🏆 Recognition

Contributors are recognized in:
- `AUTHORS.md` file
- Release notes
- GitHub contributor statistics

## 📞 Getting Help

- Check existing documentation and issues
- Ask questions in GitHub Discussions
- Review existing code for patterns
- Join our community (if available)

Thank you for contributing to KDEsktop-copycat! 🎉