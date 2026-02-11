use crate::ComputerInfoExt;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessInfo {
	pub processes: Vec<ProcessEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessEntry {
	pub name: String,
	pub pid: u32,
	pub cpu_seconds: u64,
	pub memory_mb: f64,
	pub exe_path: String,
	pub command: String,
}

impl ComputerInfoExt for ProcessInfo {
	fn fetch() -> Result<Self> {
		let mut sys = System::new_all();
		sys.refresh_all();

		let mut processes: Vec<ProcessEntry> = sys
			.processes()
			.values()
			.map(|p| ProcessEntry {
				name: p.name().to_string_lossy().to_string(),
				pid: p.pid().as_u32(),
				cpu_seconds: p.run_time(),
				memory_mb: p.memory() as f64 / (1024.0 * 1024.0),
				exe_path: p
					.exe()
					.map(|e| e.to_string_lossy().to_string())
					.unwrap_or_default(),
				command: p
					.cmd()
					.iter()
					.map(|s| s.to_string_lossy())
					.collect::<Vec<_>>()
					.join(" "),
			})
			.collect();

		processes.sort_by(|a, b| {
			b.memory_mb
				.partial_cmp(&a.memory_mb)
				.unwrap_or(std::cmp::Ordering::Equal)
		});
		processes.truncate(30);

		Ok(ProcessInfo { processes })
	}
}
