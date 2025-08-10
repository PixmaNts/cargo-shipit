# Contributing to cargo-shipit

Thank you for your interest in contributing to cargo-shipit! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/cargo-shipit.git`
3. Create a feature branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes
6. Submit a pull request

## Development Setup

### Prerequisites

- Rust (latest stable version)
- Git

### Building the Project

```bash
# Clone the repository
git clone https://github.com/your-username/cargo-shipit.git
cd cargo-shipit

# Build the project
cargo build

# Run tests
cargo test

# Install locally for testing
cargo install --path .
```

## Code Style and Standards

### Formatting

- Use `cargo fmt` to format your code before committing
- Run `cargo clippy` to catch common mistakes and improve code quality

### Code Quality

- Write clear, readable code with meaningful variable and function names
- Add documentation comments for public APIs
- Include tests for new functionality
- Follow Rust naming conventions

### Commit Messages

- Use clear, descriptive commit messages
- Follow the format: `type: brief description`
- Examples:
  - `feat: add SSH key authentication support`
  - `fix: resolve authentication timeout issue`
  - `docs: update README with new CLI options`
  - `refactor: simplify configuration parsing`

## Testing

- Write unit tests for new functionality
- Ensure all existing tests pass: `cargo test`
- Test your changes with real SSH connections when possible
- Include integration tests for major features

## Documentation

- Update README.md if you add new features or change existing behavior
- Add inline documentation for public functions and modules
- Update help text in CLI arguments when adding new options

## Pull Request Process

1. Ensure your code follows the style guidelines
2. Run `cargo fmt` and `cargo clippy`
3. Make sure all tests pass
4. Update documentation as needed
5. Create a pull request with:
   - Clear title describing the change
   - Detailed description of what was changed and why
   - Reference any related issues

## Issue Reporting

When reporting issues, please include:

- Rust version (`rustc --version`)
- Operating system and version
- cargo-shipit version
- Steps to reproduce the issue
- Expected vs actual behavior
- Relevant configuration files (remove sensitive information)

## Security

If you discover a security vulnerability, please report it privately by emailing the maintainers rather than opening a public issue.

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help create a welcoming environment for all contributors

## License

By contributing to cargo-shipit, you agree that your contributions will be licensed under the same license as the project (MIT/Apache-2.0).