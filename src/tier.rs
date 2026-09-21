//! Hardware tier calculation

use crate::HardwareInfo;

/// Tier thresholds for RAM (in GB)
const TIER_RAM_HIGH: u64 = 64; // >= 64GB: Tier 4
const TIER_RAM_MID: u64 = 32; // >= 32GB: Tier 3
const TIER_RAM_LOW: u64 = 16; // >= 16GB: Tier 2

/// Tier thresholds for GPU VRAM (in GB)
const TIER_VRAM_HIGH: u64 = 24; // >= 24GB: Tier 5
const TIER_VRAM_MID: u64 = 12; // >= 12GB: Tier 4
const TIER_VRAM_LOW: u64 = 8; // >= 8GB: Tier 3
const TIER_VRAM_MIN: u64 = 4; // >= 4GB: Tier 2

/// Hardware tier (1-5, higher is better)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum Tier {
    T1 = 1,
    T2 = 2,
    T3 = 3,
    T4 = 4,
    T5 = 5,
}

impl Tier {
    /// Get the tier as a number
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// Get the tier name
    pub fn name(self) -> &'static str {
        match self {
            Tier::T1 => "Minimal",
            Tier::T2 => "Basic",
            Tier::T3 => "Standard",
            Tier::T4 => "Performance",
            Tier::T5 => "Ultra",
        }
    }

    /// Get the tier description
    pub fn description(self) -> &'static str {
        match self {
            Tier::T1 => "Entry-level hardware, suitable for small models",
            Tier::T2 => "Basic hardware, can run medium-small models",
            Tier::T3 => "Standard hardware, can run medium models",
            Tier::T4 => "Performance hardware, can run large models",
            Tier::T5 => "High-end hardware, can run very large models",
        }
    }

    /// Get recommended model sizes for this tier
    pub fn recommended_model_size_gb(self) -> (f64, f64) {
        match self {
            Tier::T1 => (0.0, 3.0),      // 0-3 GB
            Tier::T2 => (2.0, 7.0),      // 2-7 GB
            Tier::T3 => (4.0, 13.0),     // 4-13 GB
            Tier::T4 => (8.0, 20.0),     // 8-20 GB
            Tier::T5 => (16.0, 100.0),   // 16+ GB
        }
    }

    /// Get the color for console output (ANSI escape code)
    pub fn color_code(self) -> &'static str {
        match self {
            Tier::T1 => "\x1b[31m", // Red
            Tier::T2 => "\x1b[33m", // Yellow
            Tier::T3 => "\x1b[32m", // Green
            Tier::T4 => "\x1b[36m", // Cyan
            Tier::T5 => "\x1b[35m", // Magenta
        }
    }

    /// Reset color code
    pub fn reset_code() -> &'static str {
        "\x1b[0m"
    }

    /// Calculate tier from hardware info
    pub fn from_hardware_info(hw: &HardwareInfo) -> Self {
        let mut tier = Tier::T1;

        // RAM tier
        let ram_gb = hw.ram_bytes / (1024 * 1024 * 1024);
        if ram_gb >= TIER_RAM_HIGH {
            tier = tier.max(Tier::T4);
        } else if ram_gb >= TIER_RAM_MID {
            tier = tier.max(Tier::T3);
        } else if ram_gb >= TIER_RAM_LOW {
            tier = tier.max(Tier::T2);
        }

        // GPU tier (can upgrade RAM tier)
        if let Some(ref gpu) = hw.gpu {
            let vram_gb = gpu.vram_bytes / (1024 * 1024 * 1024);
            if vram_gb >= TIER_VRAM_HIGH {
                tier = tier.max(Tier::T5);
            } else if vram_gb >= TIER_VRAM_MID {
                tier = tier.max(Tier::T4);
            } else if vram_gb >= TIER_VRAM_LOW {
                tier = tier.max(Tier::T3);
            } else if vram_gb >= TIER_VRAM_MIN {
                tier = tier.max(Tier::T2);
            }
        }

        tier
    }

    /// Get a summary with recommended model sizes
    pub fn summary_with_recommendations(self) -> String {
        let (min_gb, max_gb) = self.recommended_model_size_gb();
        format!(
            "{}Tier {} ({}): {} - Recommended model size: {:.1}-{:.1} GB{}",
            self.color_code(),
            self.as_u8(),
            self.name(),
            self.description(),
            min_gb,
            max_gb,
            Self::reset_code()
        )
    }

    /// Get a markdown-formatted summary
    pub fn markdown_summary(self) -> String {
        let (min_gb, max_gb) = self.recommended_model_size_gb();
        format!(
            "### Tier {} - {}\n\n**Description**: {}\n\n**Recommended model size**: {:.1} - {:.1} GB",
            self.as_u8(),
            self.name(),
            self.description(),
            min_gb,
            max_gb
        )
    }
}

impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tier {} ({})", self.as_u8(), self.name())
    }
}

impl Default for crate::CpuInfo {
    fn default() -> Self {
        crate::CpuInfo {
            name: "Unknown CPU".to_string(),
            cores: 1,
            threads: 1,
            arch: "unknown".to_string(),
            features: vec![],
        }
    }
}

impl Default for crate::DiskInfo {
    fn default() -> Self {
        crate::DiskInfo {
            total_bytes: 0,
            available_bytes: 0,
            data_path: "/".to_string(),
        }
    }
}

impl Default for crate::PlatformInfo {
    fn default() -> Self {
        crate::PlatformInfo {
            os: "unknown".to_string(),
            os_version: "unknown".to_string(),
            arch: "unknown".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_ordering() {
        assert!(Tier::T1 < Tier::T2);
        assert!(Tier::T2 < Tier::T3);
        assert!(Tier::T3 < Tier::T4);
        assert!(Tier::T4 < Tier::T5);
    }

    #[test]
    fn test_tier_descriptions() {
        assert_eq!(Tier::T1.name(), "Minimal");
        assert_eq!(Tier::T2.name(), "Basic");
        assert_eq!(Tier::T3.name(), "Standard");
        assert_eq!(Tier::T4.name(), "Performance");
        assert_eq!(Tier::T5.name(), "Ultra");
    }

    #[test]
    fn test_tier_from_ram() {
        // Test RAM-based tiers
        let hw_t1 = HardwareInfo {
            cpu: Default::default(),
            ram_bytes: 8 * 1024 * 1024 * 1024, // 8 GB
            ram_available_bytes: 8 * 1024 * 1024 * 1024,
            gpu: None,
            disk: Default::default(),
            platform: Default::default(),
        };
        assert_eq!(Tier::from_hardware_info(&hw_t1), Tier::T1);

        let hw_t2 = HardwareInfo {
            cpu: Default::default(),
            ram_bytes: 16 * 1024 * 1024 * 1024, // 16 GB
            ram_available_bytes: 16 * 1024 * 1024 * 1024,
            gpu: None,
            disk: Default::default(),
            platform: Default::default(),
        };
        assert_eq!(Tier::from_hardware_info(&hw_t2), Tier::T2);
    }

    #[test]
    fn test_tier_from_gpu() {
        // Test GPU-based tiers
        let hw_t5 = HardwareInfo {
            cpu: Default::default(),
            ram_bytes: 32 * 1024 * 1024 * 1024, // 32 GB RAM
            ram_available_bytes: 32 * 1024 * 1024 * 1024,
            gpu: Some(crate::GpuInfo {
                name: "RTX 4090".to_string(),
                vendor: crate::GpuVendor::Nvidia,
                vram_bytes: 24 * 1024 * 1024 * 1024, // 24 GB VRAM
                vram_available_bytes: 24 * 1024 * 1024 * 1024,
                cuda_version: Some("12.0".to_string()),
                supported: true,
            }),
            disk: Default::default(),
            platform: Default::default(),
        };
        assert_eq!(Tier::from_hardware_info(&hw_t5), Tier::T5);
    }
}
