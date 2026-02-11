use crate::{ComputerInfoExt, VariantExt};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;
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

		// Build registry-based VRAM lookup: gpu_name -> vram_bytes
		let registry_vram = Self::fetch_registry_vram();

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

				let name = data.get_string("Name").unwrap_or_default();

				// Try registry VRAM first (accurate for >4GB), fall back to WMI AdapterRAM
				let vram_bytes = registry_vram
					.get(&name)
					.copied()
					.unwrap_or_else(|| data.get_u64("AdapterRAM").unwrap_or(0));

				GpuAdapter {
					name,
					driver_version: data.get_string("DriverVersion").unwrap_or_default(),
					driver_date: data
						.get_string("DriverDate")
						.map(|d| {
							if d.len() >= 8 {
								format!("{}-{}-{}", &d[..4], &d[4..6], &d[6..8])
							} else {
								d
							}
						})
						.unwrap_or_default(),
					adapter_ram_mb: vram_bytes / (1024 * 1024),
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

impl GpuInfo {
	/// Read VRAM from the registry where `HardwareInformation.qwMemorySize` is a REG_QWORD.
	/// This avoids the WMI uint32 cap (~4GB) on AdapterRAM.
	fn fetch_registry_vram() -> HashMap<String, u64> {
		let mut map = HashMap::new();
		let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
		let Ok(class_key) = hklm.open_subkey(
			r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}",
		) else {
			return map;
		};

		for subkey_name in class_key.enum_keys().flatten() {
			let Ok(subkey) = class_key.open_subkey(&subkey_name) else {
				continue;
			};
			let Ok(desc): Result<String, _> = subkey.get_value("DriverDesc") else {
				continue;
			};
			if let Ok(vram) = subkey.get_value::<u64, _>("HardwareInformation.qwMemorySize") {
				if vram > 0 {
					map.insert(desc, vram);
				}
			}
		}

		map
	}
}
