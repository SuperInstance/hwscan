//! Platform information

/// Platform information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlatformInfo {
    /// Operating system
    pub os: String,
    /// OS version
    pub os_version: String,
    /// Architecture
    pub arch: String,
}

impl PlatformInfo {
    /// Get a summary of the platform
    pub fn summary(&self) -> String {
        format!("{} {} ({})", self.os, self.os_version, self.arch)
    }

    /// Check if running on Linux
    pub fn is_linux(&self) -> bool {
        self.os == "linux"
    }

    /// Check if running on macOS
    pub fn is_macos(&self) -> bool {
        self.os == "macos"
    }

    /// Check if running on Windows
    pub fn is_windows(&self) -> bool {
        self.os == "windows"
    }

    /// Check if running on x86_64
    pub fn is_x86_64(&self) -> bool {
        self.arch == "x86_64"
    }

    /// Check if running on ARM64
    pub fn is_aarch64(&self) -> bool {
        self.arch == "aarch64"
    }

    /// Check if running on Apple Silicon
    pub fn is_apple_silicon(&self) -> bool {
        self.is_macos() && self.is_aarch64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_summary() {
        let platform = PlatformInfo {
            os: "linux".to_string(),
            os_version: "6.0".to_string(),
            arch: "x86_64".to_string(),
        };

        let summary = platform.summary();
        assert!(summary.contains("linux"));
        assert!(summary.contains("6.0"));
        assert!(summary.contains("x86_64"));
    }

    #[test]
    fn test_is_linux() {
        let platform = PlatformInfo {
            os: "linux".to_string(),
            os_version: "6.0".to_string(),
            arch: "x86_64".to_string(),
        };

        assert!(platform.is_linux());
        assert!(!platform.is_macos());
        assert!(!platform.is_windows());
    }

    #[test]
    fn test_is_apple_silicon() {
        let platform = PlatformInfo {
            os: "macos".to_string(),
            os_version: "14.0".to_string(),
            arch: "aarch64".to_string(),
        };

        assert!(platform.is_apple_silicon());
    }
}
