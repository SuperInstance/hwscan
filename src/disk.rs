//! Disk information

/// Disk information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiskInfo {
    /// Total disk space in bytes
    pub total_bytes: u64,
    /// Available disk space in bytes
    pub available_bytes: u64,
    /// Path to data directory
    pub data_path: String,
}

impl DiskInfo {
    /// Get disk usage percentage
    pub fn usage_percent(&self) -> f64 {
        if self.total_bytes == 0 {
            return 0.0;
        }
        let used = self.total_bytes - self.available_bytes;
        (used as f64 / self.total_bytes as f64) * 100.0
    }

    /// Get a summary of the disk
    pub fn summary(&self) -> String {
        format!(
            "{} / {} ({}% used)",
            crate::format_bytes(self.available_bytes),
            crate::format_bytes(self.total_bytes),
            self.usage_percent() as u32
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_usage_percent() {
        let disk = DiskInfo {
            total_bytes: 1000,
            available_bytes: 500,
            data_path: "/test".to_string(),
        };

        assert_eq!(disk.usage_percent(), 50.0);
    }

    #[test]
    fn test_disk_summary() {
        let disk = DiskInfo {
            total_bytes: 1024 * 1024 * 1024, // 1 GB
            available_bytes: 512 * 1024 * 1024, // 512 MB
            data_path: "/test".to_string(),
        };

        let summary = disk.summary();
        assert!(summary.contains("50%"));
    }
}
