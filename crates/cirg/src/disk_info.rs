use crate::{ComputerInfoExt, VariantExt};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wmi::Variant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiskInfo {
	pub physical_disks: Vec<PhysicalDisk>,
	pub logical_disks: Vec<LogicalDisk>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PhysicalDisk {
	pub model: String,
	pub interface_type: String,
	pub media_type: String,
	pub size_gb: f64,
	pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LogicalDisk {
	pub device_id: String,
	pub volume_name: String,
	pub file_system: String,
	pub total_gb: f64,
	pub free_gb: f64,
	pub used_gb: f64,
	pub usage_pct: f64,
}

impl ComputerInfoExt for DiskInfo {
	fn fetch() -> Result<Self> {
		let com = wmi::WMIConnection::new()?;

		let phys_results: Vec<HashMap<String, Variant>> =
			com.raw_query("SELECT * FROM Win32_DiskDrive")?;
		let physical_disks = phys_results
			.iter()
			.map(|data| {
				let size_bytes = data.get_u64("Size").unwrap_or(0);
				PhysicalDisk {
					model: data.get_string("Model").unwrap_or_default(),
					interface_type: data.get_string("InterfaceType").unwrap_or_default(),
					media_type: data.get_string("MediaType").unwrap_or_default(),
					size_gb: size_bytes as f64 / (1024.0 * 1024.0 * 1024.0),
					status: data.get_string("Status").unwrap_or_default(),
				}
			})
			.collect();

		let log_results: Vec<HashMap<String, Variant>> =
			com.raw_query("SELECT * FROM Win32_LogicalDisk WHERE DriveType=3")?;
		let logical_disks = log_results
			.iter()
			.map(|data| {
				let total = data.get_u64("Size").unwrap_or(0) as f64;
				let free = data.get_u64("FreeSpace").unwrap_or(0) as f64;
				let used = total - free;
				let total_gb = total / (1024.0 * 1024.0 * 1024.0);
				let free_gb = free / (1024.0 * 1024.0 * 1024.0);
				let used_gb = used / (1024.0 * 1024.0 * 1024.0);
				let usage_pct = if total > 0.0 {
					(used / total) * 100.0
				} else {
					0.0
				};

				LogicalDisk {
					device_id: data.get_string("DeviceID").unwrap_or_default(),
					volume_name: data.get_string("VolumeName").unwrap_or_default(),
					file_system: data.get_string("FileSystem").unwrap_or_default(),
					total_gb,
					free_gb,
					used_gb,
					usage_pct,
				}
			})
			.collect();

		Ok(DiskInfo {
			physical_disks,
			logical_disks,
		})
	}
}
