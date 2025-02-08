use std::fmt;
use crate::utils::format_bytes;

#[derive(Debug)]
pub struct CalculationResult {
    pub(crate) raw_capacity: u128,
    pub(crate) usable_capacity: u128,
    pub(crate) storage_efficiency: f64,
    pub(crate) max_drive_tolerance: u32,
    pub(crate) max_server_tolerance: u32,
    pub(crate) max_rack_tolerance: u32,
    pub(crate) total_servers: u32,
    pub(crate) power_estimate_watts: u32,
    pub(crate) network_bandwidth_gbps: u32,
}

impl fmt::Display for CalculationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Storage Configuration Results:")?;
        writeln!(f, "----------------------------")?;
        writeln!(
            f,
            "Usable Capacity:     {}",
            format_bytes(self.usable_capacity)
        )?;
        writeln!(
            f,
            "Raw Capacity:        {}",
            format_bytes(self.raw_capacity)
        )?;
        writeln!(
            f,
            "Storage Efficiency:  {:.1}%",
            self.storage_efficiency * 100.0
        )?;
        writeln!(f, "Total Servers:       {}", self.total_servers)?;
        writeln!(f, "Failure Tolerance:")?;
        writeln!(f, "  - Racks:           {} rack failures per stripe", self.max_rack_tolerance)?;
        writeln!(f, "  - Servers:         {} server failures per stripe", self.max_server_tolerance)?;
        writeln!(f, "  - Drives:          {} drive failures per stripe", self.max_drive_tolerance)?;
        
        
        writeln!(f, "Power estimate       {} Watt", self.power_estimate_watts)?;
        writeln!(f, "Network Bandwidth    {} Gbps", self.network_bandwidth_gbps)
    }
}

