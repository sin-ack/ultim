use std::{cell::RefCell, error::Error};

use super::{manifest::ModuleManifest, module::Module, ModuleManifestEntry};

pub struct ModuleRegistry {
    pub modules: Vec<RegisteredModule>,
}

impl ModuleRegistry {
    pub fn from_manifest(manifest: ModuleManifest) -> Result<Self, Box<dyn Error>> {
        let mut registry = Self { modules: vec![] };

        for entry in manifest.modules {
            let module = RegisteredModule::load_from_entry(&registry, &entry)?;
            registry.modules.push(module);
        }

        Ok(registry)
    }

    pub fn with_module<F, T: Module>(&self, name: &str, f: F) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce(Option<&mut T>) -> Result<(), Box<dyn Error>>,
    {
        let registered_module = match self.modules.iter().find(|m| m.name == name) {
            Some(m) => m,
            None => return f(None),
        };

        let mut module_ref = registered_module.module.borrow_mut();
        let module = match module_ref.downcast_mut::<T>() {
            Some(m) => m,
            None => panic!(
                r#"!!! Module "{}" is not of type {}"#,
                name,
                std::any::type_name::<T>()
            ),
        };

        f(Some(module))
    }
}

pub struct RegisteredModule {
    name: String,
    // NOTE: We need to hold on to the shared object so that it doesn't get
    //      unloaded during Ultim's lifetime.
    #[allow(dead_code)]
    shared_object: libloading::Library,

    module: RefCell<Box<dyn Module>>,
}

type InitFn = fn(
    registry: &ModuleRegistry,
    value: &serde_json::Value,
) -> Result<Box<dyn Module>, Box<dyn Error>>;

impl RegisteredModule {
    pub fn load_from_entry(
        registry: &ModuleRegistry,
        entry: &ModuleManifestEntry,
    ) -> Result<Self, Box<dyn Error>> {
        // NOTE: This is safe because we are only loading modules we build
        //       together with Ultim as part of the same build process.
        let shared_object = unsafe { libloading::Library::new(entry.path.clone()) }?;
        let init = unsafe { shared_object.get::<InitFn>(b"ultim_init\0") }?;

        let module = init(registry, &entry.config)?;

        Ok(Self {
            name: entry.name.clone(),
            shared_object,

            module: RefCell::new(module),
        })
    }
}
