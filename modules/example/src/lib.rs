use ultim::modules::builder::ModuleBuilder;

#[export_name = "ultim_init"]
pub fn init(
    builder: &mut ModuleBuilder,
    config: &serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", config["greeting"]);
    Ok(())
}
