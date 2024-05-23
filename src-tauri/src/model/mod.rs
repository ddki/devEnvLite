use anyhow::Result;

pub mod config;
pub mod env;
pub mod group;
pub mod settings;

pub trait InfoEvent {
	fn apply(&self) -> Result<()>;
	fn check(&self) -> Result<()>;
}
