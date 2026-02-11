use crate::{ComputerInfoExt, VariantExt};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wmi::Variant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryInfo {
	pub slots: Vec<MemorySlot>,
	pub total_slots: u32,
	pub max_capacity_gb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemorySlot {
	pub bank_label: String,
	pub capacity_gb: f64,
	pub speed_mhz: u32,
	pub memory_type: String,
	pub form_factor: String,
	pub manufacturer: String,
	pub part_number: String,
}

impl ComputerInfoExt for MemoryInfo {
	fn fetch() -> Result<Self> {
		let com = wmi::WMIConnection::new()?;

		let array_results: Vec<HashMap<String, Variant>> =
			com.raw_query("SELECT * FROM Win32_PhysicalMemoryArray")?;
		let array_data = array_results.first();

		let total_slots = array_data
			.and_then(|d| d.get_u32("MemoryDevices").ok())
			.unwrap_or(0);
		let max_capacity_kb = array_data
			.and_then(|d| d.get_u64("MaxCapacity").ok())
			.unwrap_or(0);
		let max_capacity_gb = max_capacity_kb / (1024 * 1024);

		let mem_results: Vec<HashMap<String, Variant>> =
			com.raw_query("SELECT * FROM Win32_PhysicalMemory")?;

		let slots = mem_results
			.iter()
			.map(|data| {
				let capacity_bytes = data.get_u64("Capacity").unwrap_or(0);
				let capacity_gb = capacity_bytes as f64 / (1024.0 * 1024.0 * 1024.0);

				let type_code = data.get_u16("SMBIOSMemoryType").unwrap_or(0);
				let memory_type = match type_code {
					20 => "DDR",
					21 => "DDR2",
					24 => "DDR3",
					26 => "DDR4",
					34 => "DDR5",
					_ => "Unknown",
				}
				.to_string();

				let ff_code = data.get_u16("FormFactor").unwrap_or(0);
				let form_factor = match ff_code {
					8 => "DIMM",
					12 => "SO-DIMM",
					_ => "Unknown",
				}
				.to_string();

				MemorySlot {
					bank_label: data.get_string("BankLabel").unwrap_or_default(),
					capacity_gb,
					speed_mhz: data.get_u32("Speed").unwrap_or(0),
					memory_type,
					form_factor,
					manufacturer: data.get_string("Manufacturer").unwrap_or_default(),
					part_number: data
						.get_string("PartNumber")
						.unwrap_or_default()
						.trim()
						.to_string(),
				}
			})
			.collect();

		Ok(MemoryInfo {
			slots,
			total_slots,
			max_capacity_gb,
		})
	}
}
