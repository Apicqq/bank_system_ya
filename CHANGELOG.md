## [0.1.0] - 2026-05-29

### 🚀 Features

- Initialize Rust library crate with add function and .gitignore
- Scaffold core project module structure and entry points (#2)
- *(models)* Add transaction domain models and export them (#4)
- Add error mapping (#5)
- *(format)* Add BankFormat trait and scaffold format implementations (#6)
- Add csv parser implementation (#7)
- Implement txt parser (#8)
- Implement binary parser (#9)
- *(cli)* Add format converter CLI and integration tests (#11)
- Implement comparer (#12)

### 📚 Documentation

- Add project and CLI READMEs and define named binaries

### 🎨 Styling

- Reorder format trait impls and clean up model docs (#10)

### ⚙️ Miscellaneous Tasks

- Add GitHub Actions cargo checks workflow and lockfile (#1)
- *(pre-commit)* Add Rust pre-commit hooks for fmt, check, and clippy with pre-push tests (#3)
