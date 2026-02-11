use std::time::Instant;
use cirg::{
	audio_info::AudioInfo,
	computer_info::ComputerInfo,
	cpu_info::CpuInfo,
	disk_info::DiskInfo,
	environment_info::EnvironmentInfo,
	event_log_info::EventLogInfo,
	gpu_info::GpuInfo,
	hotfix_info::HotfixInfo,
	memory_info::MemoryInfo,
	monitor_info::MonitorInfo,
	network_info::NetworkInfo,
	power_info::PowerInfo,
	process_info::ProcessInfo,
	scheduled_task_info::ScheduledTaskInfo,
	security_info::SecurityInfo,
	service_info::ServiceInfo,
	software_info::SoftwareInfo,
	startup_info::StartupInfo,
	usb_info::UsbInfo,
	users_groups_info::UsersGroupsInfo,
	ComputerInfoExt,
};
use system_pause::pause;

#[tokio::main]
async fn main() {
	let stopwatch = Instant::now();
	let (
		computer,
		power,
		security,
		cpu,
		gpu,
		audio,
		startup,
		hotfix,
		service,
		memory,
		disk,
		network,
		monitor,
		usb,
		software,
		environment,
		process,
		users_groups,
		event_log,
		scheduled_task,
	) = tokio::join!(
		tokio::task::spawn_blocking(ComputerInfo::fetch),
		tokio::task::spawn_blocking(PowerInfo::fetch),
		tokio::task::spawn_blocking(SecurityInfo::fetch),
		tokio::task::spawn_blocking(CpuInfo::fetch),
		tokio::task::spawn_blocking(GpuInfo::fetch),
		tokio::task::spawn_blocking(AudioInfo::fetch),
		tokio::task::spawn_blocking(StartupInfo::fetch),
		tokio::task::spawn_blocking(HotfixInfo::fetch),
		tokio::task::spawn_blocking(ServiceInfo::fetch),
		tokio::task::spawn_blocking(MemoryInfo::fetch),
		tokio::task::spawn_blocking(DiskInfo::fetch),
		tokio::task::spawn_blocking(NetworkInfo::fetch),
		tokio::task::spawn_blocking(MonitorInfo::fetch),
		tokio::task::spawn_blocking(UsbInfo::fetch),
		tokio::task::spawn_blocking(SoftwareInfo::fetch),
		tokio::task::spawn_blocking(EnvironmentInfo::fetch),
		tokio::task::spawn_blocking(ProcessInfo::fetch),
		tokio::task::spawn_blocking(UsersGroupsInfo::fetch),
		tokio::task::spawn_blocking(EventLogInfo::fetch),
		tokio::task::spawn_blocking(ScheduledTaskInfo::fetch),
	);

	let sections: Vec<(&str, String)> = vec![
		("Computer", fmt(&computer)),
		("CPU", fmt(&cpu)),
		("GPU", fmt(&gpu)),
		("Memory", fmt(&memory)),
		("Disk", fmt(&disk)),
		("Network", fmt(&network)),
		("Monitor", fmt(&monitor)),
		("Audio", fmt(&audio)),
		("USB", fmt(&usb)),
		("Power", fmt(&power)),
		("Security", fmt(&security)),
		("Process (Top 30)", fmt(&process)),
		("Services", fmt(&service)),
		("Startup", fmt(&startup)),
		("Software", fmt(&software)),
		("Hotfixes", fmt(&hotfix)),
		("Users & Groups", fmt(&users_groups)),
		("Environment", fmt(&environment)),
		("Event Log", fmt(&event_log)),
		("Scheduled Tasks", fmt(&scheduled_task)),
	];

	for (name, json) in sections {
		println!("{}: {}", name, json);
	}


	println!("Finished after {:?}", stopwatch.elapsed());

	pause!();
}

fn fmt<T: serde::Serialize>(
	result: &Result<anyhow::Result<T>, tokio::task::JoinError>,
) -> String {
	match result {
		Ok(Ok(data)) => serde_json::to_string_pretty(data).unwrap_or_else(|e| format!("Serialization error: {}", e)),
		Ok(Err(e)) => format!("\"Error: {}\"", e),
		Err(e) => format!("\"Task panicked: {}\"", e),
	}
}
