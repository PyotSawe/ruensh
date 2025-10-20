# Git & GitHub Setup for RuenSH

Complete instructions for setting up the GitHub repository and publishing RuenSH.

## Prerequisites

- Git installed (`git --version`)
- GitHub account created (https://github.com)
- Rust toolchain (already installed)
- crates.io account (https://crates.io) - for publishing

## Step 1: Verify Local Repository

### Check current directory
```bash
cd /home/yathur/2025SRU/TaunSys/TUILab/ruensh
pwd  # Should show ruensh directory
```

### Verify git initialization
```bash
git status
```

If not initialized:
```bash
git init
git config user.name "Your Name"
git config user.email "your.email@example.com"
```

## Step 2: Add All Files to Git

```bash
# Add all files
git add .

# Verify staged files
git status

# Should show:
# - src/ files
# - examples/ files
# - Cargo.toml
# - README.md
# - All documentation
# - LICENSE
# - .gitignore
```

## Step 3: Create Initial Commit

```bash
git commit -m "Initial commit: RuenSH v0.1.0

- Modal component with smooth animations
- Keyboard & mouse event handling
- Type-safe component system
- Comprehensive documentation
- Ready for production use"
```

### Verify commit
```bash
git log
git log --oneline -1
```

## Step 4: Create GitHub Repository

### On GitHub.com:
1. Go to https://github.com/new
2. Enter:
   - **Owner**: Your GitHub account
   - **Repository name**: `ruensh`
   - **Description**: "A powerful TUI component library for Rust with REPL support"
   - **Public** or **Private** (recommend Public for open source)
3. **Do NOT initialize** with README, .gitignore, or license (we already have them)
4. Click "Create repository"

## Step 5: Connect Local Repository to GitHub

```bash
# Add remote (replace USERNAME with your GitHub username)
git remote add origin https://github.com/USERNAME/ruensh.git

# Verify remote
git remote -v
# Should show:
# origin  https://github.com/USERNAME/ruensh.git (fetch)
# origin  https://github.com/USERNAME/ruensh.git (push)

# Set default branch to main
git branch -M main

# Push to GitHub
git push -u origin main

# Verify push
git log --oneline origin/main
```

If pushing fails with authentication:

**Option A: Personal Access Token (Recommended)**
```bash
# Go to https://github.com/settings/tokens
# Create new token with 'repo' scope
# When prompted for password during git push, use the token instead
```

**Option B: SSH Key**
```bash
# Setup SSH key (if not already done)
ssh-keygen -t ed25519 -C "your.email@example.com"
# Add public key to GitHub: https://github.com/settings/keys

# Change remote to SSH
git remote set-url origin git@github.com:USERNAME/ruensh.git
git push -u origin main
```

## Step 6: Create Release Tag

```bash
# Create annotated tag for v0.1.0
git tag -a v0.1.0 -m "RuenSH v0.1.0 - Initial Release"

# Push tag to GitHub
git push origin v0.1.0

# Verify tag
git tag -l
git show v0.1.0
```

## Step 7: Create GitHub Release

### On GitHub.com:
1. Go to: `https://github.com/USERNAME/ruensh/releases`
2. Click "Draft a new release"
3. Enter:
   - **Tag**: v0.1.0 (should auto-complete)
   - **Title**: "RuenSH v0.1.0 - Initial Release"
   - **Description**: Copy and paste from CHANGELOG.md

```markdown
## Features

### Modal Component
- Smooth popup animations (appearing/disappearing states)
- Keyboard navigation (Tab, Shift+Tab, Arrows, Enter, Esc, Y/N)
- Mouse support (hover detection, click activation)
- Button focus highlighting with visual feedback
- Customizable styling and colors

### Event System
- Comprehensive keyboard event handling
- Full mouse event support (move, click)
- Terminal resize events
- Tick events for animations
- Type-safe message routing

### Architecture
- Trait-based component system with generic messages
- Decoupled event handling and rendering
- Builder pattern for component configuration
- Zero unsafe code in core components

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ruensh = "0.1"
```

## Quick Start

See README.md for complete examples.

## Documentation

- [README.md](README.md) - Project overview
- [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - API reference
- [COMPONENT_GUIDE.md](COMPONENT_GUIDE.md) - Building components
- [REPL_INTEGRATION_GUIDE.md](REPL_INTEGRATION_GUIDE.md) - REPL integration

## Performance

- Event handling: O(1) complexity
- Rendering: < 1ms per frame
- 60 FPS smooth animations
- ~500 bytes per modal component

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)

## License

MIT License - see [LICENSE](LICENSE)
```

4. Click "Publish release"

## Step 8: Setup Continuous Integration (Optional but Recommended)

The CI/CD workflow is already configured in `.github/workflows/rust.yml`.

### Verify it's working:
1. Go to: `https://github.com/USERNAME/ruensh/actions`
2. Should see a workflow run from your push
3. Click on it to see build details

### Monitor CI status:
- Badge will appear in README.md
- All PRs will run CI checks
- Releases will be built automatically

## Step 9: Publish to crates.io

### Prerequisites:
- Create account at https://crates.io if not done
- Generate token at https://crates.io/me

### Configure local environment:
```bash
# Store crates.io token (you'll be prompted if missing)
cargo login
# Enter your token when prompted
```

### Dry run first (always recommended):
```bash
cargo publish --dry-run

# Should show:
# Uploading ruensh v0.1.0 to registry `crates-io`
# Verifying package...
# Done (without actually publishing)
```

### Publish for real:
```bash
cargo publish

# Should show:
# Uploading ruensh v0.1.0 to registry `crates-io`
# Verifying package...
# Uploading crate documentation
# Waiting for package verification...
# Published: ruensh v0.1.0 at registry `crates-io`
```

### Verify on crates.io:
- Visit: https://crates.io/crates/ruensh
- Should show v0.1.0 published
- Documentation at: https://docs.rs/ruensh

## Step 10: Configure Repository Settings (Optional)

### On GitHub.com:
1. Go to: `Settings > Options`
2. Configure:
   - **Description**: "A powerful TUI component library for Rust"
   - **Website**: https://crates.io/crates/ruensh
   - **Topics**: tui, terminal, ui, repl, rust
3. Click "Save"

### Enable GitHub Discussions (Optional):
1. Go to: `Settings > Features`
2. Check "Discussions"
3. Users can now discuss in Discussions tab

### Enable GitHub Pages (Optional):
1. Go to: `Settings > Pages`
2. Set source to: `main` branch, `/root` folder
3. Wait for deployment
4. Documentation will be at: `https://USERNAME.github.io/ruensh`

## Verify Everything Works

```bash
# Test local build
cargo build ✅
cargo test ✅
cargo build --all-examples ✅

# Test package
cargo publish --dry-run ✅

# Verify git setup
git remote -v ✅
git log --oneline -1 ✅
git tag ✅

# Check GitHub
# - https://github.com/USERNAME/ruensh (repo)
# - https://github.com/USERNAME/ruensh/releases (v0.1.0 release)
# - https://crates.io/crates/ruensh (published package)
# - https://docs.rs/ruensh (documentation)
```

## File Structure After Publication

```
.git/                                    # Git repository
.github/
  workflows/
    rust.yml                            # CI/CD workflow
  ISSUE_TEMPLATE/
    bug_report.yml
    feature_request.yml
  pull_request_template.md

src/
  lib.rs
  terminal.rs
  events.rs
  state.rs
  style.rs
  layout/
    mod.rs
  components/
    mod.rs
    modal.rs
    list.rs

examples/
  modal_demo.rs
  repl_cli.rs

Cargo.toml                              # Package manifest
Cargo.lock                              # Dependency lock (generated)
LICENSE                                 # MIT License
.gitignore                              # Git ignore rules
README.md                               # Project overview
QUICK_REFERENCE.md                      # API reference
COMPONENT_GUIDE.md                      # Component guide
REPL_INTEGRATION_GUIDE.md               # REPL integration
CONTRIBUTING.md                         # Contributing guidelines
CHANGELOG.md                            # Version history
GITHUB_PUBLICATION_CHECKLIST.md         # Pre-pub checklist
GITHUB_PUBLICATION_SUMMARY.md           # Publication summary
GIT_GITHUB_SETUP.md                     # This file
```

## Troubleshooting

### Git push fails with authentication error
```bash
# Try SSH instead:
git remote set-url origin git@github.com:USERNAME/ruensh.git
git push -u origin main

# Or use personal access token:
# https://github.com/settings/tokens
```

### Can't find repository URL
```bash
# Get correct URL from:
# https://github.com/USERNAME/ruensh
# Click "Code" button for correct URL
```

### Cargo publish fails
```bash
# Check token:
cargo login
# Re-enter token if needed

# Verify package:
cargo publish --dry-run

# Try again:
cargo publish
```

### CI/CD workflow not running
```bash
# Check:
# 1. Workflow file exists: .github/workflows/rust.yml
# 2. Push was successful
# 3. Branch is 'main'
# 4. Check Actions tab for errors
```

## Next Steps After Publication

1. **Promote on social media**
   - Twitter/X
   - Reddit r/rust
   - Mastodon Rust community

2. **Add to Rust ecosystem lists**
   - Awesome Rust: https://github.com/rust-unofficial/awesome-rust
   - This Week in Rust: https://this-week-in-rust.org

3. **Monitor feedback**
   - Check GitHub Issues
   - Respond to questions
   - Gather feature requests

4. **Plan v0.2.0**
   - Input field component
   - More examples
   - Community feedback integration

5. **Consider ecosystem integration**
   - Language REPL implementations
   - CLI tool integrations
   - More advanced examples

## Additional Resources

- [GitHub Help](https://docs.github.com)
- [Git Documentation](https://git-scm.com/doc)
- [Cargo Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io Guidelines](https://crates.io/guidelines)
- [Semantic Versioning](https://semver.org/)

---

## Summary Checklist

- [ ] Repository created on GitHub
- [ ] Local git configured and committed
- [ ] Remote origin added and verified
- [ ] Code pushed to main branch
- [ ] v0.1.0 tag created and pushed
- [ ] GitHub Release created
- [ ] Published to crates.io
- [ ] Documentation live on docs.rs
- [ ] CI/CD workflow running
- [ ] Repository settings configured

**Status**: ✅ Ready for Publication

Once all steps complete, RuenSH will be live and ready for use!
