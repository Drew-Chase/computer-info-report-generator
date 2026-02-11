use crate::{ComputerInfoExt, VariantExt};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wmi::Variant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioInfo {
	pub devices: Vec<AudioDevice>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioDevice {
	pub name: String,
	pub manufacturer: String,
	pub status: String,
	pub device_id: String,
}

impl ComputerInfoExt for AudioInfo {
	fn fetch() -> Result<Self> {
		let com = wmi::WMIConnection::new()?;
		let results: Vec<HashMap<String, Variant>> =
			com.raw_query("SELECT * FROM Win32_SoundDevice")?;

		let devices = results
			.iter()
			.map(|data| AudioDevice {
				name: data.get_string("Name").unwrap_or_default(),
				manufacturer: data.get_string("Manufacturer").unwrap_or_default(),
				status: data.get_string("Status").unwrap_or_default(),
				device_id: data.get_string("DeviceID").unwrap_or_default(),
			})
			.collect();

		Ok(AudioInfo { devices })
	}
}
