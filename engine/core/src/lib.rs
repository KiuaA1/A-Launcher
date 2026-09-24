pub mod artifacts;
pub mod cache;
pub mod download;
pub mod error;
pub mod fs;
pub mod classpath;
pub mod native;
pub mod instance;
pub mod launch;
pub mod manifest;
pub mod mojang;
pub mod plan;
pub mod process;
pub mod resolver;
pub mod runtime;

pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");
