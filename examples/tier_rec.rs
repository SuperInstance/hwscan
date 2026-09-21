//! Tier recommendation example
//!
//! This example shows how to get the hardware tier and recommendations

use hwscan::HardwareDetector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Detect hardware
    let hw_info = HardwareDetector::detect()?;

    // Get tier
    let tier = hw_info.tier();

    // Print tier information
    println!("Hardware Tier: {}", tier);
    println!("Description: {}", tier.description());

    // Get recommended model sizes
    let (min_gb, max_gb) = tier.recommended_model_size_gb();
    println!("Recommended model size: {:.1} - {:.1} GB", min_gb, max_gb);

    // Print detailed summary
    println!("\n{}", tier.summary_with_recommendations());

    // Check what models can run
    println!("\nModel Compatibility:");
    let test_sizes = vec![
        (3, "phi-3-mini (4K)"),
        (7, "llama-3.2-8b"),
        (13, "llama-3.1-8b Q5"),
        (20, "mixtral-8x7b Q4"),
        (40, "llama-3.1-70b Q4"),
    ];

    for (size_gb, model) in test_sizes {
        let size_bytes = size_gb as u64 * 1024 * 1024 * 1024;
        let can_run_gpu = hw_info.can_run_model(size_bytes, true);
        let can_run_cpu = hw_info.can_run_model(size_bytes, false);

        println!(
            "  {:20} ({} GB): GPU: {:5} | CPU: {:5}",
            model,
            size_gb,
            if can_run_gpu { "✓" } else { "✗" },
            if can_run_cpu { "✓" } else { "✗" }
        );
    }

    Ok(())
}
