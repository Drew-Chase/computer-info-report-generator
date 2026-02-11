use crate::{ComputerInfoExt, VariantExt};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wmi::Variant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StartupInfo {
	pub items: Vec<StartupItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StartupItem {
	pub name: String,
	pub command: String,
	pub location: String,
	pub user: String,
}

impl ComputerInfoExt for StartupInfo {
	fn fetch() -> Result<Self> {
		let com = wmi::WMIConnection::new()?;
		let results: Vec<HashMap<String, Variant>> =
			com.raw_query("SELECT * FROM Win32_StartupCommand")?;

		let items = results
			.iter()
			.map(|data| StartupItem {
				name: data.get_string("Name").unwrap_or_default(),
				command: data.get_string("Command").unwrap_or_default(),
				location: data.get_string("Location").unwrap_or_default(),
				user: data.get_string("User").unwrap_or_default(),
			})
			.collect();

		Ok(StartupInfo { items })
	}
}
