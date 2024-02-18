//! All built-in configuration sources

#[cfg(feature = "clap")]
pub mod cli;
pub mod env;
#[cfg(any(feature = "yaml", feature = "json", feature = "toml"))]
pub mod file;
pub mod manual;

mod utils;

#[cfg(feature = "clap")]
pub use cli::CliSource;
pub use env::EnvSource;
#[cfg(any(feature = "yaml", feature = "json", feature = "toml"))]
pub use file::FileSource;
pub use manual::ManualSource;
