use log::debug;
use sqlx::migrate::Migrator;

use super::module::ModuleConfig;

pub struct ModuleTaskRegistry {
    migrators: Vec<&'static Migrator>,
}

impl Default for ModuleTaskRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ModuleTaskRegistry {
    pub fn new() -> Self {
        Self { migrators: vec![] }
    }
}

pub struct ModuleBuilder<'a> {
    name: &'a str,
    config: &'a mut ModuleConfig,
    task_registry: &'a mut ModuleTaskRegistry,
}

impl<'a> ModuleBuilder<'a> {
    pub fn for_module(
        name: &'a str,
        config: &'a mut ModuleConfig,
        task_registry: &'a mut ModuleTaskRegistry,
    ) -> Self {
        Self {
            name,
            config,
            task_registry,
        }
    }

    pub fn add_migrator(&mut self, migrator: &'static Migrator) -> &mut Self {
        debug!("Adding migrator for module {}", self.name);
        self.task_registry.migrators.push(migrator);
        self
    }
}
