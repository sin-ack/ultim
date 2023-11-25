use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ModuleManifestEntry {
    pub name: String,
    pub path: String,
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug)]
pub struct ModuleManifest {
    pub modules: Vec<ModuleManifestEntry>,
}

impl ModuleManifest {
    pub fn from_path(path: &str) -> Result<Self, std::io::Error> {
        let manifest = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&manifest)?)
    }
}
