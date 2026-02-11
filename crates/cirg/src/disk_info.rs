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
	pub disk_type: String,
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

		// Query MSFT_PhysicalDisk for disk type (SSD/HDD), keyed by serial and device id
		let (serial_map, devid_map) = Self::fetch_msft_disk_types();

		let phys_results: Vec<HashMap<String, Variant>> =
			com.raw_query("SELECT * FROM Win32_DiskDrive")?;
		let physical_disks = phys_results
			.iter()
			.map(|data| {
				let size_bytes = data.get_u64("Size").unwrap_or(0);
				let model = data.get_string("Model").unwrap_or_default();
				let serial = data
					.get_string("SerialNumber")
					.unwrap_or_default()
					.trim()
					.to_string();
				let index = data.get_u32("Index").unwrap_or(u32::MAX);

				// 1. Match by serial number
				// 2. Fallback: match by device id (MSFT DeviceId "0","1" == Win32 Index 0,1)
				// 3. Fallback: heuristic on model name
				let disk_type = serial_map
					.get(&serial)
					.or_else(|| devid_map.get(&index.to_string()))
					.cloned()
					.unwrap_or_else(|| Self::guess_disk_type(&model));

				PhysicalDisk {
					model,
					interface_type: data.get_string("InterfaceType").unwrap_or_default(),
					media_type: data.get_string("MediaType").unwrap_or_default(),
					disk_type,
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

impl DiskInfo {
	/// Returns (serial_number -> disk_type, device_id -> disk_type) maps.
	/// On failure, returns empty maps so callers fall back to heuristics.
	fn fetch_msft_disk_types() -> (HashMap<String, String>, HashMap<String, String>) {
		let Ok(com) =
			wmi::WMIConnection::with_namespace_path(r"root\Microsoft\Windows\Storage")
		else {
			return (HashMap::new(), HashMap::new());
		};
		let Ok(results): Result<Vec<HashMap<String, Variant>>, _> = com
			.raw_query("SELECT MediaType, SerialNumber, DeviceId FROM MSFT_PhysicalDisk")
		else {
			return (HashMap::new(), HashMap::new());
		};

		let mut serial_map = HashMap::new();
		let mut devid_map = HashMap::new();

		for data in &results {
			let media_type = data.get_u16("MediaType").unwrap_or(0);
			let disk_type = match media_type {
				3 => "HDD",
				4 => "SSD",
				5 => "SCM",
				_ => "Unknown",
			}
			.to_string();

			if let Ok(serial) = data.get_string("SerialNumber") {
				let serial = serial.trim().to_string();
				if !serial.is_empty() {
					serial_map.insert(serial, disk_type.clone());
				}
			}
			if let Ok(dev_id) = data.get_string("DeviceId") {
				devid_map.insert(dev_id.trim().to_string(), disk_type);
			}
		}

		(serial_map, devid_map)
	}

	fn guess_disk_type(model: &str) -> String {
		let upper = model.to_uppercase();
		if upper.contains("SSD") || upper.contains("NVME") || upper.contains("NVM") {
			"SSD".to_string()
		} else {
			"Unknown".to_string()
		}
	}
}
