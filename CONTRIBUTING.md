# Contributing to Abyss

First off, thank you for considering contributing to **Abyss**! 🚀  
Abyss is a Rust-based wrapper around [BepInEx](https://github.com/BepInEx/BepInEx) for *Hollow Knight: Silksong*.  
Our goal is to build a clean, professional, and maintainable foundation for Silksong modding.

---

## 📜 Code of Conduct

Please note that this project follows a [Code of Conduct](CODE_OF_CONDUCT.md).  
We expect all contributors to respect it to ensure a welcoming and inclusive environment.

---

## 🤝 How to Contribute

There are many ways to help:

- 🐛 Reporting bugs
- 💡 Suggesting new features
- 📝 Improving documentation
- 💻 Submitting code changes (fixes, features, tests)

Check our [issues](https://github.com/SilksongModding/Abyss/issues) to see what needs work.

---

## 🛠️ Development Setup

### Prerequisites
- [Rust](https://www.rust-lang.org/) (latest stable)
- Cargo (comes with Rust)
- Git

### Build the project
```bash
git clone https://github.com/SilksongModding/Abyss.git
cd abyss
cargo build
```

### Run tests
```bash
cargo test
```

---

## 🎨 Code Style

- Format code with:
  ```bash
  cargo fmt
  ```

- Run linter with:
  ```bash
  cargo clippy
  ```

- No warnings should remain in merged code.

- Write idiomatic Rust and keep code clear and simple.

---

## 🌿 Git Workflow

- **Branches**:
  - `main` → stable branch
  - `dev` → development branch
  - `feat/xxx` → feature branches

- **Commits** should be clear, in English, and follow a conventional style. Examples:

  ```
  feat(installer): add automatic BepInEx installation
  fix(mods): handle missing plugins directory
  docs(readme): clarify usage of abyss list
  ```

---

## 🔀 Pull Requests

- Open PRs against the `dev` branch.  
- Ensure that before submitting:
  - Code is formatted (`cargo fmt`)
  - No warnings remain (`cargo clippy`)
  - All tests pass (`cargo test`)

- Clearly describe the purpose of the PR.  
- Link to related issues when possible.

---

## 🧪 Testing

- Add unit tests for all new features and fixes.  
- Use integration tests for larger workflows (e.g., simulating BepInEx installation in a temporary directory).  
- Make sure tests pass before opening a PR.

---

## 📖 Documentation

- Document public functions with `///` Rustdoc comments.  
- Update the `README.md` if your change affects the user workflow.  
- Add examples when possible.

<!-- ---

## 🚀 Release Process

- We follow **Semantic Versioning (semver)**: `MAJOR.MINOR.PATCH`.  
- Each release must include an updated `CHANGELOG.md`.  
- GitHub Actions ensure cross-platform builds and tests. -->

---

Thank you for helping make Abyss better! 🌌
