//! CPU information

/// CPU information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CpuInfo {
    /// CPU model name
    pub name: String,
    /// Number of physical cores
    pub cores: usize,
    /// Number of logical threads
    pub threads: usize,
    /// CPU architecture
    pub arch: String,
    /// CPU features (AVX, AVX2, etc.)
    pub features: Vec<String>,
}

impl CpuInfo {
    /// Get a summary of the CPU
    pub fn summary(&self) -> String {
        format!(
            "{} ({} cores, {} threads, {})",
            self.name,
            self.cores,
            self.threads,
            self.arch
        )
    }

    /// Check if CPU supports a specific feature
    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.iter().any(|f| f.eq_ignore_ascii_case(feature))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_summary() {
        let cpu = CpuInfo {
            name: "Test CPU".to_string(),
            cores: 8,
            threads: 16,
            arch: "x86_64".to_string(),
            features: vec!["AVX".to_string(), "AVX2".to_string()],
        };

        let summary = cpu.summary();
        assert!(summary.contains("Test CPU"));
        assert!(summary.contains("8 cores"));
        assert!(summary.contains("16 threads"));
    }

    #[test]
    fn test_has_feature() {
        let cpu = CpuInfo {
            name: "Test CPU".to_string(),
            cores: 8,
            threads: 16,
            arch: "x86_64".to_string(),
            features: vec!["AVX".to_string(), "AVX2".to_string()],
        };

        assert!(cpu.has_feature("AVX"));
        assert!(cpu.has_feature("avx"));
        assert!(!cpu.has_feature("AVX512"));
    }
}
