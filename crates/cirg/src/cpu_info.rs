use crate::{ComputerInfoExt, VariantExt};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wmi::Variant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CpuInfo {
	pub name: String,
	pub cores: u32,
	pub logical_processors: u32,
	pub max_clock_mhz: u32,
	pub current_clock_mhz: u32,
	pub socket: String,
	pub l2_cache_kb: u32,
	pub l3_cache_kb: u32,
	pub architecture: String,
	pub virtualization: bool,
	pub status: String,
	pub load_pct: u16,
}

impl ComputerInfoExt for CpuInfo {
	fn fetch() -> Result<Self> {
		let com = wmi::WMIConnection::new()?;
		let results: Vec<HashMap<String, Variant>> =
			com.raw_query("SELECT * FROM Win32_Processor")?;
		let data = results
			.first()
			.ok_or_else(|| anyhow!("No CPU info found"))?;

		let arch_code = data.get_u16("Architecture").unwrap_or(9);
		let architecture = match arch_code {
			0 => "x86",
			5 => "ARM",
			6 => "ia64",
			9 => "x64",
			12 => "ARM64",
			_ => "Unknown",
		}
		.to_string();

		Ok(CpuInfo {
			name: data.get_string("Name").unwrap_or_default(),
			cores: data.get_u32("NumberOfCores").unwrap_or(0),
			logical_processors: data.get_u32("NumberOfLogicalProcessors").unwrap_or(0),
			max_clock_mhz: data.get_u32("MaxClockSpeed").unwrap_or(0),
			current_clock_mhz: data.get_u32("CurrentClockSpeed").unwrap_or(0),
			socket: data.get_string("SocketDesignation").unwrap_or_default(),
			l2_cache_kb: data.get_u32("L2CacheSize").unwrap_or(0),
			l3_cache_kb: data.get_u32("L3CacheSize").unwrap_or(0),
			architecture,
			virtualization: data
				.get_bool("VirtualizationFirmwareEnabled")
				.unwrap_or(false),
			status: data.get_string("Status").unwrap_or_default(),
			load_pct: data.get_u16("LoadPercentage").unwrap_or(0),
		})
	}
}
