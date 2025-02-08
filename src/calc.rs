use std::cmp::Ordering;
use crate::calcation_result::CalculationResult;
use std::error::Error;
pub(crate) use crate::configuration::Configuration;
use crate::configuration::ScalingRecommendation;

pub fn calculate(c: &Configuration) -> Result<CalculationResult, Box<dyn Error>> {

    // Calculate raw capacity
    let total_drives = c.total_drives();
    let raw_capacity = total_drives as u128 * c.drive_capacity_bytes;

    // Calculate storage efficiency (data drives / total drives in stripe)
    let data_drives = c.stripe_size - c.parity_count;
    let storage_efficiency = data_drives as f64 / c.stripe_size as f64;
    let usable_capacity = (raw_capacity as f64 * storage_efficiency) as u128;

    let total_servers = c.total_servers();
    
    // Calculate power and network estimates
    let watts_per_server = 300; // Approximate watts per server
    let power_estimate = total_servers * watts_per_server;

    let bandwidth_per_server = 25; // Gbps per server
    let total_bandwidth = total_servers * bandwidth_per_server;

    // Calculate failure tolerances
    let max_drive_tolerance = c.parity_count;
    let max_server_tolerance = if c.servers_per_rack >= c.stripe_size {
        c.parity_count
    } else {
        0
    };
    let max_rack_tolerance = if c.number_of_racks >= c.stripe_size {
        c.parity_count
    } else {
        0
    };

    Ok(CalculationResult {
        raw_capacity,
        usable_capacity,
        storage_efficiency,
        max_drive_tolerance,
        max_server_tolerance,
        max_rack_tolerance,
        total_servers,
        power_estimate_watts: power_estimate,
        network_bandwidth_gbps: total_bandwidth,
    })
}

pub fn recommend_scaling(c: Configuration, target_capacity: u128) -> Result<ScalingRecommendation, Box<dyn Error>> {
    let current_result = calculate(&c)?;

    match current_result.usable_capacity.cmp(&target_capacity) {
        Ordering::Equal => Ok(ScalingRecommendation {
            current_config: c.clone(),
            recommended_config: c.clone(),
            reason: "Current configuration meets capacity requirements".into(),
            capacity_increase: 0.0,
            efficiency_change: 0.0,
        }),
        Ordering::Less => {
            // Calculate how many more racks we need
            let capacity_ratio = (target_capacity as f64) / (current_result.usable_capacity as f64);
            let recommended_racks = (c.number_of_racks as f64 * capacity_ratio).ceil() as u32;

            let mut recommended_config = c.clone();
            recommended_config.number_of_racks = recommended_racks;

            let new_result = calculate(&recommended_config)?;

            Ok(ScalingRecommendation {
                current_config: c.clone(),
                recommended_config,
                reason: format!("Need {} additional racks to meet capacity requirements",
                                recommended_racks - c.number_of_racks),
                capacity_increase: (new_result.usable_capacity as f64) /
                    (current_result.usable_capacity as f64) - 1.0,
                efficiency_change: new_result.storage_efficiency - current_result.storage_efficiency,
            })
        },
        Ordering::Greater => {
            // Calculate how many fewer racks we need
            let capacity_ratio = (target_capacity as f64) / (current_result.usable_capacity as f64);
            let recommended_racks = (c.number_of_racks as f64 * capacity_ratio).ceil() as u32;

            let mut recommended_config = c.clone();
            recommended_config.number_of_racks = recommended_racks.max(1);

            let new_result = calculate(&recommended_config)?;

            Ok(ScalingRecommendation {
                current_config: c.clone(),
                recommended_config,
                reason: "Current configuration exceeds capacity requirements".into(),
                capacity_increase: (new_result.usable_capacity as f64) /
                    (current_result.usable_capacity as f64) - 1.0,
                efficiency_change: new_result.storage_efficiency - current_result.storage_efficiency,
            })
        }
    }
}




// Helper function to calculate valid stripe sizes
pub fn calculate_stripe_sizes(total_drives: u32) -> Vec<u32> {
    let mut sizes = Vec::new();
    let max_size = total_drives.min(16); // Maximum stripe size is typically 16

    for size in (4..=max_size).step_by(2) {
        // Start from 4, increment by 2
        if total_drives >= size {
            sizes.push(size);
        }
    }

    sizes
}

// Helper function to calculate valid parity counts for a given stripe size
pub fn calculate_parity_counts(stripe_size: u32) -> Vec<u32> {
    let mut counts = Vec::new();
    let mut count = stripe_size;

    while count >= 1 {
        if count % 2 == 0 {
            counts.push(count / 2);
        }
        count -= 1;
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_calculation() -> Result<(), Box<dyn Error>> {
        let config = Configuration::new_with_unit(
            1,                         // racks
            4,                         // servers per rack
            4,                         // drives per server
            4, 
            crate::utils::StorageUnit::TiB,
            8,                         // stripe size
            3,                         // parity count
        )?;

        let result = calculate(&config)?;

        assert_eq!(result.total_servers, 4);
        assert_eq!(result.max_drive_tolerance, 3);
        assert!(result.storage_efficiency > 0.6 && result.storage_efficiency < 0.7);

        Ok(())
    }
}
