use ultim_admin::AdminModule;

mod panels;

#[export_name = "ultim_init"]
pub fn init(
    registry: &ultim::ModuleRegistry,
    config: &serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    registry.with_module::<AdminModule>("admin", |admin| {
        admin.add_panel("Ultim", panels::PostPanel::new());
        Ok(())
    })?;
    Ok(())
}
