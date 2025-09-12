# 🌌 Abyss

**Abyss** is a **Rust-based wrapper around [BepInEx](https://github.com/BepInEx/BepInEx)** for *Hollow Knight: Silksong*.  
It simplifies the installation of BepInEx, manages plugins, and provides the technical backend for [Purity](https://github.com/SilksongModding/Purity), the official mod launcher.

---

## ✨ Features (current & planned)

- 🔍 Detect *Silksong* installation folder (Steam by default, manual override available).
- ⚡ Install and configure **BepInEx** automatically.
- 📂 Manage mods (list, enable/disable, remove) inside `BepInEx/plugins/`.
- 🔌 Provide a local API (JSON-RPC/REST) for Purity integration.
- 🧩 Future: detect mod dependencies and conflicts.
- ⬆️ Future: auto-update mods from NexusMods/GameBanana.

---

## 📦 Installation

> ⚠️ Abyss is under active development and not production-ready yet.

1. Clone the repository:
   ```bash
   git clone https://github.com/SilksongModding/Abyss.git
   cd abyss
   ```

2. Build with Cargo:
   ```bash
   cargo build --release
   ```

3. Run Abyss:
   ```bash
   ./target/release/abyss install
   ```

---

## 🚀 Usage

Available commands (work in progress):

```bash
abyss install            # Install BepInEx into Silksong directory
abyss list               # List installed mods
abyss enable <mod>       # Enable a mod
abyss disable <mod>      # Disable a mod
abyss remove <mod>       # Remove a mod
```

---

## 🧪 Development

### Requirements
- [Rust](https://www.rust-lang.org/) (latest stable)
- Cargo (comes with Rust)
- Git

### Run tests
```bash
cargo test
```

### Linting & formatting
```bash
cargo fmt
cargo clippy
```

---

## 📖 Documentation

- [BepInEx Official Docs](https://docs.bepinex.dev/)
- [Silksong Modding Resources](https://www.nexusmods.com/)

We aim to provide **clear developer & user documentation** as the project grows.

---

## 🛠️ Roadmap

- [ ] MVP: Install BepInEx & manage plugins
- [ ] Expose a local API for Purity
- [ ] Dependency/conflict detection
- [ ] Mod auto-updates

---

## 🤝 Contributing

Contributions are welcome!  
Please check our [issues](https://github.com/your-org/abyss/issues) and open pull requests.  

Before submitting, make sure to:
```bash
cargo fmt
cargo clippy
cargo test
```

---

## 📜 License

MIT License © 2025 — Abyss contributors
