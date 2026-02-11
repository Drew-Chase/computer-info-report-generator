use crate::{ComputerInfoExt, VariantExt};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wmi::Variant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsersGroupsInfo {
	pub users: Vec<LocalUser>,
	pub groups: Vec<LocalGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalUser {
	pub name: String,
	pub disabled: bool,
	pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalGroup {
	pub name: String,
	pub description: String,
	pub members: Vec<String>,
}

impl ComputerInfoExt for UsersGroupsInfo {
	fn fetch() -> Result<Self> {
		let com = wmi::WMIConnection::new()?;

		let user_results: Vec<HashMap<String, Variant>> = com.raw_query(
			"SELECT Name, Disabled, Description FROM Win32_UserAccount WHERE LocalAccount=True",
		)?;

		let users = user_results
			.iter()
			.map(|data| LocalUser {
				name: data.get_string("Name").unwrap_or_default(),
				disabled: data.get_bool("Disabled").unwrap_or(false),
				description: data.get_string("Description").unwrap_or_default(),
			})
			.collect();

		let group_results: Vec<HashMap<String, Variant>> = com.raw_query(
			"SELECT Name, Description FROM Win32_Group WHERE LocalAccount=True",
		)?;

		let member_results: Vec<HashMap<String, Variant>> = com
			.raw_query("SELECT GroupComponent, PartComponent FROM Win32_GroupUser")
			.unwrap_or_default();

		let mut group_members: HashMap<String, Vec<String>> = HashMap::new();
		for data in &member_results {
			if let (Ok(group_ref), Ok(part_ref)) = (
				data.get_string("GroupComponent"),
				data.get_string("PartComponent"),
			) {
				let group_name = extract_name(&group_ref);
				let member_name = extract_name(&part_ref);
				if !group_name.is_empty() && !member_name.is_empty() {
					group_members
						.entry(group_name)
						.or_default()
						.push(member_name);
				}
			}
		}

		let groups = group_results
			.iter()
			.map(|data| {
				let name = data.get_string("Name").unwrap_or_default();
				let members = group_members.get(&name).cloned().unwrap_or_default();
				LocalGroup {
					name,
					description: data.get_string("Description").unwrap_or_default(),
					members,
				}
			})
			.collect();

		Ok(UsersGroupsInfo { users, groups })
	}
}

fn extract_name(wmi_path: &str) -> String {
	if let Some(start) = wmi_path.rfind("Name=\"") {
		let rest = &wmi_path[start + 6..];
		if let Some(end) = rest.find('"') {
			return rest[..end].to_string();
		}
	}
	String::new()
}
