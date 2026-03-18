# Social Media Automation Foundation Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the foundational infrastructure for social media automation system with browser engine and stealth capabilities

**Architecture:** Rust-based system using Chromey for browser automation, SQLite for data persistence, YAML for configuration. Modular design with clear separation between browser engine, data layer, and configuration management.

**Tech Stack:** Rust, Chromey (CDP), Tokio, SQLite, serde_yaml, Axum

---

## Scope

This plan covers Phase 1 (Foundation) and Phase 2 (Browser Engine) from the design document. It establishes:
- Project structure and dependencies
- Database schema and operations
- Configuration management
- Browser automation engine with stealth patch
- Basic CLI interface

Later phases (platform adapters, task scheduler, web UI) will build on this foundation.

---

## File Structure

```
social-auto/
├── Cargo.toml                          # Project manifest
├── config.yaml                         # Main configuration
├── src/
│   ├── main.rs                        # Entry point
│   ├── lib.rs                         # Library root
│   ├── cli/
│   │   ├── mod.rs                     # CLI module
│   │   ├── commands.rs                # Command handlers
│   │   └── args.rs                    # Argument parsing
│   ├── config/
│   │   ├── mod.rs                     # Config module
│   │   └── settings.rs                # Settings struct
│   ├── db/
│   │   ├── mod.rs                     # Database module
│   │   ├── schema.rs                  # Schema definitions
│   │   └── operations.rs              # CRUD operations
│   ├── models/
│   │   ├── mod.rs                     # Models module
│   │   ├── account.rs                 # Account model
│   │   ├── task.rs                    # Task model
│   │   └── content.rs                 # Content model
│   ├── browser/
│   │   ├── mod.rs                     # Browser module
│   │   ├── engine.rs                  # Chromey wrapper
│   │   ├── stealth.rs                 # Stealth patch
│   │   └── behavior.rs                # Human behavior simulation
│   └── utils/
│       ├── mod.rs                     # Utils module
│       ├── crypto.rs                  # Encryption utilities
│       └── logger.rs                  # Logging setup
├── tests/
│   ├── integration_test.rs            # Integration tests
│   └── fixtures/
│       └── test_config.yaml           # Test configuration
└── data/                              # Runtime data directory
    └── .gitkeep
```

---

### Task 1: Project Initialization

**Files:**
- Create: `Cargo.toml`
- Create: `.gitignore`
- Create: `config.yaml`
- Create: `data/.gitkeep`

- [ ] **Step 1: Create Cargo.toml with dependencies**

```toml
[package]
name = "social-auto"
version = "0.1.0"
edition = "2021"

[dependencies]
# Async runtime
tokio = { version = "1.35", features = ["full"] }

# Browser automation
chromiumoxide = "0.5"

# Web framework
axum = "0.7"
tower = "0.4"
tower-http = { version = "0.5", features = ["fs", "trace"] }

# Database
rusqlite = { version = "0.30", features = ["bundled"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"

# CLI
clap = { version = "4.4", features = ["derive"] }

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Crypto
aes-gcm = "0.10"
rand = "0.8"
base64 = "0.21"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Utilities
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.6", features = ["v4", "serde"] }

[dev-dependencies]
tempfile = "3.8"
```

- [ ] **Step 2: Create .gitignore**

```
/target
/data/*.db
/data/*.db-*
/profiles
*.log
.DS_Store
config.local.yaml
```

- [ ] **Step 3: Create config.yaml (already exists, verify content)**

- [ ] **Step 4: Create data directory**

```bash
mkdir -p data
touch data/.gitkeep
```

- [ ] **Step 5: Commit project initialization**

```bash
git add Cargo.toml .gitignore data/.gitkeep
git commit -m "chore: initialize project structure"
```

---

