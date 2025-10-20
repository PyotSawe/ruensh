# Changelog

All notable changes to RuenSH will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned

- Input field component with validation
- List component with scrolling and selection
- Table component with sorting and filtering
- Progress bar component
- Tabs component
- Text area with word wrapping
- Form validation system
- Async task integration
- Additional theme presets
- Accessibility improvements
- Example implementations for popular languages

## [0.1.0] - 2025-10-18

### Added

- **Initial Release** - RuenSH TUI Library v0.1.0

#### Core Features
- Modal component with smooth popup animations
  - State machine (Hidden → Appearing → Visible → Disappearing)
  - Customizable button labels
  - Content and title support
  - Theme support

#### Event System
- Comprehensive keyboard event handling
  - Tab and Shift+Tab navigation
  - Arrow key support
  - Enter/Escape key handling
  - Quick access keys (Y/N)

#### Mouse Support
- Full mouse event handling
  - Hover detection and visual feedback
  - Click support for button activation
  - Position tracking
  - Button state management

#### Visual Features
- Button focus highlighting with color inversion
- Smooth animations (10-frame transitions at 60 FPS)
- Customizable color themes (light/dark presets)
- Border styling support
- Animation state tracking

#### Architecture
- Trait-based component system with generic message types
- Type-safe event routing with Message enums
- Decoupled event handling and rendering
- Builder pattern for component configuration

#### Documentation
- Comprehensive README with quick start guide
- Modal component technical documentation
- Quick reference guide for API and keyboard shortcuts
- Component building guide with examples
- Contributing guidelines
- GitHub issue and PR templates

#### Examples
- Interactive modal demo showcasing all features
- REPL CLI example for language integration
- Complete working implementations

#### Project Setup
- GitHub CI/CD pipeline (Rust.yml workflow)
- MIT License
- .gitignore configuration
- Cargo.toml metadata for crates.io
- Semantic versioning

#### Performance
- O(1) event handling complexity
- < 1ms rendering time per frame
- ~500 bytes memory per modal component
- No heap allocations in hot paths
- 60 FPS animation target met

### Documentation

- Full API documentation with examples
- Architecture overview and design patterns
- Best practices guide for component development
- REPL integration guide for language CLIs
- Performance metrics and benchmarks

---

## Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (0): Breaking changes
- **MINOR** (.1): New features (backward compatible)
- **PATCH** (.0): Bug fixes and improvements

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.
