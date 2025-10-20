# Contributing to RuenSH

We're excited that you want to contribute to RuenSH! This document will guide you through the contribution process.

## Code of Conduct

Please be respectful, inclusive, and professional in all interactions.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/ruensh.git
   cd ruensh
   ```
3. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

1. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test
   ```

4. Run the example:
   ```bash
   cargo run --example modal_demo
   ```

## Coding Guidelines

- **Code Style**: Follow Rust idioms and naming conventions
- **Comments**: Document non-obvious logic with clear comments
- **Testing**: Add tests for new functionality
- **Documentation**: Update documentation for public APIs
- **Performance**: Consider performance implications of changes

## Commit Guidelines

- Use descriptive commit messages
- Reference issues when applicable (e.g., `Fixes #123`)
- Keep commits focused and atomic
- Format: `type(scope): description`

Examples:
- `feat(modal): add support for custom styling`
- `fix(events): correct mouse event coordinate handling`
- `docs(readme): update installation instructions`
- `test(components): add modal animation tests`

Commit types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

## Pull Request Process

1. **Update your branch**: Ensure it's up to date with `main`
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Test your changes**:
   ```bash
   cargo test
   cargo build --release
   cargo run --example modal_demo
   ```

3. **Create a Pull Request** with:
   - Clear title describing the change
   - Description of what was changed and why
   - Link to related issues
   - Screenshots/GIFs if UI changes
   - Checklist of testing completed

## Pull Request Checklist

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] New code is documented
- [ ] Commits are descriptive
- [ ] No unnecessary dependencies added
- [ ] Breaking changes clearly noted

## Reporting Issues

When reporting issues, please include:
- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Rust version (`rustc --version`)
- Platform (Linux, macOS, Windows)
- Code example if applicable

## Questions?

Feel free to:
- Open a GitHub discussion
- Create an issue for clarification
- Reach out to maintainers

Thank you for contributing! 🎉
