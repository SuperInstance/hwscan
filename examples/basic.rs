//! Basic usage example
//!
//! This is the simplest example of how to use hwscan

use hwscan::HardwareDetector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Detect hardware
    let hw_info = HardwareDetector::detect()?;

    // Print summary
    println!("Hardware Summary:");
    println!("{}", hw_info.summary());

    // Print tier
    println!("\nHardware Tier: {}", hw_info.tier());

    Ok(())
}
