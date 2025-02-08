use std::error::Error;
use std::fmt;
use crate::calc::calculate;
use crate::calcation_result::{CalculationResult};
use crate::utils::{format_bytes, StorageUnit};

#[derive(Debug)]
pub struct SystemRequirements {
    pub min_usable_capacity: u128,
    pub min_drive_tolerance: u32,
    pub min_server_tolerance: u32,
    pub min_rack_tolerance: u32,
    pub max_drive_size: u128,
}

#[derive(Debug)]
pub struct ScalingRecommendation {
    pub current_config: Configuration,
    pub recommended_config: Configuration,
    pub reason: String,
    pub capacity_increase: f64,
    pub efficiency_change: f64,
}

#[derive(Debug, Clone)]
pub struct Configuration {
    pub number_of_racks: u32,
    pub servers_per_rack: u32,
    pub drives_per_server: u32,
    pub drive_capacity_bytes: u128,
    pub stripe_size: u32,
    pub parity_count: u32,
}

impl Configuration {
    pub fn new(
        number_of_racks: u32,
        servers_per_rack: u32,
        drives_per_server: u32,
        drive_capacity_bytes: u128,
        stripe_size: u32,
        parity_count: u32,
    ) -> Result<Self, Box<dyn Error>> {
        Self::validate_configuration(
            number_of_racks,
            servers_per_rack,
            drives_per_server,
            drive_capacity_bytes,
            stripe_size,
            parity_count,
        )?;
        Ok(Configuration {
            number_of_racks,
            servers_per_rack,
            drives_per_server,
            drive_capacity_bytes,
            stripe_size,
            parity_count,
        })
    }

    pub fn new_with_unit(
        number_of_racks: u32,
        servers_per_rack: u32,
        drives_per_server: u32,
        drive_capacity: u128,
        drive_unit: StorageUnit,
        stripe_size: u32,
        parity_count: u32,
    ) -> Result<Self, Box<dyn Error>> {
        let drive_capacity_bytes = drive_capacity * drive_unit.to_bytes();
        Self::validate_configuration(
            number_of_racks,
            servers_per_rack,
            drives_per_server,
            drive_capacity_bytes,
            stripe_size,
            parity_count,
        )?;
        Ok(Configuration {
            number_of_racks,
            servers_per_rack,
            drives_per_server,
            drive_capacity_bytes,
            stripe_size,
            parity_count,
        })
    }
    
    fn validate_configuration(
        number_of_racks: u32,
        servers_per_rack: u32,
        drives_per_server: u32,
        drive_capacity_bytes: u128,
        stripe_size: u32,
        parity_count: u32,
    ) -> Result<(), Box<dyn Error>> {
        if number_of_racks < 1 {
            return Err("Number of racks must be at least 1".into());
        }
        if servers_per_rack < 1 {
            return Err("Servers per rack must be at least 1".into());
        }
        if drives_per_server < 1 {
            return Err("Drives per server must be at least 1".into());
        }
        if drive_capacity_bytes == 0 {
            return Err("Drive capacity must be greater than 0".into());
        }
        if stripe_size < parity_count {
            return Err("Stripe size must be greater than parity count".into());
        }
        Ok(())
    }

    pub fn check_requirements(&self, requirements: &SystemRequirements) -> Result<(), String> {
        let result: CalculationResult = calculate(&self).map_err(|e| e.to_string())?;

        if result.usable_capacity < requirements.min_usable_capacity {
            return Err("Configuration does not meet minimum usable capacity requirement".into());
        }
        if result.max_drive_tolerance < requirements.min_drive_tolerance {
            return Err("Configuration does not meet minimum drive tolerance requirement".into());
        }
        if result.max_server_tolerance < requirements.min_server_tolerance {
            return Err("Configuration does not meet minimum server tolerance requirement".into());
        }
        if result.max_rack_tolerance < requirements.min_rack_tolerance {
            return Err("Configuration does not meet minimum rack tolerance requirement".into());
        }
        if self.drive_capacity_bytes > requirements.max_drive_size {
            return Err("Drive capacity exceeds maximum allowed size".into());
        }

        Ok(())
    }

    /// Calculate total number of drives in the system
    pub fn total_drives(&self) -> u64 {
        self.number_of_racks as u64 * self.servers_per_rack as u64 * self.drives_per_server as u64
    }

    /// Calculate total raw capacity of the system in bytes
    pub fn total_raw_capacity(&self) -> u128 {
        self.total_drives() as u128 * self.drive_capacity_bytes
    }
    
    pub fn total_servers(&self) -> u32 {
        self.number_of_racks * self.servers_per_rack
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Erasure Coding Configuration:")?;
        writeln!(f, "----------------------------")?;
        writeln!(f, "Physical Layout:")?;
        writeln!(f, "  - Number of Racks:      {}", self.number_of_racks)?;
        writeln!(f, "  - Servers per Rack:     {}", self.servers_per_rack)?;
        writeln!(f, "  - Drives per Server:    {}", self.drives_per_server)?;
        writeln!(f, "  - Drive Capacity:       {}", format_bytes(self.drive_capacity_bytes))?;
        writeln!(f)?;
        writeln!(f, "Totals:")?;
        writeln!(f, "  - Total Servers:        {}", self.number_of_racks * self.servers_per_rack)?;
        writeln!(f, "  - Total Drives:         {}", self.total_drives())?;
        writeln!(f, "  - Total Raw Capacity:   {}", format_bytes(self.total_raw_capacity()))?;
        writeln!(f)?;
        writeln!(f, "Erasure Coding Settings:")?;
        writeln!(f, "  - Stripe Size:          {}", self.stripe_size)?;
        writeln!(f, "  - Parity Count:         {}", self.parity_count)?;
        writeln!(f, "  - Data Drives/Stripe:   {}", self.stripe_size - self.parity_count)
    }
}