# hwscan

[![crates.io](https://img.shields.io/crates/v/hwscan.svg)](https://crates.io/crates/hwscan)
[![docs.rs](https://img.shields.io/docsrs/hwscan.svg)](https://docs.rs/hwscan)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Build Status](https://github.com/SuperInstance/hwscan/workflows/CI/badge.svg)](https://github.com/SuperInstance/hwscan/actions)

> Cross-platform hardware detection and tier calculation for Rust

**hwscan** detects your system's hardware capabilities (CPU, GPU, RAM, disk) and calculates a hardware tier to help you choose the right models for your system.

## Features

- **Cross-platform**: Linux (x86_64, ARM64), macOS (Intel, Apple Silicon), Windows (x86_64)
- **GPU Detection**: Automatically detects NVIDIA, AMD, Intel, and Apple Silicon GPUs
- **Tier Calculation**: Calculates a hardware tier (1-5) based on RAM and VRAM
- **Fast**: Detection completes in 100-500ms
- **CLI Tool**: Command-line interface for quick hardware scanning
- **Easy Integration**: Simple API for Rust applications

## Quick Start

### As a Library

```rust
use hwscan::HardwareDetector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Detect hardware
    let hw_info = HardwareDetector::detect()?;

    // Print summary
    println!("{}", hw_info.summary());

    // Get hardware tier
    println!("Hardware tier: {}", hw_info.tier());

    Ok(())
}
```

### As a CLI Tool

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

## Hardware Tiers

The tier system helps you understand what your system can handle:

| Tier | Name | RAM | VRAM | Model Size |
|------|------|-----|------|------------|
| **5** | Ultra | 64+ GB | 24+ GB | 16-100 GB |
| **4** | Performance | 32+ GB | 12+ GB | 8-20 GB |
| **3** | Standard | 16+ GB | 8+ GB | 4-13 GB |
| **2** | Basic | 16+ GB | 4+ GB | 2-7 GB |
| **1** | Minimal | 8 GB | None | 0-3 GB |

The final tier is the **maximum** of RAM and GPU tiers.

## Platform Support

| Platform | Architecture | GPU Support | Status |
|----------|-------------|-------------|--------|
| Linux | x86_64 | NVIDIA, AMD, Intel | ✅ Full |
| Linux | ARM64 | NVIDIA, AMD | ✅ Full |
| macOS | x86_64 | AMD, Intel | ✅ Full |
| macOS | ARM64 | Apple Silicon | ✅ Full |
| Windows | x86_64 | NVIDIA, AMD, Intel | ✅ Full |

## Examples

See the [examples](examples/) directory:

- **basic.rs** - Simple hardware detection
- **json_output.rs** - Output as JSON
- **tier_rec.rs** - Get tier recommendations
- **markdown_report.rs** - Generate markdown report
- **integration.rs** - Integration example

```bash
# Run examples
cargo run --example basic
cargo run --example tier_rec
cargo run --example markdown_report
```

## API Documentation

### HardwareDetector

```rust
use hwscan::HardwareDetector;

// Detect all hardware
let hw_info = HardwareDetector::detect()?;
```

### HardwareInfo

```rust
// Check minimum requirements (8 GB RAM, 10 GB disk)
let meets_min = hw_info.meets_minimum_requirements();

// Check if a model can run
let model_size = 4 * 1024 * 1024 * 1024; // 4 GB
let can_run_gpu = hw_info.can_run_model(model_size, true);
let can_run_cpu = hw_info.can_run_model(model_size, false);

// Get hardware tier
let tier = hw_info.tier();
println!("Tier: {}", tier);

// Get summary
println!("{}", hw_info.summary());
```

### Tier

```rust
use hwscan::Tier;

// Get tier information
let tier = hw_info.tier();

// Tier name and description
println!("Name: {}", tier.name());        // "Ultra", "Performance", etc.
println!("Description: {}", tier.description());

// Recommended model sizes
let (min_gb, max_gb) = tier.recommended_model_size_gb();
println!("Recommended: {:.1} - {:.1} GB", min_gb, max_gb);

// Summary with recommendations
println!("{}", tier.summary_with_recommendations());

// Markdown summary
println!("{}", tier.markdown_summary());
```

## CLI Usage

### Basic Output

```bash
$ hwscan
╔════════════════════════════════════════════════════════════╗
║                    Hardware Scan Report                  ║
╚════════════════════════════════════════════════════════════╝

📌 Platform
   OS:        Linux kernel 6.5.0
   Arch:      x86_64

💻 CPU
   Model:     Intel(R) Core(TM) i7-12700K
   Cores:     12
   Threads:   20
   Features:  AVX, AVX2

🧠 Memory
   Total:     31.2 GB
   Available: 24.1 GB

🎮 GPU
   Model:     NVIDIA GeForce RTX 3060
   Vendor:    NVIDIA
   VRAM:      12.0 GB / 12.0 GB
   CUDA:      12.0

💾 Disk
   Total:     1.8 TB
   Available: 500.2 GB

📊 Hardware Tier
   Tier 4 (Performance): Performance hardware, can run large models
   - Recommended model size: 8.0 - 20.0 GB
```

### JSON Output

```bash
$ hwscan --json
{
  "cpu": {
    "name": "Intel(R) Core(TM) i7-12700K",
    "cores": 12,
    "threads": 20,
    "arch": "x86_64",
    "features": ["AVX", "AVX2"]
  },
  "ram_bytes": 33554432000,
  "ram_available_bytes": 25899827200,
  "gpu": {
    "name": "NVIDIA GeForce RTX 3060",
    "vendor": "Nvidia",
    "vram_bytes": 12884901888,
    "vram_available_bytes": 12884901888,
    "cuda_version": "12.0",
    "supported": true
  },
  "disk": {
    "total_bytes": 1995006438400,
    "available_bytes": 537875466240,
    "data_path": "/home/user/.hwscan"
  },
  "platform": {
    "os": "linux",
    "os_version": "Linux kernel 6.5.0",
    "arch": "x86_64"
  }
}
```

## Use Cases

### Model Selection

```rust
let hw_info = HardwareDetector::detect()?;
let tier = hw_info.tier();

match tier {
    Tier::T5 => println!("Running 70B parameter model"),
    Tier::T4 => println!("Running 8B parameter model"),
    Tier::T3 => println!("Running 3B parameter model"),
    Tier::T2 | Tier::T1 => println!("Running small model or using API"),
}
```

### System Requirements Check

```rust
let hw_info = HardwareDetector::detect()?;

if !hw_info.meets_minimum_requirements() {
    eprintln!("Error: System does not meet minimum requirements");
    eprintln!("Required: 8 GB RAM, 10 GB disk");
    std::process::exit(1);
}
```

### Resource Allocation

```rust
let hw_info = HardwareDetector::detect()?;

// Allocate 80% of available RAM
let ram_to_use = (hw_info.ram_available_bytes as f64 * 0.8) as u64;

// If GPU available, use VRAM
if let Some(ref gpu) = hw_info.gpu {
    let vram_to_use = (gpu.vram_available_bytes as f64 * 0.9) as u64;
    println!("Using {} of GPU VRAM", format_bytes(vram_to_use));
}
```

## Performance

Hardware detection is fast and efficient:

| Platform | Detection Time | Memory Usage |
|----------|---------------|--------------|
| Linux | 100-200ms | ~2 MB |
| macOS | 150-300ms | ~2 MB |
| Windows | 200-500ms | ~3 MB |

Results should be cached for the application lifetime.

## Used By

- [SuperInstance](https://github.com/SuperInstance) - Multi-agent AI orchestration platform

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

This project is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license for your use.

## Acknowledgments

Built with:
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - System information detection
- [serde](https://github.com/serde-rs/serde) - Serialization framework
- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing

---

**Made with ❤️ by the SuperInstance team**
