# Contributing to hwscan

Thank you for your interest in contributing to hwscan! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to info@superinstance.ai.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When you create a bug report, include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps to reproduce** the problem
- **Provide specific examples** to demonstrate the issue
- **Describe the expected behavior**
- **Describe the actual behavior**
- **Include platform information**: OS, architecture, hardware details
- **Include relevant logs**, error messages, or screenshots

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When suggesting an enhancement:

- **Use a clear and descriptive title**
- **Provide a detailed description** of the suggested enhancement
- **Explain why this enhancement** would be useful
- **List some examples** of how this feature would be used
- **Include mock-ups** or screenshots if applicable

### Pull Requests

1. **Fork the repository** and create your branch from `main`.
2. **Make your changes** with clear, descriptive commit messages.
3. **Add tests** for new functionality or bug fixes.
4. **Ensure all tests pass**: `cargo test`
5. **Ensure no warnings**: `cargo clippy --all-targets --all-features`
6. **Format your code**: `cargo fmt`
7. **Update documentation** if you've changed APIs or behavior.
8. **Submit a pull request** with a clear description of changes.

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git

### Building

```bash
# Clone the repository
git clone https://github.com/SuperInstance/hwscan.git
cd hwscan

# Build
cargo build

# Run tests
cargo test

# Run examples
cargo run --example basic
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_hardware_detection

# Run on all targets (if available)
cargo test --all-targets
```

### Linting

```bash
# Check code style
cargo fmt --all -- --check

# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features
```

## Coding Standards

### Rust Style

- Follow standard Rust style guidelines (use `cargo fmt`)
- Avoid `unwrap()` in production code; use proper error handling
- Document public APIs with doc comments
- Include examples in doc comments
- Write unit tests for new functionality

### Documentation

- Use `///` for public API documentation
- Include examples in documentation
- Run `cargo doc` to verify documentation builds
- Include error conditions in documentation
- Document platform-specific behavior

### Commit Messages

Follow these guidelines for commit messages:

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally
- Consider starting the commit message with a relevant emoji:
  - ✨ for new features
  - 🐛 for bug fixes
  - 📝 for documentation
  - ♻️ for refactoring
  - ✅ for tests
  - 🚀 for performance improvements

Example:
```
✨ Add ARM64 GPU detection for Linux

Implement GPU detection for ARM64 platforms on Linux,
including support for NVIDIA Jetson devices.

Fixes #123
```

## Platform-Specific Considerations

### Linux

- Test on both x86_64 and ARM64 if possible
- Test with different GPU vendors (NVIDIA, AMD, Intel)
- Verify `nvidia-smi` and `rocm-smi` detection

### macOS

- Test on both Intel (x86_64) and Apple Silicon (ARM64)
- Verify Apple Silicon GPU detection
- Test `sw_vers` integration

### Windows

- Test on x86_64
- Verify GPU detection with different vendors
- Test disk space detection fallback

## Adding New Features

### GPU Detection

When adding support for new GPU types:

1. Create a new detection method in `src/lib.rs`
2. Add unit tests for the detection method
3. Update `GpuVendor` enum if needed
4. Document platform-specific behavior
5. Test on actual hardware if possible

### Platform Detection

When adding support for new platforms:

1. Implement platform-specific code with `#[cfg(target_os = "...")]`
2. Provide fallback mechanisms
3. Add comprehensive tests
4. Document limitations
5. Test on target platform

### Tier Calculation

When modifying tier thresholds:

1. Update constants in `src/tier.rs`
2. Update tier documentation
3. Update README table
4. Add tests for new thresholds
5. Consider backward compatibility

## Performance Considerations

- Hardware detection should complete in < 500ms
- Minimize subprocess calls
- Cache detection results when appropriate
- Avoid expensive operations in hot paths

## Testing Strategy

### Unit Tests

- Test individual functions and methods
- Mock external dependencies when needed
- Test error paths
- Test edge cases

### Integration Tests

- Test the full detection flow
- Test on multiple platforms
- Test with different hardware configurations
- Test error handling

### Documentation Tests

- Include examples in doc comments
- Ensure examples compile and run
- Test examples with `cargo test --doc`

## Release Process

Releases are managed by the maintainers. The process includes:

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Create git tag
4. Push to GitHub
5. Create GitHub release
6. Publish to crates.io

## Questions?

Feel free to open an issue for clarification or discussion.

---

Thank you for contributing to hwscan! 🎉
