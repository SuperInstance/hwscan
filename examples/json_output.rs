//! JSON output example
//!
//! This example shows how to output hardware information as JSON

use hwscan::HardwareDetector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Detect hardware
    let hw_info = HardwareDetector::detect()?;

    // Convert to JSON
    let json = hw_info.to_json()?;

    // Print JSON
    println!("{}", json);

    Ok(())
}
