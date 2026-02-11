use crate::ComputerInfoExt;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wmi::Variant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitorInfo {
	pub monitors: Vec<Monitor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Monitor {
	pub manufacturer: String,
	pub name: String,
	pub serial_number: String,
	pub year_of_manufacture: u16,
}

impl ComputerInfoExt for MonitorInfo {
	fn fetch() -> Result<Self> {
		let com = wmi::WMIConnection::with_namespace_path(r"root\wmi")?;
		let results: Vec<HashMap<String, Variant>> =
			com.raw_query("SELECT * FROM WmiMonitorID")?;

		let monitors = results
			.iter()
			.map(|data| {
				let manufacturer = data
					.get("ManufacturerName")
					.map(decode_wmi_byte_array)
					.unwrap_or_default();
				let name = data
					.get("UserFriendlyName")
					.map(decode_wmi_byte_array)
					.unwrap_or_default();
				let serial = data
					.get("SerialNumberID")
					.map(decode_wmi_byte_array)
					.unwrap_or_default();
				let year = match data.get("YearOfManufacture") {
					Some(Variant::UI2(y)) => *y,
					Some(Variant::I2(y)) => *y as u16,
					_ => 0,
				};

				Monitor {
					manufacturer,
					name,
					serial_number: serial,
					year_of_manufacture: year,
				}
			})
			.collect();

		Ok(MonitorInfo { monitors })
	}
}

fn decode_wmi_byte_array(variant: &Variant) -> String {
	match variant {
		Variant::Array(arr) => {
			let bytes: Vec<u8> = arr
				.iter()
				.filter_map(|v| match v {
					Variant::UI1(b) => Some(*b),
					Variant::UI2(b) => Some(*b as u8),
					Variant::I2(b) => Some(*b as u8),
					_ => None,
				})
				.take_while(|&b| b != 0)
				.collect();
			String::from_utf8_lossy(&bytes).to_string()
		}
		_ => String::new(),
	}
}
