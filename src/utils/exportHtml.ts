import type {AllSystemInfo} from "../types/system-info";

export function exportAsHtml(data: AllSystemInfo): string {
    const computerName = data.computer?.name ?? "System Report";
    const now = new Date().toLocaleString();

    const section = (title: string, content: string) =>
        `<div class="section"><h2>${title}</h2>${content}</div>`;

    const table = (headers: string[], rows: string[][]) => {
        const ths = headers.map(h => `<th>${h}</th>`).join("");
        const trs = rows.map(r => `<tr>${r.map(c => `<td>${c}</td>`).join("")}</tr>`).join("");
        return `<table><thead><tr>${ths}</tr></thead><tbody>${trs}</tbody></table>`;
    };

    const field = (label: string, value: unknown) =>
        `<div class="field"><span class="label">${label}</span><span class="value">${value ?? "N/A"}</span></div>`;

    let body = "";

    if (data.computer) {
        const c = data.computer;
        body += section("Computer", `
            <div class="fields">
                ${field("Name", c.name)}
                ${field("Domain", c.domain)}
                ${field("Manufacturer", c.manufacturer)}
                ${field("System Type", c.system_type)}
                ${field("OS", `${c.operating_system.name} ${c.operating_system.version}`)}
                ${field("Architecture", c.operating_system.architecture)}
                ${field("Timezone", c.operating_system.timezone)}
                ${field("BIOS", `${c.bios.manufacturer} ${c.bios.version}`)}
            </div>
        `);
    }

    if (data.cpu) {
        const c = data.cpu;
        body += section("CPU", `
            <div class="fields">
                ${field("Name", c.name)}
                ${field("Cores", c.cores)}
                ${field("Logical Processors", c.logical_processors)}
                ${field("Max Clock (MHz)", c.max_clock_mhz)}
                ${field("Socket", c.socket)}
                ${field("Architecture", c.architecture)}
                ${field("L2 Cache (KB)", c.l2_cache_kb)}
                ${field("L3 Cache (KB)", c.l3_cache_kb)}
            </div>
        `);
    }

    if (data.gpu) {
        body += section("GPU", data.gpu.adapters.map(a => `
            <div class="fields">
                ${field("Name", a.name)}
                ${field("Driver", a.driver_version)}
                ${field("VRAM (MB)", a.adapter_ram_mb)}
                ${field("Resolution", a.resolution)}
                ${field("Refresh Rate", `${a.refresh_rate} Hz`)}
            </div>
        `).join("<hr>"));
    }

    if (data.memory) {
        body += section("Memory",
            table(
                ["Bank", "Capacity (GB)", "Speed (MHz)", "Type", "Manufacturer", "Part Number"],
                data.memory.slots.map(s => [s.bank_label, String(s.capacity_gb), String(s.speed_mhz), s.memory_type, s.manufacturer, s.part_number])
            )
        );
    }

    if (data.disk) {
        let diskContent = "";
        if (data.disk.physical_disks.length > 0) {
            diskContent += "<h3>Physical Disks</h3>" + table(
                ["Model", "Interface", "Media Type", "Size (GB)", "Status"],
                data.disk.physical_disks.map(d => [d.model, d.interface_type, d.media_type, d.size_gb.toFixed(1), d.status])
            );
        }
        if (data.disk.logical_disks.length > 0) {
            diskContent += "<h3>Logical Drives</h3>" + table(
                ["Drive", "File System", "Total (GB)", "Free (GB)", "Used (GB)", "Usage %"],
                data.disk.logical_disks.map(d => [d.device_id, d.file_system, d.total_gb.toFixed(1), d.free_gb.toFixed(1), d.used_gb.toFixed(1), d.usage_pct.toFixed(1)])
            );
        }
        body += section("Storage", diskContent);
    }

    if (data.network) {
        body += section("Network", data.network.adapters.map(a => `
            <div class="fields">
                ${field("Name", a.name)}
                ${field("MAC", a.mac_address)}
                ${field("Speed", a.speed)}
                ${field("Gateway", a.gateway)}
                ${field("DHCP", a.dhcp_enabled)}
                ${field("IPv4", a.ipv4_addresses.join(", "))}
                ${field("IPv6", a.ipv6_addresses.join(", "))}
                ${field("DNS", a.dns_servers.join(", "))}
            </div>
        `).join("<hr>"));
    }

    if (data.security) {
        const s = data.security;
        let pendingContent: string;
        if (s.pending_updates === null) {
            pendingContent = "<p>Unable to query Windows Update</p>";
        } else if (s.pending_updates.length === 0) {
            pendingContent = "<p>No pending updates</p>";
        } else {
            pendingContent = table(
                ["Title", "KB", "Severity", "Downloaded", "Mandatory", "Category"],
                s.pending_updates.map(u => [
                    u.title,
                    u.kb_article_ids.join(", ") || "—",
                    u.severity ?? "—",
                    u.is_downloaded ? "Yes" : "No",
                    u.is_mandatory ? "Yes" : "No",
                    u.categories.join(", ") || "—",
                ])
            );
        }

        body += section("Security", `
            <div class="fields">
                ${field("Secure Boot", s.secure_boot)}
                ${field("UAC", s.uac)}
                ${field("BitLocker", s.bit_locker)}
                ${field("RDP Enabled", s.rdp_enabled)}
                ${field("Antivirus", s.antivirus ?? "Not detected")}
            </div>
            <h3>Pending Updates</h3>
            ${pendingContent}
        `);
    }

    if (data.software) {
        body += section("Installed Software",
            table(
                ["Name", "Version", "Publisher", "Install Date"],
                data.software.programs.map(p => [p.name, p.version, p.publisher, p.install_date])
            )
        );
    }

    return `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>${computerName} - CIRG Report</title>
<style>
* { margin: 0; padding: 0; box-sizing: border-box; }
body { font-family: 'Segoe UI', Inter, sans-serif; background: #111; color: #f5f5f5; padding: 2rem; line-height: 1.6; }
h1 { font-size: 1.5rem; font-weight: 600; margin-bottom: 0.25rem; }
.meta { color: #888; font-size: 0.85rem; margin-bottom: 2rem; }
.section { margin-bottom: 2rem; background: #1a1a1a; border: 1px solid #262626; border-radius: 0.75rem; padding: 1.5rem; }
.section h2 { font-size: 1.1rem; font-weight: 600; margin-bottom: 1rem; color: #60a5fa; }
.section h3 { font-size: 0.95rem; font-weight: 600; margin: 1rem 0 0.5rem; color: #888; }
.fields { display: grid; grid-template-columns: repeat(auto-fill, minmax(250px, 1fr)); gap: 0.75rem; }
.field { display: flex; flex-direction: column; gap: 0.15rem; }
.label { font-size: 0.7rem; text-transform: uppercase; letter-spacing: 0.05em; color: #666; font-weight: 500; }
.value { font-size: 0.9rem; color: #ddd; }
table { width: 100%; border-collapse: collapse; font-size: 0.85rem; }
th { text-align: left; padding: 0.5rem; background: #1f1f1f; color: #888; font-size: 0.7rem; text-transform: uppercase; letter-spacing: 0.05em; font-weight: 500; }
td { padding: 0.5rem; border-bottom: 1px solid #262626; color: #ccc; }
hr { border: none; border-top: 1px solid #262626; margin: 1rem 0; }
</style>
</head>
<body>
<h1>${computerName}</h1>
<p class="meta">Generated by CIRG on ${now}</p>
${body}
</body>
</html>`;
}
