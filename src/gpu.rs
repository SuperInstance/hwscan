//! GPU information

/// GPU information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GpuInfo {
    /// GPU name
    pub name: String,
    /// GPU vendor (nvidia, amd, intel, apple)
    pub vendor: GpuVendor,
    /// Total VRAM in bytes
    pub vram_bytes: u64,
    /// Available VRAM in bytes
    pub vram_available_bytes: u64,
    /// CUDA compute capability (for NVIDIA)
    pub cuda_version: Option<String>,
    /// Whether the GPU supports the required features
    pub supported: bool,
}

/// GPU vendor
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Apple,
    Other,
}

impl GpuVendor {
    /// Get the vendor name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            GpuVendor::Nvidia => "NVIDIA",
            GpuVendor::Amd => "AMD",
            GpuVendor::Intel => "Intel",
            GpuVendor::Apple => "Apple",
            GpuVendor::Other => "Other",
        }
    }
}

impl GpuInfo {
    /// Get VRAM in GB
    pub fn vram_gb(&self) -> f64 {
        self.vram_bytes as f64 / (1024.0 * 1024.0 * 1024.0)
    }

    /// Get available VRAM in GB
    pub fn vram_available_gb(&self) -> f64 {
        self.vram_available_bytes as f64 / (1024.0 * 1024.0 * 1024.0)
    }

    /// Get a summary of the GPU
    pub fn summary(&self) -> String {
        format!(
            "{} {} ({:.1} GB VRAM)",
            self.vendor.as_str(),
            self.name,
            self.vram_gb()
        )
    }

    /// Check if the GPU has sufficient VRAM
    pub fn has_sufficient_vram(&self, required_bytes: u64) -> bool {
        self.vram_available_bytes >= required_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_vendor_as_str() {
        assert_eq!(GpuVendor::Nvidia.as_str(), "NVIDIA");
        assert_eq!(GpuVendor::Amd.as_str(), "AMD");
        assert_eq!(GpuVendor::Intel.as_str(), "Intel");
        assert_eq!(GpuVendor::Apple.as_str(), "Apple");
    }

    #[test]
    fn test_vram_gb() {
        let gpu = GpuInfo {
            name: "Test GPU".to_string(),
            vendor: GpuVendor::Nvidia,
            vram_bytes: 8 * 1024 * 1024 * 1024, // 8 GB
            vram_available_bytes: 8 * 1024 * 1024 * 1024,
            cuda_version: Some("12.0".to_string()),
            supported: true,
        };

        assert_eq!(gpu.vram_gb(), 8.0);
    }

    #[test]
    fn test_has_sufficient_vram() {
        let gpu = GpuInfo {
            name: "Test GPU".to_string(),
            vendor: GpuVendor::Nvidia,
            vram_bytes: 8 * 1024 * 1024 * 1024, // 8 GB
            vram_available_bytes: 8 * 1024 * 1024 * 1024,
            cuda_version: Some("12.0".to_string()),
            supported: true,
        };

        assert!(gpu.has_sufficient_vram(4 * 1024 * 1024 * 1024)); // 4 GB
        assert!(!gpu.has_sufficient_vram(16 * 1024 * 1024 * 1024)); // 16 GB
    }
}
