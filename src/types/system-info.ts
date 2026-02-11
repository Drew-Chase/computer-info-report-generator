export interface OSInfo {
    name: string;
    version: string;
    build_lab: string;
    architecture: string;
    install_date: string;
    last_boot_date: string;
    uptime: number;
    timezone: string;
}

export interface BIOSInfo {
    manufacturer: string;
    version: string;
    release_date: string;
}

export interface ComputerInfo {
    name: string;
    domain: string;
    manufacturer: string;
    system_type: string;
    operating_system: OSInfo;
    bios: BIOSInfo;
}

export interface CpuInfo {
    name: string;
    cores: number;
    logical_processors: number;
    max_clock_mhz: number;
    current_clock_mhz: number;
    socket: string;
    l2_cache_kb: number;
    l3_cache_kb: number;
    architecture: string;
    virtualization: boolean;
    status: string;
    load_pct: number;
}

export interface GpuAdapter {
    name: string;
    driver_version: string;
    driver_date: string;
    adapter_ram_mb: number;
    resolution: string;
    refresh_rate: number;
    status: string;
    availability: string;
}

export interface GpuInfo {
    adapters: GpuAdapter[];
}

export interface MemorySlot {
    bank_label: string;
    capacity_gb: number;
    speed_mhz: number;
    memory_type: string;
    form_factor: string;
    manufacturer: string;
    part_number: string;
}

export interface MemoryInfo {
    slots: MemorySlot[];
    total_slots: number;
    max_capacity_gb: number;
}

export interface PhysicalDisk {
    model: string;
    interface_type: string;
    media_type: string;
    size_gb: number;
    status: string;
}

export interface LogicalDisk {
    device_id: string;
    volume_name: string;
    file_system: string;
    total_gb: number;
    free_gb: number;
    used_gb: number;
    usage_pct: number;
}

export interface DiskInfo {
    physical_disks: PhysicalDisk[];
    logical_disks: LogicalDisk[];
}

export interface NetworkAdapter {
    name: string;
    description: string;
    mac_address: string;
    speed: string;
    ipv4_addresses: string[];
    ipv6_addresses: string[];
    dns_servers: string[];
    dhcp_enabled: boolean;
    gateway: string;
}

export interface NetworkInfo {
    adapters: NetworkAdapter[];
}

export interface Monitor {
    manufacturer: string;
    name: string;
    serial_number: string;
    year_of_manufacture: number;
    resolution: string;
    refresh_rate: number;
}

export interface MonitorInfo {
    monitors: Monitor[];
}

export interface AudioDevice {
    name: string;
    manufacturer: string;
    status: string;
    device_id: string;
}

export interface AudioInfo {
    devices: AudioDevice[];
}

export interface UsbDevice {
    name: string;
    device_id: string;
    manufacturer: string;
    status: string;
}

export interface UsbInfo {
    devices: UsbDevice[];
}

export interface BatteryInfo {
    name: string;
    status: string;
    charge_pct: string;
    run_time_mins: string;
    design_capacity: string;
    full_charge_capacity: string;
    chemistry: string;
}

export interface PowerInfo {
    plan: string;
    battery: BatteryInfo | null;
}

export interface TpmInfo {
    present: boolean;
    ready: boolean;
    enabled: boolean;
    activated: boolean;
    version: string;
    manufacturer: string;
}

export interface FirewallInfo {
    domain_enabled: boolean | null;
    domain_inbound: string | null;
    domain_outbound: string | null;
    private_enabled: boolean | null;
    private_inbound: string | null;
    private_outbound: string | null;
    public_enabled: boolean | null;
    public_inbound: string | null;
    public_outbound: string | null;
}

export interface SecurityInfo {
    secure_boot: boolean;
    tpm: TpmInfo | null;
    antivirus: string | null;
    firewall: FirewallInfo | null;
    uac: boolean;
    rdp_enabled: boolean;
    bit_locker: boolean;
    pending_updates: string;
}

export interface ProcessEntry {
    name: string;
    pid: number;
    cpu_seconds: number;
    memory_mb: number;
    exe_path: string;
    command: string;
}

export interface ProcessInfo {
    processes: ProcessEntry[];
}

export interface Service {
    name: string;
    display_name: string;
    state: string;
    start_mode: string;
    account: string;
    path: string;
    description: string;
}

export interface ServiceInfo {
    services: Service[];
}

export interface InstalledProgram {
    name: string;
    version: string;
    publisher: string;
    install_date: string;
}

export interface SoftwareInfo {
    programs: InstalledProgram[];
}

export interface Hotfix {
    hotfix_id: string;
    description: string;
    installed_by: string;
    installed_on: string;
}

export interface HotfixInfo {
    hotfixes: Hotfix[];
}

export interface StartupItem {
    name: string;
    command: string;
    location: string;
    user: string;
}

export interface StartupInfo {
    items: StartupItem[];
}

export interface ScheduledTask {
    name: string;
    path: string;
    state: string;
    last_run: string;
    next_run: string;
    result: string;
    author: string;
}

export interface ScheduledTaskInfo {
    tasks: ScheduledTask[];
}

export interface LocalUser {
    name: string;
    disabled: boolean;
    description: string;
}

export interface LocalGroup {
    name: string;
    description: string;
    members: string[];
}

export interface UsersGroupsInfo {
    users: LocalUser[];
    groups: LocalGroup[];
}

export interface EnvironmentInfo {
    variables: Record<string, string>;
}

export interface EventEntry {
    level: string;
    source: string;
    event_id: string;
    time_created: string;
    message: string;
}

export interface EventLogInfo {
    system_events: EventEntry[];
    application_events: EventEntry[];
}

export interface AllSystemInfo {
    computer?: ComputerInfo;
    cpu?: CpuInfo;
    gpu?: GpuInfo;
    memory?: MemoryInfo;
    disk?: DiskInfo;
    network?: NetworkInfo;
    monitor?: MonitorInfo;
    audio?: AudioInfo;
    usb?: UsbInfo;
    power?: PowerInfo;
    security?: SecurityInfo;
    process?: ProcessInfo;
    service?: ServiceInfo;
    startup?: StartupInfo;
    software?: SoftwareInfo;
    hotfix?: HotfixInfo;
    users_groups?: UsersGroupsInfo;
    environment?: EnvironmentInfo;
    event_log?: EventLogInfo;
    scheduled_task?: ScheduledTaskInfo;
}
