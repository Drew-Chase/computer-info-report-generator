use crate::ComputerInfoExt;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use winreg::enums::*;
use winreg::RegKey;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentInfo {
	pub variables: BTreeMap<String, String>,
}

impl ComputerInfoExt for EnvironmentInfo {
	fn fetch() -> Result<Self> {
		let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
		let key = hklm.open_subkey(
			r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment",
		)?;

		let mut variables = BTreeMap::new();
		for result in key.enum_values() {
			let (name, _) = result?;
			let value: String = key.get_value(&name).unwrap_or_default();
			variables.insert(name, value);
		}

		Ok(EnvironmentInfo { variables })
	}
}
