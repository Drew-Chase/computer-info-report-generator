use crate::ComputerInfoExt;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use windows::Win32::Graphics::Gdi::*;
use windows::core::PCWSTR;
use wmi::Variant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitorInfo {
	pub monitors: Vec<Monitor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Monitor {
	pub manufacturer: String,
	pub name: String,
	pub serial_number: String,
	pub year_of_manufacture: u16,
	pub resolution: String,
	pub refresh_rate: u32,
}

impl ComputerInfoExt for MonitorInfo {
	fn fetch() -> Result<Self> {
		let com_wmi = wmi::WMIConnection::with_namespace_path(r"root\wmi")?;
		let results: Vec<HashMap<String, Variant>> =
			com_wmi.raw_query("SELECT * FROM WmiMonitorID")?;

		// Use Win32 API for resolution/refresh rate (per-monitor, not per-GPU)
		let display_modes = fetch_display_modes();

		let monitors = results
			.iter()
			.enumerate()
			.map(|(idx, data)| {
				let manufacturer = data
					.get("ManufacturerName")
					.map(decode_wmi_byte_array)
					.unwrap_or_default();
				let name = data
					.get("UserFriendlyName")
					.map(decode_wmi_byte_array)
					.unwrap_or_default();
				let serial = data
					.get("SerialNumberID")
					.map(decode_wmi_byte_array)
					.unwrap_or_default();
				let year = match data.get("YearOfManufacture") {
					Some(Variant::UI2(y)) => *y,
					Some(Variant::I2(y)) => *y as u16,
					_ => 0,
				};

				let (resolution, refresh_rate) = display_modes
					.get(idx)
					.cloned()
					.unwrap_or_else(|| ("N/A".to_string(), 0));

				Monitor {
					manufacturer,
					name,
					serial_number: serial,
					year_of_manufacture: year,
					resolution,
					refresh_rate,
				}
			})
			.collect();

		Ok(MonitorInfo { monitors })
	}
}

/// Enumerate active displays using Win32 `EnumDisplayDevicesW` / `EnumDisplaySettingsW`.
/// Returns a Vec of (resolution_string, refresh_rate_hz) in OS display order.
fn fetch_display_modes() -> Vec<(String, u32)> {
	let mut displays = Vec::new();
	let mut dev_num = 0u32;

	loop {
		let mut device = DISPLAY_DEVICEW {
			cb: std::mem::size_of::<DISPLAY_DEVICEW>() as u32,
			..Default::default()
		};

		let ok =
			unsafe { EnumDisplayDevicesW(PCWSTR::null(), dev_num, &mut device, 0) };
		if !ok.as_bool() {
			break;
		}

		if (device.StateFlags & DISPLAY_DEVICE_ACTIVE).0 != 0 {
			let mut devmode = DEVMODEW {
				dmSize: std::mem::size_of::<DEVMODEW>() as u16,
				..Default::default()
			};

			let ok = unsafe {
				EnumDisplaySettingsW(
					PCWSTR(device.DeviceName.as_ptr()),
					ENUM_CURRENT_SETTINGS,
					&mut devmode,
				)
			};
			if ok.as_bool() {
				let w = devmode.dmPelsWidth;
				let h = devmode.dmPelsHeight;
				let hz = devmode.dmDisplayFrequency;
				let res = if w > 0 && h > 0 {
					format!("{}x{}", w, h)
				} else {
					"N/A".to_string()
				};
				displays.push((res, hz));
			}
		}

		dev_num += 1;
	}

	displays
}

fn decode_wmi_byte_array(variant: &Variant) -> String {
	match variant {
		Variant::Array(arr) => {
			let bytes: Vec<u8> = arr
				.iter()
				.filter_map(|v| match v {
					Variant::UI1(b) => Some(*b),
					Variant::UI2(b) => Some(*b as u8),
					Variant::I2(b) => Some(*b as u8),
					_ => None,
				})
				.take_while(|&b| b != 0)
				.collect();
			String::from_utf8_lossy(&bytes).to_string()
		}
		_ => String::new(),
	}
}
