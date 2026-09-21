//! hwscan CLI tool
//!
//! Cross-platform hardware detection and tier calculation

use clap::Parser;
use hwscan::HardwareDetector;

#[derive(Parser, Debug)]
#[command(name = "hwscan")]
#[command(about = "Cross-platform hardware detection and tier calculation", long_about = None)]
#[command(version = "0.1.0")]
struct Args {
    /// Output as JSON
    #[arg(short, long)]
    json: bool,

    /// Only show tier
    #[arg(short, long)]
    tier: bool,

    /// Generate markdown report
    #[arg(short, long)]
    markdown: bool,

    /// Colored output
    #[arg(short, long, default_value_t = true)]
    color: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Detect hardware
    let hw_info = HardwareDetector::detect()?;

    if args.json {
        // JSON output
        println!("{}", hw_info.to_json()?);
        return Ok(());
    }

    if args.tier {
        // Tier only
        let tier = hw_info.tier();
        if args.color {
            println!("{}", tier.summary_with_recommendations());
        } else {
            println!("{} - {}", tier, tier.description());
            println!(
                "Recommended model size: {:.1} - {:.1} GB",
                tier.recommended_model_size_gb().0,
                tier.recommended_model_size_gb().1
            );
        }
        return Ok(());
    }

    if args.markdown {
        // Markdown report
        println!("# Hardware Scan Report\n");
        println!("## Platform\n");
        println!("- **OS**: {}", hw_info.platform.os_version);
        println!("- **Architecture**: {}\n", hw_info.platform.arch);

        println!("## CPU\n");
        println!("- **Model**: {}", hw_info.cpu.name);
        println!("- **Cores**: {}", hw_info.cpu.cores);
        println!("- **Threads**: {}", hw_info.cpu.threads);
        println!("- **Architecture**: {}", hw_info.cpu.arch);
        if !hw_info.cpu.features.is_empty() {
            println!("- **Features**: {}\n", hw_info.cpu.features.join(", "));
        }

        println!("## Memory\n");
        println!(
            "- **Total RAM**: {}",
            hwscan::format_bytes(hw_info.ram_bytes)
        );
        println!(
            "- **Available RAM**: {}\n",
            hwscan::format_bytes(hw_info.ram_available_bytes)
        );

        if let Some(ref gpu) = hw_info.gpu {
            println!("## GPU\n");
            println!("- **Model**: {}", gpu.name);
            println!("- **Vendor**: {}", gpu.vendor.as_str());
            println!(
                "- **VRAM**: {} / {}",
                hwscan::format_bytes(gpu.vram_available_bytes),
                hwscan::format_bytes(gpu.vram_bytes)
            );
            if let Some(ref cuda) = gpu.cuda_version {
                println!("- **CUDA Version**: {}\n", cuda);
            } else {
                println!();
            }
        }

        println!("## Disk\n");
        println!(
            "- **Total**: {}",
            hwscan::format_bytes(hw_info.disk.total_bytes)
        );
        println!(
            "- **Available**: {}\n",
            hwscan::format_bytes(hw_info.disk.available_bytes)
        );

        println!("## Hardware Tier\n");
        println!("{}", hw_info.tier().markdown_summary());
        return Ok(());
    }

    // Default: human-readable output
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                    Hardware Scan Report                  ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("📌 Platform");
    println!("   OS:        {}", hw_info.platform.os_version);
    println!("   Arch:      {}\n", hw_info.platform.arch);

    println!("💻 CPU");
    println!("   Model:     {}", hw_info.cpu.name);
    println!("   Cores:     {}", hw_info.cpu.cores);
    println!("   Threads:   {}", hw_info.cpu.threads);
    if !hw_info.cpu.features.is_empty() {
        println!("   Features:  {}", hw_info.cpu.features.join(", "));
    }
    println!();

    println!("🧠 Memory");
    println!(
        "   Total:     {}",
        hwscan::format_bytes(hw_info.ram_bytes)
    );
    println!(
        "   Available: {}\n",
        hwscan::format_bytes(hw_info.ram_available_bytes)
    );

    if let Some(ref gpu) = hw_info.gpu {
        println!("🎮 GPU");
        println!("   Model:     {}", gpu.name);
        println!("   Vendor:    {}", gpu.vendor.as_str());
        println!(
            "   VRAM:      {} / {}",
            hwscan::format_bytes(gpu.vram_available_bytes),
            hwscan::format_bytes(gpu.vram_bytes)
        );
        if let Some(ref cuda) = gpu.cuda_version {
            println!("   CUDA:      {}", cuda);
        }
        println!();
    }

    println!("💾 Disk");
    println!(
        "   Total:     {}",
        hwscan::format_bytes(hw_info.disk.total_bytes)
    );
    println!(
        "   Available: {}\n",
        hwscan::format_bytes(hw_info.disk.available_bytes)
    );

    let tier = hw_info.tier();
    if args.color {
        println!("📊 Hardware Tier\n");
        println!("{}", tier.summary_with_recommendations());
    } else {
        println!("📊 Hardware Tier");
        println!("   {} - {}", tier, tier.description());
        println!(
            "   Recommended model size: {:.1} - {:.1} GB",
            tier.recommended_model_size_gb().0,
            tier.recommended_model_size_gb().1
        );
    }

    Ok(())
}
