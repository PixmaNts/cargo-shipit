# cargo-shipit

A Cargo subcommand for building and deploying Rust binaries to Linux targets via SSH.

## Features

- Cross-compile Rust projects for Linux targets
- Deploy binaries to remote devices via SSH/SCP
- SSH key and password authentication
- Automatic binary detection from Cargo.toml
- Intelligent build detection (only builds when needed)
- Support for debug, release, and custom build profiles
- Interactive directory creation on remote targets
- Relative path support in configuration files

## Installation

```bash
cargo install cargo-shipit
```

Or build from source:

```bash
git clone <repository-url>
cd cargo-shipit
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Initialize a config file
cargo shipit --init shipit.json

# Edit shipit.json with your target details, then deploy
cargo shipit

# Or specify options directly
cargo shipit --host 192.168.1.100 --username root --key ~/.ssh/id_rsa
```

### Command Line Options

```
Cargo subcommand for building and deploying Rust binaries to Linux targets via SSH. Ship it!

Usage: cargo-shipit [OPTIONS]

Options:
  -c, --config <CONFIG>                The path to the configuration file [default: shipit.json]
  -b, --binaries <BINARIES>            The binary files to upload to the Linux target (auto-detected from Cargo.toml if not specified)
  -d, --debug                          Enable debug mode
  -r, --remote-folder <REMOTE_FOLDER>  The remote directory path on the Linux target
  -H, --host <HOST>                    Hostname or IP address of the Linux target
  -U, --username <USERNAME>            SSH username for the Linux target
  -P, --password <PASSWORD>            SSH password for the Linux target
  -k, --key <KEY>                      Path to SSH private key file for authentication
  -p, --port <PORT>                    SSH port of the Linux target [default: 22]
  -t, --target <TARGET>                Rust target triple for cross-compilation (e.g., 'armv7-unknown-linux-gnueabihf', 'aarch64-unknown-linux-gnu')
  -T, --target-folder <TARGET_FOLDER>  Local cargo target directory path
  -i, --init <INIT>                    Initialize a new configuration file at the specified path
  -B, --build                          Build the project before uploading (auto-detected if not specified)
      --profile <PROFILE>              Build profile to use (debug, release, or custom profile name) [default: release]
  -h, --help                           Print help
  -V, --version                        Print version
```

### Configuration File

Create a configuration file to avoid specifying connection details every time:

```bash
cargo shipit --init embedded.json
```

This creates a template configuration file:

```json
{
  "host": "pi5.lan",
  "port": 22,
  "username": "pi", 
  "password": null,
  "key": "~/.ssh/id_rsa",
  "target_folder": "./target/",
  "target": "aarch64-unknown-linux-gnu",
  "remote_folder": "/home/pi/apps/",
  "profile": "release"
}
```

Configuration supports:
- Relative paths (relative to config file location)
- Tilde expansion for SSH keys
- Automatic binary detection
- Automatic build detection

Then use it:

```bash
cargo shipit --config embedded.json --binaries my_app --build
```

## SSH Authentication

cargo-shipit supports both password and SSH key-based authentication:

### Password Authentication

```bash
cargo shipit --host 192.168.1.100 --username root --password mypass --binaries my_app
```

### SSH Key Authentication

```bash
# Using SSH private key
cargo shipit --host 192.168.1.100 --username root --key ~/.ssh/id_rsa --binaries my_app
```

Or in configuration file:
```json
{
  "host": "192.168.1.100",
  "username": "root",
  "key": "/home/user/.ssh/id_rsa",
  "target": "armv7-unknown-linux-gnueabihf"
}
```

**Note**: When using SSH keys, cargo-shipit will look for both the private key file and its corresponding `.pub` public key file in the same directory.

## Cross-Compilation Setup

Before using cargo-shipit, ensure you have the appropriate Rust target installed:

```bash
# For ARM embedded Linux (e.g., Raspberry Pi, BeagleBone)
rustup target add armv7-unknown-linux-gnueabihf

# For ARM64 embedded Linux
rustup target add aarch64-unknown-linux-gnu

# For x86_64 embedded Linux
rustup target add x86_64-unknown-linux-gnu
```

You may also need to configure a cross-compilation toolchain in your `.cargo/config.toml`:

```toml
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

## Build Profiles

cargo-shipit supports different build profiles:

- **debug**: Use `--debug` flag or `--profile debug` for debug builds (faster compilation, larger binaries)
- **release**: Default profile for optimized builds (slower compilation, smaller binaries)
- **Custom profiles**: Specify any custom profile defined in your `Cargo.toml`

### Custom Profile Example

Define a custom profile in your `Cargo.toml`:

```toml
[profile.release-strip]
inherits = "release"
strip = true
lto = true
codegen-units = 1
```

Then use it:

```bash
cargo shipit --profile release-strip --binaries my_app --build
```

Or save it in your configuration:

```json
{
  "profile": "release-strip"
}
```

## Examples

### Example 1: Raspberry Pi Deployment

```bash
cargo shipit --init shipit.json
# Edit shipit.json with your Pi's details
cargo shipit
```

### Example 2: Direct Command Line

```bash
cargo shipit --host 192.168.1.100 --username pi --key ~/.ssh/id_rsa
```

## Security Notes

- Consider using SSH keys instead of passwords for better security
- Store sensitive configuration files outside of version control
- Use strong passwords for embedded device access
- Consider network security when deploying over wireless connections

## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Roadmap (08/2025)

### Authentication & Security
- [x] SSH key authentication support
- [ ] SSH agent integration
- [ ] Known hosts verification
- [ ] Config file encryption for stored passwords
- [ ] Support for SSH config file (~/.ssh/config)

### Remote Debugging
- [ ] GDB remote debugging support
  - [ ] Automatic gdbserver launch on target
  - [ ] Local GDB client connection setup
  - [ ] Cross-compilation toolchain detection
- [ ] LLDB remote debugging support
- [ ] Debug session management (start/stop/restart)

### IDE Integration
- [ ] VS Code launch.json generation for remote debugging

### Deployment Features
- [ ] Multiple target deployment (deploy to multiple hosts)
- [ ] Service management (systemd integration)
- [ ] File permission and ownership management
- [ ] Pre/post deployment hooks and scripts
- [ ] Rollback functionality
- [ ] Health checks after deployment

### Configuration & Usability
- [ ] Environment-based configurations (dev/staging/prod)
- [ ] Interactive configuration wizard
- [ ] Bash/Zsh completion scripts
- [ ] Verbose logging and better error messages
- [ ] Dry-run mode (show what would be deployed)

### Performance & Reliability
- [ ] Parallel deployment to multiple targets
- [ ] Incremental uploads (only changed binaries)
- [ ] Connection pooling and reuse
- [ ] Resume interrupted transfers
- [ ] Deployment verification and testing

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.