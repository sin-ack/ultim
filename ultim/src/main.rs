use actix_web::{App, HttpServer};
use log::{info, trace};
use ultim::modules::{ModuleManifest, ModuleRegistry};

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .with_module_level("ultim", log::LevelFilter::Trace)
        .init()
        .unwrap();
    info!("Starting Ultim");

    let manifest = ModuleManifest::from_path("manifest.json").expect("Failed to load manifest");
    trace!("Loaded manifest: {:?}", manifest);
    let registry = match ModuleRegistry::from_manifest(manifest) {
        Ok(r) => r,
        Err(err) => {
            panic!("Failed to load modules: {}", err);
        }
    };

    info!("Initialized {} modules", registry.modules.len());

    let server = HttpServer::new(|| App::new())
        .bind(("0.0.0.0", 8000))?
        .run();
    info!("Server started at 0.0.0.0:8000");
    server.await
}
