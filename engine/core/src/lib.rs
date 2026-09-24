pub mod cache;
pub mod error;
pub mod fs;
pub mod launch;
pub mod manifest;
pub mod mojang;
pub mod resolver;

pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");
