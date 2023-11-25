use actix_web::{web, App, HttpResponse, HttpServer};
use log::{info, trace};
use ultim::modules::{
    builder::ModuleTaskRegistry,
    manifest::ModuleManifest,
    registry::ModuleRegistry,
};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

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
    let registry = ModuleRegistry::from_manifest(manifest);
    if registry.is_err() {
        panic!("Failed to load modules: {:?}", registry.err());
    }

    let mut registry = registry.unwrap();
    info!("Loaded {} modules", registry.modules.len());

    let mut task_registry = ModuleTaskRegistry::new();

    for module in registry.modules.iter_mut() {
        module
            .initialize(&mut task_registry)
            .expect("Failed to initialize module");
    }

    let server = HttpServer::new(|| App::new().service(web::resource("/").to(index)))
        .bind(("0.0.0.0", 8000))?
        .run();
    info!("Server started at 0.0.0.0:8000");
    server.await
}
