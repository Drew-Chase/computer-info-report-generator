use crate::ComputerInfoExt;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventLogInfo {
	pub system_events: Vec<EventEntry>,
	pub application_events: Vec<EventEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventEntry {
	pub level: String,
	pub source: String,
	pub event_id: String,
	pub time_created: String,
	pub message: String,
}

impl ComputerInfoExt for EventLogInfo {
	fn fetch() -> Result<Self> {
		let now = chrono::Utc::now();
		let yesterday = now - chrono::Duration::hours(24);
		let time_filter = yesterday.format("%Y-%m-%dT%H:%M:%S").to_string();

		Ok(EventLogInfo {
			system_events: query_event_log("System", &time_filter),
			application_events: query_event_log("Application", &time_filter),
		})
	}
}

fn query_event_log(log_name: &str, since: &str) -> Vec<EventEntry> {
	let query = format!(
		"*[System[(Level>=1 and Level<=3) and TimeCreated[@SystemTime>='{}']]]",
		since
	);

	let output = Command::new("wevtutil")
		.args([
			"qe",
			log_name,
			&format!("/q:{}", query),
			"/c:15",
			"/rd:true",
			"/f:xml",
		])
		.output();

	let Ok(output) = output else {
		return Vec::new();
	};
	let xml = String::from_utf8_lossy(&output.stdout);

	parse_event_xml(&xml)
}

fn parse_event_xml(xml: &str) -> Vec<EventEntry> {
	let mut events = Vec::new();

	for event_block in xml.split("<Event xmlns=") {
		if event_block.trim().is_empty() {
			continue;
		}

		let level = extract_xml_value(event_block, "Level")
			.map(|l| match l.as_str() {
				"1" => "Critical".to_string(),
				"2" => "Error".to_string(),
				"3" => "Warning".to_string(),
				_ => l,
			})
			.unwrap_or_default();

		let source = extract_xml_attr(event_block, "Provider", "Name").unwrap_or_default();
		let event_id = extract_xml_value(event_block, "EventID").unwrap_or_default();
		let time_created =
			extract_xml_attr(event_block, "TimeCreated", "SystemTime").unwrap_or_default();
		let message = extract_xml_value(event_block, "Data").unwrap_or_default();

		if !level.is_empty() {
			events.push(EventEntry {
				level,
				source,
				event_id,
				time_created,
				message,
			});
		}
	}

	events
}

fn extract_xml_value(xml: &str, tag: &str) -> Option<String> {
	let open = format!("<{}", tag);
	let close = format!("</{}>", tag);
	let start = xml.find(&open)?;
	let rest = &xml[start..];
	let content_start = rest.find('>')? + 1;
	let content = &rest[content_start..];
	let end = content.find(&close)?;
	let value = content[..end].trim().to_string();
	if value.is_empty() {
		None
	} else {
		Some(value)
	}
}

fn extract_xml_attr(xml: &str, tag: &str, attr: &str) -> Option<String> {
	let open = format!("<{}", tag);
	let start = xml.find(&open)?;
	let rest = &xml[start..];
	let tag_end = rest.find('>')?;
	let tag_content = &rest[..tag_end];

	// Try single quotes
	let attr_pattern = format!("{}='", attr);
	if let Some(attr_start) = tag_content.find(&attr_pattern) {
		let value_start = attr_start + attr_pattern.len();
		let value_rest = &tag_content[value_start..];
		let end = value_rest.find('\'')?;
		return Some(value_rest[..end].to_string());
	}

	// Try double quotes
	let attr_pattern = format!("{}=\"", attr);
	if let Some(attr_start) = tag_content.find(&attr_pattern) {
		let value_start = attr_start + attr_pattern.len();
		let value_rest = &tag_content[value_start..];
		let end = value_rest.find('"')?;
		return Some(value_rest[..end].to_string());
	}

	None
}
