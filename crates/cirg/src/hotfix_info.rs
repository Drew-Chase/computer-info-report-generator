use crate::{ComputerInfoExt, VariantExt};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wmi::Variant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HotfixInfo {
	pub hotfixes: Vec<Hotfix>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Hotfix {
	pub hotfix_id: String,
	pub description: String,
	pub installed_by: String,
	pub installed_on: String,
}

impl ComputerInfoExt for HotfixInfo {
	fn fetch() -> Result<Self> {
		let com = wmi::WMIConnection::new()?;
		let results: Vec<HashMap<String, Variant>> =
			com.raw_query("SELECT * FROM Win32_QuickFixEngineering")?;

		let hotfixes = results
			.iter()
			.take(25)
			.map(|data| Hotfix {
				hotfix_id: data.get_string("HotFixID").unwrap_or_default(),
				description: data.get_string("Description").unwrap_or_default(),
				installed_by: data.get_string("InstalledBy").unwrap_or_default(),
				installed_on: data.get_string("InstalledOn").unwrap_or_default(),
			})
			.collect();

		Ok(HotfixInfo { hotfixes })
	}
}
