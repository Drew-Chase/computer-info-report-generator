use crate::{ComputerInfoExt, VariantExt};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wmi::Variant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkInfo {
	pub adapters: Vec<NetworkAdapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkAdapter {
	pub name: String,
	pub description: String,
	pub mac_address: String,
	pub speed: String,
	pub ipv4_addresses: Vec<String>,
	pub ipv6_addresses: Vec<String>,
	pub dns_servers: Vec<String>,
	pub dhcp_enabled: bool,
	pub gateway: String,
}

impl ComputerInfoExt for NetworkInfo {
	fn fetch() -> Result<Self> {
		let com = wmi::WMIConnection::new()?;

		let adapter_results: Vec<HashMap<String, Variant>> = com.raw_query(
			"SELECT Index, Name, Speed, NetConnectionID, MACAddress FROM Win32_NetworkAdapter WHERE NetEnabled=True",
		)?;

		let mut adapter_map: HashMap<u32, (String, u64, String)> = HashMap::new();
		for data in &adapter_results {
			if let Ok(idx) = data.get_u32("Index") {
				let name = data
					.get_string("NetConnectionID")
					.or_else(|_| data.get_string("Name"))
					.unwrap_or_default();
				let speed = data.get_u64("Speed").unwrap_or(0);
				let mac = data.get_string("MACAddress").unwrap_or_default();
				adapter_map.insert(idx, (name, speed, mac));
			}
		}

		let config_results: Vec<HashMap<String, Variant>> = com.raw_query(
			"SELECT * FROM Win32_NetworkAdapterConfiguration WHERE IPEnabled=True",
		)?;

		let adapters = config_results
			.iter()
			.map(|data| {
				let index = data.get_u32("Index").unwrap_or(0);
				let (name, speed_bps, mac) =
					adapter_map.get(&index).cloned().unwrap_or_default();

				let description = data.get_string("Description").unwrap_or_default();

				let ip_addresses = data
					.get("IPAddress")
					.map(extract_string_array)
					.unwrap_or_default();

				let mut ipv4 = Vec::new();
				let mut ipv6 = Vec::new();
				for ip in &ip_addresses {
					if ip.contains(':') {
						ipv6.push(ip.clone());
					} else {
						ipv4.push(ip.clone());
					}
				}

				let dns_servers = data
					.get("DNSServerSearchOrder")
					.map(extract_string_array)
					.unwrap_or_default();

				let gateway = data
					.get("DefaultIPGateway")
					.map(extract_string_array)
					.unwrap_or_default()
					.first()
					.cloned()
					.unwrap_or_default();

				NetworkAdapter {
					name,
					description,
					mac_address: mac,
					speed: format_speed(speed_bps),
					ipv4_addresses: ipv4,
					ipv6_addresses: ipv6,
					dns_servers,
					dhcp_enabled: data.get_bool("DHCPEnabled").unwrap_or(false),
					gateway,
				}
			})
			.collect();

		Ok(NetworkInfo { adapters })
	}
}

fn extract_string_array(variant: &Variant) -> Vec<String> {
	match variant {
		Variant::Array(arr) => arr
			.iter()
			.filter_map(|v| {
				if let Variant::String(s) = v {
					Some(s.clone())
				} else {
					None
				}
			})
			.collect(),
		_ => Vec::new(),
	}
}

fn format_speed(bps: u64) -> String {
	if bps >= 1_000_000_000 {
		format!("{:.1} Gbps", bps as f64 / 1_000_000_000.0)
	} else if bps >= 1_000_000 {
		format!("{:.0} Mbps", bps as f64 / 1_000_000.0)
	} else if bps >= 1_000 {
		format!("{:.0} Kbps", bps as f64 / 1_000.0)
	} else if bps > 0 {
		format!("{} bps", bps)
	} else {
		"N/A".to_string()
	}
}
