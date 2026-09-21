//! Integration example
//!
//! This example shows how to integrate hwscan into your application

use hwscan::HardwareDetector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Hardware Detection Integration Example ===\n");

    // Detect hardware
    let hw_info = HardwareDetector::detect()?;

    // Example 1: Check minimum requirements
    println!("1. Checking minimum requirements...");
    if hw_info.meets_minimum_requirements() {
        println!("   ✓ System meets minimum requirements");
    } else {
        println!("   ✗ System does NOT meet minimum requirements");
        println!("   Required: 8 GB RAM, 10 GB disk");
    }

    // Example 2: Get hardware tier
    println!("\n2. Hardware tier assessment...");
    let tier = hw_info.tier();
    println!("   Your system is rated as: {}", tier);
    println!("   Description: {}", tier.description());

    // Example 3: Model compatibility check
    println!("\n3. Model compatibility check...");
    let model_size_gb = 7;
    let model_size_bytes = model_size_gb as u64 * 1024 * 1024 * 1024;

    // Check GPU availability
    if hw_info.gpu.is_some() {
        let can_run_gpu = hw_info.can_run_model(model_size_bytes, true);
        println!(
            "   Can run {} GB model on GPU: {}",
            model_size_gb,
            if can_run_gpu { "Yes" } else { "No" }
        );
    } else {
        println!("   No GPU detected, checking CPU compatibility...");
    }

    let can_run_cpu = hw_info.can_run_model(model_size_bytes, false);
    println!(
        "   Can run {} GB model on CPU: {}",
        model_size_gb,
        if can_run_cpu { "Yes" } else { "No" }
    );

    // Example 4: Feature detection
    println!("\n4. CPU features...");
    println!("   Architecture: {}", hw_info.cpu.arch);
    if hw_info.cpu.has_feature("AVX2") {
        println!("   ✓ CPU supports AVX2 (optimized inference available)");
    } else {
        println!("   ✗ CPU does not support AVX2");
    }

    if hw_info.cpu.has_feature("NEON") {
        println!("   ✓ CPU supports NEON (ARM optimizations available)");
    }

    // Example 5: Platform-specific optimizations
    println!("\n5. Platform-specific optimizations...");
    if hw_info.platform.is_apple_silicon() {
        println!("   ✓ Apple Silicon detected - using Metal optimizations");
    } else if hw_info.platform.is_linux() {
        println!("   ✓ Linux detected - using CUDA/ROCm optimizations");
    } else if hw_info.platform.is_windows() {
        println!("   ✓ Windows detected - using DirectML optimizations");
    }

    // Example 6: Memory recommendations
    println!("\n6. Memory recommendations...");
    let ram_gb = hw_info.ram_bytes / (1024 * 1024 * 1024);
    let available_gb = hw_info.ram_available_bytes / (1024 * 1024 * 1024);
    println!("   Total RAM: {} GB", ram_gb);
    println!("   Available: {} GB", available_gb);

    // Recommend context size based on available RAM
    let recommended_ctx = if available_gb >= 32 {
        "16384 tokens (very large context)"
    } else if available_gb >= 16 {
        "8192 tokens (large context)"
    } else if available_gb >= 8 {
        "4096 tokens (standard context)"
    } else {
        "2048 tokens (small context)"
    };
    println!("   Recommended context size: {}", recommended_ctx);

    println!("\n=== Integration Example Complete ===");

    Ok(())
}
