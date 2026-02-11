use crate::{ComputerInfoExt, VariantExt};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wmi::Variant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GpuInfo {
	pub adapters: Vec<GpuAdapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GpuAdapter {
	pub name: String,
	pub driver_version: String,
	pub driver_date: String,
	pub adapter_ram_mb: u64,
	pub resolution: String,
	pub refresh_rate: u32,
	pub status: String,
	pub availability: String,
}

impl ComputerInfoExt for GpuInfo {
	fn fetch() -> Result<Self> {
		let com = wmi::WMIConnection::new()?;
		let results: Vec<HashMap<String, Variant>> =
			com.raw_query("SELECT * FROM Win32_VideoController")?;

		let adapters = results
			.iter()
			.map(|data| {
				let avail_code = data.get_u16("Availability").unwrap_or(0);
				let availability = match avail_code {
					2 => "Unknown",
					3 => "Running/Full Power",
					4 => "Warning",
					5 => "In Test",
					8 => "Off Line",
					_ => "Other",
				}
				.to_string();

				let h_res = data.get_u32("CurrentHorizontalResolution").unwrap_or(0);
				let v_res = data.get_u32("CurrentVerticalResolution").unwrap_or(0);
				let resolution = if h_res > 0 && v_res > 0 {
					format!("{}x{}", h_res, v_res)
				} else {
					"N/A".to_string()
				};

				let adapter_ram = data.get_u32("AdapterRAM").unwrap_or(0) as u64;

				GpuAdapter {
					name: data.get_string("Name").unwrap_or_default(),
					driver_version: data.get_string("DriverVersion").unwrap_or_default(),
					driver_date: data
						.get_string("DriverDate")
						.map(|d| if d.len() >= 8 { d[..8].to_string() } else { d })
						.unwrap_or_default(),
					adapter_ram_mb: adapter_ram / (1024 * 1024),
					resolution,
					refresh_rate: data.get_u32("CurrentRefreshRate").unwrap_or(0),
					status: data.get_string("Status").unwrap_or_default(),
					availability,
				}
			})
			.collect();

		Ok(GpuInfo { adapters })
	}
}
