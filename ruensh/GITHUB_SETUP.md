# GitHub Setup Guide for RuenSH

Quick guide to publish RuenSH to GitHub.

## 1. One-Command Setup (Recommended)

```bash
bash prepare-github.sh
```

This script will:
- Initialize git repository
- Add all files
- Create initial commit
- Create version tag v0.1.0
- Display next steps

## 2. Manual Setup

If you prefer manual steps:

### Step 1: Initialize Git
```bash
git init
git config user.name "Your Name"
git config user.email "your@email.com"
```

### Step 2: Add and Commit
```bash
git add .
git commit -m "Initial commit: RuenSH v0.1.0 - TUI Library for REPL CLIs"
```

### Step 3: Create Version Tag
```bash
git tag -a v0.1.0 -m "Release v0.1.0: Initial public release"
```

### Step 4: Create GitHub Repository
1. Go to [github.com/new](https://github.com/new)
2. Create repository: `ruensh`
3. Make it **Public**
4. **Don't** initialize with README/license
5. Click "Create repository"

### Step 5: Push to GitHub
```bash
git remote add origin https://github.com/namqhorah/ruensh.git
git branch -M main
git push -u origin main
git push origin v0.1.0
```

## 3. Configure GitHub Repository

### Settings → General
- Add description: "A powerful, ergonomic Rust TUI component library for building interactive terminal interfaces and REPL applications"
- Add topics: `tui` `terminal-ui` `repl` `rust` `interactive`
- Enable "Discussions" (optional)

### Pages (Optional)
- Enable GitHub Pages
- Source: GitHub Actions
- This will auto-deploy docs from docs.rs

## 4. Verify Setup

### Check Git Status
```bash
git log --oneline -5
git tag
git remote -v
```

### Verify Files on GitHub
- All source files present
- All documentation visible
- README renders properly
- LICENSE is readable

### Check CI/CD
- Go to Actions tab
- Workflow should run on every push
- Verify builds pass

## 5. Publish to Crates.io (Optional)

When ready to publish:

### 1. Create Crates.io Account
- Visit https://crates.io
- Create account with GitHub

### 2. Generate API Token
- Account Settings → API Tokens
- Copy token

### 3. Configure Cargo
```bash
cargo login
# Paste token when prompted
```

### 4. Test Publish
```bash
cargo publish --dry-run
```

### 5. Publish
```bash
cargo publish
```

## 6. Share with Community

Once published:

- [ ] Share on Reddit r/rust
- [ ] Post on Twitter/X
- [ ] Share in language communities (for REPL use case)
- [ ] Add to Awesome Rust list
- [ ] Update your GitHub profile

## File Checklist

### Repository Root
- ✅ Cargo.toml (with metadata)
- ✅ Cargo.lock
- ✅ LICENSE (MIT)
- ✅ .gitignore
- ✅ README.md
- ✅ CHANGELOG.md
- ✅ CONTRIBUTING.md

### Documentation
- ✅ GITHUB_PREP.md
- ✅ GITHUB_SETUP.md (this file)
- ✅ COMPONENT_GUIDE.md
- ✅ QUICK_REFERENCE.md
- ✅ MODAL_DOCUMENTATION.md
- ✅ IMPLEMENTATION_SUMMARY.md

### Source Code
- ✅ src/lib.rs
- ✅ src/terminal.rs
- ✅ src/events.rs
- ✅ src/state.rs
- ✅ src/style.rs
- ✅ src/layout/mod.rs
- ✅ src/components/mod.rs
- ✅ src/components/modal.rs
- ✅ src/components/list.rs

### Examples
- ✅ examples/modal_demo.rs
- ✅ examples/repl_cli.rs

### GitHub Configuration
- ✅ .github/workflows/rust.yml
- ✅ .github/ISSUE_TEMPLATE/bug_report.yml
- ✅ .github/ISSUE_TEMPLATE/feature_request.yml
- ✅ .github/pull_request_template.md

### Scripts
- ✅ prepare-github.sh

## Troubleshooting

### Git Remote Error
```bash
# If remote already exists
git remote remove origin
git remote add origin https://github.com/namqhorah/ruensh.git
```

### Push Rejected
```bash
# If main doesn't match origin
git pull origin main --allow-unrelated-histories
git push origin main
```

### Cargo Publish Fails
```bash
# Check metadata
cargo package --allow-dirty

# Check docs
cargo doc --no-deps
```

## Success Confirmation

✅ Git repository initialized
✅ All files committed
✅ Version tags created
✅ GitHub repository created
✅ Remote configured
✅ Code pushed
✅ CI/CD running
✅ Documentation visible
✅ Ready for contributions

## Next Steps

1. **Maintain Repository**
   - Review and merge PRs
   - Respond to issues
   - Update CHANGELOG with fixes

2. **Release Versions**
   - Update version in Cargo.toml
   - Update CHANGELOG.md
   - Create git tag
   - Publish to crates.io

3. **Build Community**
   - Respond to issues helpfully
   - Guide contributors
   - Share improvements

---

**RuenSH** - Now on GitHub and ready for community contributions! 🎉
