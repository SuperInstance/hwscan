# hwscan v0.1.0 Release Notes

## Overview

hwscan is a cross-platform hardware detection and tier calculation library for Rust. This initial release provides comprehensive hardware detection capabilities with a simple API and CLI tool.

## What's New

### Features

- **Cross-platform hardware detection**
  - Linux (x86_64, ARM64)
  - macOS (Intel, Apple Silicon)
  - Windows (x86_64)

- **GPU detection**
  - NVIDIA GPUs (via nvidia-smi)
  - AMD GPUs (via rocm-smi)
  - Intel GPUs (via sycl-ls)
  - Apple Silicon GPUs (unified memory)

- **CPU detection**
  - Model name and architecture
  - Core and thread counts
  - Feature detection (AVX, AVX2, AVX512, FMA, NEON)

- **Memory detection**
  - Total RAM
  - Available RAM
  - Disk space

- **Tier calculation**
  - Hardware tier 1-5 based on RAM and VRAM
  - Model size recommendations
  - Compatibility checking

- **CLI tool**
  - Human-readable output
  - JSON output mode
  - Markdown report generation
  - Tier-only mode

### API

```rust
use hwscan::HardwareDetector;

// Detect hardware
let hw_info = HardwareDetector::detect()?;

// Get tier
let tier = hw_info.tier();

// Check compatibility
let can_run = hw_info.can_run_model(model_size, needs_gpu);
```

### CLI Usage

```bash
# Install
cargo install hwscan

# Run
hwscan

# JSON output
hwscan --json

# Tier only
hwscan --tier

# Markdown report
hwscan --markdown
```

## Documentation

- [README](README.md) - Getting started guide
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [docs.rs](https://docs.rs/hwscan) - API documentation

## Examples

Five comprehensive examples included:
- `basic.rs` - Simple hardware detection
- `json_output.rs` - JSON output example
- `tier_rec.rs` - Tier recommendations
- `markdown_report.rs` - Markdown report generation
- `integration.rs` - Integration example

## Platform Support

| Platform | Architecture | GPU Support | Status |
|----------|-------------|-------------|--------|
| Linux | x86_64 | NVIDIA, AMD, Intel | ✅ Full |
| Linux | ARM64 | NVIDIA, AMD | ✅ Full |
| macOS | x86_64 | AMD, Intel | ✅ Full |
| macOS | ARM64 | Apple Silicon | ✅ Full |
| Windows | x86_64 | NVIDIA, AMD, Intel | ✅ Full |

## Testing

- **Unit tests**: 17 tests, all passing
- **Doc tests**: 4 tests, all passing
- **Total**: 21/21 tests passing (100%)
- **Clippy**: Zero warnings
- **Formatting**: Passes `cargo fmt`

## Dependencies

- `sysinfo` - System information detection
- `serde` - Serialization framework
- `serde_json` - JSON support (optional, CLI feature)
- `clap` - CLI argument parsing (optional, CLI feature)
- `dirs` - Directory resolution
- `thiserror` - Error handling
- `tracing` - Instrumentation

## Performance

- Detection time: 100-500ms depending on platform
- Memory usage: ~2-3 MB
- No external dependencies for detection

## License

Dual-licensed under:
- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Known Limitations

- Windows disk detection uses fallback values (no native implementation yet)
- AMD GPU VRAM availability may not be accurate on all systems
- Intel GPU VRAM is reported as 0 (integrated GPUs use system memory)

## Future Work

- [ ] Windows-specific disk space detection
- [ ] More accurate AMD GPU VRAM reporting
- [ ] Additional GPU vendors (e.g., Qualcomm, MediaTek)
- [ ] Battery detection for laptops
- [ ] Thermal information
- [ ] Network interface detection
- [ ] Benchmarking suite
- [ ] WebAssembly support

## Acknowledgments

Built with:
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - System information detection
- [serde](https://github.com/serde-rs/serde) - Serialization framework
- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing

## Links

- [GitHub Repository](https://github.com/SuperInstance/hwscan)
- [crates.io](https://crates.io/crates/hwscan)
- [Documentation](https://docs.rs/hwscan)
- [Issue Tracker](https://github.com/SuperInstance/hwscan/issues)

---

**Made with ❤️ by the SuperInstance team**
