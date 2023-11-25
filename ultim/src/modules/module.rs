use std::error::Error;

use super::{
    builder::{ModuleBuilder, ModuleTaskRegistry},
    manifest::ModuleManifestEntry,
};

pub struct ModuleConfig {}

pub struct Module {
    // Immutable bits (modules themselves can't touch these)
    name: String,
    shared_library: libloading::Library,
    user_config: serde_json::Value,

    // What modules add stuff onto
    config: ModuleConfig,
}

type InitFn = fn(&mut ModuleBuilder, &serde_json::Value) -> Result<(), Box<dyn Error>>;

impl Module {
    pub fn load_from_manifest(entry: ModuleManifestEntry) -> Result<Self, libloading::Error> {
        let library = unsafe { libloading::Library::new(entry.path) }?;

        Ok(Self {
            name: entry.name,
            shared_library: library,
            user_config: entry.config,

            config: ModuleConfig {},
        })
    }

    pub fn initialize(
        &mut self,
        task_registry: &mut ModuleTaskRegistry,
    ) -> Result<(), Box<dyn Error>> {
        let init: libloading::Symbol<InitFn> = unsafe { self.shared_library.get(b"ultim_init")? };
        let mut builder = ModuleBuilder::for_module(&self.name, &mut self.config, task_registry);

        init(&mut builder, &self.user_config)
    }
}
