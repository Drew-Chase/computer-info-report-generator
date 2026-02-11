use crate::ComputerInfoExt;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use winreg::enums::*;
use winreg::RegKey;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SoftwareInfo {
	pub programs: Vec<InstalledProgram>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstalledProgram {
	pub name: String,
	pub version: String,
	pub publisher: String,
	pub install_date: String,
}

impl ComputerInfoExt for SoftwareInfo {
	fn fetch() -> Result<Self> {
		let paths = [
			(
				HKEY_LOCAL_MACHINE,
				r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
			),
			(
				HKEY_LOCAL_MACHINE,
				r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
			),
			(
				HKEY_CURRENT_USER,
				r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
			),
		];

		let mut seen = HashSet::new();
		let mut programs = Vec::new();

		for (hive, path) in &paths {
			let root = RegKey::predef(*hive);
			let Ok(key) = root.open_subkey(path) else {
				continue;
			};

			for subkey_name in key.enum_keys().filter_map(|k| k.ok()) {
				let Ok(subkey) = key.open_subkey(&subkey_name) else {
					continue;
				};

				let name: String = match subkey.get_value("DisplayName") {
					Ok(n) => n,
					Err(_) => continue,
				};

				let sys_component: u32 = subkey.get_value("SystemComponent").unwrap_or(0);
				if sys_component == 1 {
					continue;
				}
				let parent: String = subkey.get_value("ParentKeyName").unwrap_or_default();
				if !parent.is_empty() {
					continue;
				}

				if !seen.insert(name.clone()) {
					continue;
				}

				programs.push(InstalledProgram {
					name,
					version: subkey.get_value("DisplayVersion").unwrap_or_default(),
					publisher: subkey.get_value("Publisher").unwrap_or_default(),
					install_date: subkey.get_value("InstallDate").unwrap_or_default(),
				});
			}
		}

		programs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

		Ok(SoftwareInfo { programs })
	}
}
