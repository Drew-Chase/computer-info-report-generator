use crate::{ComputerInfoExt, VariantExt};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wmi::Variant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsbInfo {
	pub devices: Vec<UsbDevice>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsbDevice {
	pub name: String,
	pub device_id: String,
	pub manufacturer: String,
	pub status: String,
}

impl ComputerInfoExt for UsbInfo {
	fn fetch() -> Result<Self> {
		let com = wmi::WMIConnection::new()?;
		let results: Vec<HashMap<String, Variant>> = com.raw_query(
			"SELECT Name, PNPDeviceID, Manufacturer, Status FROM Win32_PnPEntity WHERE PNPDeviceID LIKE 'USB%'",
		)?;

		let devices = results
			.iter()
			.filter(|data| {
				let name = data.get_string("Name").unwrap_or_default();
				!name.contains("Root Hub")
					&& !name.contains("Generic Hub")
					&& !name.contains("USB Composite Device")
			})
			.map(|data| UsbDevice {
				name: data.get_string("Name").unwrap_or_default(),
				device_id: data.get_string("PNPDeviceID").unwrap_or_default(),
				manufacturer: data.get_string("Manufacturer").unwrap_or_default(),
				status: data.get_string("Status").unwrap_or_default(),
			})
			.collect();

		Ok(UsbInfo { devices })
	}
}
