use std::env;

use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
	pub url: Option<String>,
	pub max_conn: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct HTTP {
	pub host: String,
	pub port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
	debug: bool,
	pub database: Database,
	pub http: HTTP,
}

impl Settings {
	pub fn new() -> Result<Self, ConfigError> {
		let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

		let s = Config::builder()
			.add_source(File::with_name("default"))
			.add_source(
				File::with_name(&format!("{}", run_mode))
					.required(false),
			)
			.add_source(File::with_name("local").required(false))
			.add_source(Environment::with_prefix("app"))
			.build()?;
		println!("Config built.");
		s.try_deserialize()
	}
}