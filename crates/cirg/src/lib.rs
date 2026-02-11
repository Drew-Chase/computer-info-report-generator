use std::collections::HashMap;
use anyhow::anyhow;
use wmi::Variant;

pub mod audio_info;
pub mod computer_info;
pub mod cpu_info;
pub mod disk_info;
pub mod environment_info;
pub mod event_log_info;
pub mod gpu_info;
pub mod hotfix_info;
pub mod memory_info;
pub mod monitor_info;
pub mod network_info;
pub mod power_info;
pub mod process_info;
pub mod scheduled_task_info;
pub mod security_info;
pub mod service_info;
pub mod software_info;
pub mod startup_info;
pub mod usb_info;
pub mod users_groups_info;

// Helper trait for extracting values from WMI Variant HashMap
pub(crate) trait VariantExt {
	fn get_string(&self, key: &str) -> anyhow::Result<String>;
	fn get_u16(&self, key: &str) -> anyhow::Result<u16>;
	fn get_u32(&self, key: &str) -> anyhow::Result<u32>;
	fn get_u64(&self, key: &str) -> anyhow::Result<u64>;
	fn get_bool(&self, key: &str) -> anyhow::Result<bool>;
}

impl VariantExt for HashMap<String, Variant> {
	fn get_string(&self, key: &str) -> anyhow::Result<String> {
		match self.get(key) {
			Some(Variant::String(s)) => Ok(s.clone()),
			None => Err(anyhow!("Key '{}' not found", key)),
			Some(_) => Err(anyhow!("Value for key '{}' is not a string", key)),
		}
	}

	fn get_u16(&self, key: &str) -> anyhow::Result<u16> {
		match self.get(key) {
			Some(Variant::UI1(v)) => Ok(*v as u16),
			Some(Variant::UI2(v)) => Ok(*v),
			Some(Variant::I2(v)) => Ok(*v as u16),
			None => Err(anyhow!("Key '{}' not found", key)),
			Some(_) => Err(anyhow!("Value for key '{}' is not a u16-compatible type", key)),
		}
	}

	fn get_u32(&self, key: &str) -> anyhow::Result<u32> {
		match self.get(key) {
			Some(Variant::UI1(v)) => Ok(*v as u32),
			Some(Variant::UI2(v)) => Ok(*v as u32),
			Some(Variant::UI4(v)) => Ok(*v),
			Some(Variant::I4(v)) => Ok(*v as u32),
			None => Err(anyhow!("Key '{}' not found", key)),
			Some(_) => Err(anyhow!("Value for key '{}' is not a u32-compatible type", key)),
		}
	}

	fn get_u64(&self, key: &str) -> anyhow::Result<u64> {
		match self.get(key) {
			Some(Variant::UI8(v)) => Ok(*v),
			Some(Variant::UI4(v)) => Ok(*v as u64),
			Some(Variant::UI2(v)) => Ok(*v as u64),
			Some(Variant::UI1(v)) => Ok(*v as u64),
			Some(Variant::I8(v)) => Ok(*v as u64),
			Some(Variant::String(s)) => s.parse::<u64>().map_err(|e| anyhow!("Failed to parse '{}' as u64: {}", s, e)),
			None => Err(anyhow!("Key '{}' not found", key)),
			Some(_) => Err(anyhow!("Value for key '{}' is not a u64-compatible type", key)),
		}
	}

	fn get_bool(&self, key: &str) -> anyhow::Result<bool> {
		match self.get(key) {
			Some(Variant::Bool(b)) => Ok(*b),
			None => Err(anyhow!("Key not found in HashMap")),
			Some(_) => Err(anyhow!("Value for key is not a boolean")),
		}
	}
}

pub trait ComputerInfoExt{
	fn fetch()->anyhow::Result<Self> where Self: Sized;
}
