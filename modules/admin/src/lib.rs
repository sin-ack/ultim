#![feature(return_position_impl_trait_in_trait)]

use log::info;
use ultim::{Module, ModuleRegistry};

pub mod panel;
pub mod view;

pub struct AdminModule {}

impl Module for AdminModule {
    fn name(&self) -> &'static str {
        "admin"
    }

    fn description(&self) -> &'static str {
        "A flexible admin dashboard system for Ultim using HATEOAS principles."
    }
}

#[export_name = "ultim_init"]
pub fn init(
    _registry: &ModuleRegistry,
    _config: &serde_json::Value,
) -> Result<Box<dyn Module>, Box<dyn std::error::Error>> {
    info!("Admin module initialized");
    Ok(Box::new(AdminModule {}))
}
