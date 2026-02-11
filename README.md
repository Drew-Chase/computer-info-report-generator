# Computer Info Report Generator (CIRG)

A comprehensive Windows system information diagnostic tool built with Tauri, React, and Rust. Collects detailed hardware, software, network, and security data and presents it in a modern glassmorphism UI with export capabilities.

## Features

- **20+ data categories** — CPU, GPU, memory, disks, monitors, audio, USB, network adapters, security (TPM, firewall, antivirus), installed software, running processes, Windows services, startup items, scheduled tasks, hotfixes, event logs, environment variables, users & groups, power plans, and more
- **Real-time refresh** with configurable intervals
- **Export** to HTML or Markdown
- **CLI tool** (`cirg-cli`) for headless JSON output, useful for scripting and automation
- **Concurrent data collection** — spawns 20 async tasks in parallel for fast results
- **Custom window chrome** with dark/light theme support

## Tech Stack

| Layer             | Technology                                   |
|-------------------|----------------------------------------------|
| Desktop framework | [Tauri 2](https://v2.tauri.app/)             |
| Frontend          | React 18, TypeScript, Vite, Tailwind CSS     |
| UI components     | [HeroUI](https://heroui.com/), Framer Motion |
| Backend           | Rust (edition 2024)                          |
| System queries    | WMI, Windows Registry, Win32 API, sysinfo    |

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) 18+
- [pnpm](https://pnpm.io/) 8.13.1+
- Windows 10/11 (WMI and Win32 APIs are Windows-only)
- [UPX](https://upx.github.io/) (optional, for compressed release builds)

## Getting Started

```bash
# Install frontend dependencies
pnpm install

# Run in development mode (Tauri + Vite dev server)
pnpm tauri-dev

# Build for production
pnpm tauri-build
```

The production build outputs a bundled installer and a UPX-compressed standalone binary at `target/cirg.exe`.

### CLI Only

```bash
cargo run -p cirg-cli
```

Outputs all system information as pretty-printed JSON to the console.

## Project Structure

```
computer-info-report-generator/
├── src/                        # React frontend
│   ├── pages/                  # Home dashboard
│   ├── components/
│   │   ├── sections/           # 8 data sections (Hardware, Storage, Network, etc.)
│   │   └── shared/             # Reusable UI components (GlassCard, DataField, etc.)
│   ├── hooks/                  # useSystemInfo, useSettings
│   ├── types/                  # TypeScript interfaces for all system info
│   └── utils/                  # HTML/Markdown export
├── src-tauri/                  # Tauri app (Rust binary + IPC commands)
├── crates/
│   ├── cirg/                   # Core system info library (21 modules)
│   └── cirg-cli/               # Standalone CLI tool
├── Cargo.toml                  # Workspace config
└── package.json
```

### Data Modules (`crates/cirg/src/`)

Each module implements the `ComputerInfoExt` trait and queries Windows via WMI, the registry, or Win32 APIs:

| Module                | Data Collected                                         |
|-----------------------|--------------------------------------------------------|
| `cpu_info`            | Name, cores, clocks, cache, architecture, load %       |
| `gpu_info`            | Adapters, VRAM (registry-based for >4 GB), driver info |
| `memory_info`         | Slots, capacity, speed, type (DDR4/DDR5), manufacturer |
| `disk_info`           | Physical disks (SSD/HDD detection), logical volumes    |
| `monitor_info`        | Manufacturer, model, serial, resolution, refresh rate  |
| `network_info`        | Adapters, IPs, MAC addresses, speed, status            |
| `audio_info`          | Audio devices                                          |
| `usb_info`            | Connected USB devices                                  |
| `security_info`       | TPM, Secure Boot, firewall, antivirus                  |
| `software_info`       | Installed programs                                     |
| `service_info`        | Windows services                                       |
| `process_info`        | Top 30 running processes                               |
| `power_info`          | Power plan, battery status                             |
| `computer_info`       | OS version, BIOS, system model                         |
| `hotfix_info`         | Installed Windows updates                              |
| `startup_info`        | Startup programs                                       |
| `scheduled_task_info` | Scheduled tasks                                        |
| `event_log_info`      | Recent Windows event log entries                       |
| `environment_info`    | Environment variables                                  |
| `users_groups_info`   | Local users and groups                                 |

## License

[GPL-3.0-or-later](https://www.gnu.org/licenses/gpl-3.0.html)
