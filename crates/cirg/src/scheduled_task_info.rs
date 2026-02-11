use crate::ComputerInfoExt;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScheduledTaskInfo {
	pub tasks: Vec<ScheduledTask>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScheduledTask {
	pub name: String,
	pub path: String,
	pub state: String,
	pub last_run: String,
	pub next_run: String,
	pub result: String,
	pub author: String,
}

impl ComputerInfoExt for ScheduledTaskInfo {
	fn fetch() -> Result<Self> {
		let output = Command::new("schtasks")
			.args(["/Query", "/FO", "CSV", "/V"])
			.output()?;

		let stdout = String::from_utf8_lossy(&output.stdout);
		let mut lines = stdout.lines();

		let header = match lines.next() {
			Some(h) => h,
			None => return Ok(ScheduledTaskInfo::default()),
		};

		let headers = parse_csv_line(header);

		let idx = |name: &str| headers.iter().position(|h| h.contains(name));
		let task_name_idx = idx("TaskName").unwrap_or(0);
		let next_run_idx = idx("Next Run Time");
		let status_idx = idx("Status");
		let last_run_idx = idx("Last Run Time");
		let last_result_idx = idx("Last Result");
		let author_idx = idx("Author");

		let mut tasks = Vec::new();

		for line in lines {
			let cols = parse_csv_line(line);
			if cols.len() <= task_name_idx {
				continue;
			}

			let path = cols[task_name_idx].clone();

			if path.starts_with("\\Microsoft\\") {
				continue;
			}

			let name = path
				.rsplit('\\')
				.next()
				.unwrap_or(&path)
				.to_string();

			tasks.push(ScheduledTask {
				name,
				path,
				state: status_idx
					.and_then(|i| cols.get(i))
					.cloned()
					.unwrap_or_default(),
				last_run: last_run_idx
					.and_then(|i| cols.get(i))
					.cloned()
					.unwrap_or_default(),
				next_run: next_run_idx
					.and_then(|i| cols.get(i))
					.cloned()
					.unwrap_or_default(),
				result: last_result_idx
					.and_then(|i| cols.get(i))
					.cloned()
					.unwrap_or_default(),
				author: author_idx
					.and_then(|i| cols.get(i))
					.cloned()
					.unwrap_or_default(),
			});
		}

		Ok(ScheduledTaskInfo { tasks })
	}
}

fn parse_csv_line(line: &str) -> Vec<String> {
	let mut fields = Vec::new();
	let mut current = String::new();
	let mut in_quotes = false;
	let mut chars = line.chars().peekable();

	while let Some(c) = chars.next() {
		match c {
			'"' if !in_quotes => in_quotes = true,
			'"' if in_quotes => {
				if chars.peek() == Some(&'"') {
					chars.next();
					current.push('"');
				} else {
					in_quotes = false;
				}
			}
			',' if !in_quotes => {
				fields.push(current.clone());
				current.clear();
			}
			_ => current.push(c),
		}
	}
	fields.push(current);
	fields
}
