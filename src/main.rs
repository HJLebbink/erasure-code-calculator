use crate::calc::{calculate, Configuration};
use log::LevelFilter;
use std::error::Error;
use crate::utils::StorageUnit;

mod calc;
mod calcation_result;
mod configuration;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_default_env() // Note: set environment variable $env:RUST_LOG = "vdb_client=DEBUG" to log info and higher
        .filter_level(LevelFilter::Debug)
        .init();

    let config = Configuration::new_with_unit(
        26,                         // racks
        5,                          // servers per rack
        24,                         // drives per server
        28608,
        StorageUnit::GiB,
        13,                         // stripe size
        3,                         // parity count
    )?;
    let result = calculate(&config)?;

    log::info!("config {}", config);
    log::info!("result {}", result);

    Ok(())
}
