# Changelog

All notable changes to cargo-shipit will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-20

### Initial Release

First release of cargo-shipit, a Cargo subcommand for building and deploying Rust binaries to Linux targets via SSH.

### Features

- Cross-compilation and deployment to Linux targets
- SSH authentication (password and key-based)
- Interactive directory creation
- Debug, release, and custom build profile support
- JSON configuration files
- Automatic binary detection from Cargo.toml
- Intelligent build detection (builds only when needed)
- Relative path support in config files
- SSH key path expansion (~/path support)
- Executable permissions on uploaded binaries

### Usage

```bash
# Initialize config file
cargo shipit --init shipit.json

# Deploy with auto-detection
cargo shipit

# Direct deployment
cargo shipit --host pi.local --username pi --key ~/.ssh/id_rsa
```

### Supported Targets

Common Linux targets:
- `armv7-unknown-linux-gnueabihf` (ARM 32-bit)
- `aarch64-unknown-linux-gnu` (ARM 64-bit)  
- `x86_64-unknown-linux-gnu` (x86_64)

### Configuration Example

```json
{
  "host": "pi5.lan",
  "username": "pi",
  "key": "~/.ssh/id_rsa",
  "target": "aarch64-unknown-linux-gnu",
  "remote_folder": "/home/pi/apps/",
  "profile": "release"
}
```

---

[0.1.0]: https://github.com/PixmaNts/cargo-shipit/releases/tag/v0.1.0