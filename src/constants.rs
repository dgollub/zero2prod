#![allow(dead_code)]

pub const SERVICE_NAME: &str = env!("CARGO_PKG_NAME");
pub const SERVICE_VERSION: &str = env!("CARGO_PKG_VERSION");

// Environment variable default values
pub const DEFAULT_SERVICE_LISTEN_PORT: u16 = 8000;
pub const DEFAULT_SERVICE_LISTEN_INTERFACE: &str = "0.0.0.0";
