use cirg::ComputerInfoExt;
use cirg::audio_info::AudioInfo;
use cirg::computer_info::ComputerInfo;
use cirg::cpu_info::CpuInfo;
use cirg::disk_info::DiskInfo;
use cirg::environment_info::EnvironmentInfo;
use cirg::event_log_info::EventLogInfo;
use cirg::gpu_info::GpuInfo;
use cirg::hotfix_info::HotfixInfo;
use cirg::memory_info::MemoryInfo;
use cirg::monitor_info::MonitorInfo;
use cirg::network_info::NetworkInfo;
use cirg::power_info::PowerInfo;
use cirg::process_info::ProcessInfo;
use cirg::scheduled_task_info::ScheduledTaskInfo;
use cirg::security_info::SecurityInfo;
use cirg::service_info::ServiceInfo;
use cirg::software_info::SoftwareInfo;
use cirg::startup_info::StartupInfo;
use cirg::usb_info::UsbInfo;
use cirg::users_groups_info::UsersGroupsInfo;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct AllSystemInfo {
    computer: Option<ComputerInfo>,
    cpu: Option<CpuInfo>,
    gpu: Option<GpuInfo>,
    memory: Option<MemoryInfo>,
    disk: Option<DiskInfo>,
    network: Option<NetworkInfo>,
    monitor: Option<MonitorInfo>,
    audio: Option<AudioInfo>,
    usb: Option<UsbInfo>,
    power: Option<PowerInfo>,
    security: Option<SecurityInfo>,
    process: Option<ProcessInfo>,
    service: Option<ServiceInfo>,
    startup: Option<StartupInfo>,
    software: Option<SoftwareInfo>,
    hotfix: Option<HotfixInfo>,
    users_groups: Option<UsersGroupsInfo>,
    environment: Option<EnvironmentInfo>,
    event_log: Option<EventLogInfo>,
    scheduled_task: Option<ScheduledTaskInfo>,
}

fn unwrap_fetch<T>(result: Result<anyhow::Result<T>, tokio::task::JoinError>) -> Option<T> {
    match result {
        Ok(Ok(data)) => Some(data),
        _ => None,
    }
}

#[tauri::command]
async fn get_all_system_info() -> AllSystemInfo {
    let (
        computer,
        cpu,
        gpu,
        memory,
        disk,
        network,
        monitor,
        audio,
        usb,
        power,
        security,
        process,
        service,
        startup,
        software,
        hotfix,
        users_groups,
        environment,
        event_log,
        scheduled_task,
    ) = tokio::join!(
        tokio::task::spawn_blocking(ComputerInfo::fetch),
        tokio::task::spawn_blocking(CpuInfo::fetch),
        tokio::task::spawn_blocking(GpuInfo::fetch),
        tokio::task::spawn_blocking(MemoryInfo::fetch),
        tokio::task::spawn_blocking(DiskInfo::fetch),
        tokio::task::spawn_blocking(NetworkInfo::fetch),
        tokio::task::spawn_blocking(MonitorInfo::fetch),
        tokio::task::spawn_blocking(AudioInfo::fetch),
        tokio::task::spawn_blocking(UsbInfo::fetch),
        tokio::task::spawn_blocking(PowerInfo::fetch),
        tokio::task::spawn_blocking(SecurityInfo::fetch),
        tokio::task::spawn_blocking(ProcessInfo::fetch),
        tokio::task::spawn_blocking(ServiceInfo::fetch),
        tokio::task::spawn_blocking(StartupInfo::fetch),
        tokio::task::spawn_blocking(SoftwareInfo::fetch),
        tokio::task::spawn_blocking(HotfixInfo::fetch),
        tokio::task::spawn_blocking(UsersGroupsInfo::fetch),
        tokio::task::spawn_blocking(EnvironmentInfo::fetch),
        tokio::task::spawn_blocking(EventLogInfo::fetch),
        tokio::task::spawn_blocking(ScheduledTaskInfo::fetch),
    );

    AllSystemInfo {
        computer: unwrap_fetch(computer),
        cpu: unwrap_fetch(cpu),
        gpu: unwrap_fetch(gpu),
        memory: unwrap_fetch(memory),
        disk: unwrap_fetch(disk),
        network: unwrap_fetch(network),
        monitor: unwrap_fetch(monitor),
        audio: unwrap_fetch(audio),
        usb: unwrap_fetch(usb),
        power: unwrap_fetch(power),
        security: unwrap_fetch(security),
        process: unwrap_fetch(process),
        service: unwrap_fetch(service),
        startup: unwrap_fetch(startup),
        software: unwrap_fetch(software),
        hotfix: unwrap_fetch(hotfix),
        users_groups: unwrap_fetch(users_groups),
        environment: unwrap_fetch(environment),
        event_log: unwrap_fetch(event_log),
        scheduled_task: unwrap_fetch(scheduled_task),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![get_all_system_info,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
