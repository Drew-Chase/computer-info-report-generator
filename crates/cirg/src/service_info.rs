use crate::{ComputerInfoExt, VariantExt};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wmi::Variant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceInfo {
	pub services: Vec<Service>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Service {
	pub name: String,
	pub display_name: String,
	pub state: String,
	pub start_mode: String,
	pub account: String,
	pub path: String,
	pub description: String,
}

impl ComputerInfoExt for ServiceInfo {
	fn fetch() -> Result<Self> {
		let com = wmi::WMIConnection::new()?;
		let results: Vec<HashMap<String, Variant>> =
			com.raw_query("SELECT * FROM Win32_Service")?;

		let services = results
			.iter()
			.map(|data| Service {
				name: data.get_string("Name").unwrap_or_default(),
				display_name: data.get_string("DisplayName").unwrap_or_default(),
				state: data.get_string("State").unwrap_or_default(),
				start_mode: data.get_string("StartMode").unwrap_or_default(),
				account: data.get_string("StartName").unwrap_or_default(),
				path: data.get_string("PathName").unwrap_or_default(),
				description: data.get_string("Description").unwrap_or_default(),
			})
			.collect();

		Ok(ServiceInfo { services })
	}
}
