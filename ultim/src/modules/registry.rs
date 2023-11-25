use super::{manifest::ModuleManifest, module::Module};

pub struct ModuleRegistry {
    pub modules: Vec<Module>,
}

impl ModuleRegistry {
    pub fn from_manifest(manifest: ModuleManifest) -> Result<Self, libloading::Error> {
        let modules = manifest
            .modules
            .into_iter()
            .map(|entry| Module::load_from_manifest(entry))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { modules })
    }
}
