import SectionHeader from "../shared/SectionHeader";
import GlassCard from "../shared/GlassCard";
import DataField from "../shared/DataField";
import {Progress} from "@heroui/react";
import type {AllSystemInfo} from "../../types/system-info";

interface Props {
    data: AllSystemInfo;
}

function formatUptime(seconds: number): string {
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    return `${days}d ${hours}h ${mins}m`;
}

export default function OverviewSection({data}: Props) {
    const {computer, cpu, memory, disk} = data;

    const totalMemoryGb = memory?.slots.reduce((sum, s) => sum + s.capacity_gb, 0) ?? 0;
    const totalDiskGb = disk?.logical_disks.reduce((sum, d) => sum + d.total_gb, 0) ?? 0;
    const usedDiskGb = disk?.logical_disks.reduce((sum, d) => sum + d.used_gb, 0) ?? 0;
    const diskUsagePct = totalDiskGb > 0 ? (usedDiskGb / totalDiskGb) * 100 : 0;

    return (
        <div>
            <SectionHeader icon="material-symbols:dashboard-rounded" title="System Overview"/>

            <GlassCard className="mb-6">
                <div className="flex flex-col gap-1">
                    <h3 className="text-xl font-semibold text-foreground/90">
                        {computer?.name ?? "Loading..."}
                    </h3>
                    <p className="text-foreground/60 text-sm">
                        {computer?.operating_system.name} {computer?.operating_system.version}
                    </p>
                    <p className="text-foreground/40 text-xs">
                        {computer?.manufacturer} - {computer?.system_type}
                    </p>
                    {computer && (
                        <p className="text-foreground/40 text-xs mt-1">
                            Uptime: {formatUptime(computer.operating_system.uptime)}
                        </p>
                    )}
                </div>
            </GlassCard>

            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                <GlassCard>
                    <DataField label="Processor" value={cpu?.name}/>
                    <div className="mt-3">
                        <div className="flex justify-between text-xs text-foreground/40 mb-1">
                            <span>CPU Load</span>
                            <span>{cpu?.load_pct ?? 0}%</span>
                        </div>
                        <Progress value={cpu?.load_pct ?? 0} color="primary" size="sm"/>
                    </div>
                </GlassCard>

                <GlassCard>
                    <DataField label="Memory" value={totalMemoryGb > 0 ? `${totalMemoryGb} GB` : undefined}/>
                    <div className="mt-3">
                        <div className="flex justify-between text-xs text-foreground/40 mb-1">
                            <span>Slots Used</span>
                            <span>{memory?.slots.length ?? 0} / {memory?.total_slots ?? 0}</span>
                        </div>
                        <Progress
                            value={memory ? (memory.slots.length / memory.total_slots) * 100 : 0}
                            color="primary" size="sm"
                        />
                    </div>
                </GlassCard>

                <GlassCard>
                    <DataField label="Storage" value={totalDiskGb > 0 ? `${totalDiskGb.toFixed(1)} GB` : undefined}/>
                    <div className="mt-3">
                        <div className="flex justify-between text-xs text-foreground/40 mb-1">
                            <span>Used</span>
                            <span>{diskUsagePct.toFixed(1)}%</span>
                        </div>
                        <Progress value={diskUsagePct} color={diskUsagePct > 90 ? "danger" : "primary"} size="sm"/>
                    </div>
                </GlassCard>

                <GlassCard>
                    <DataField label="Cores / Threads"
                               value={cpu ? `${cpu.cores}C / ${cpu.logical_processors}T` : undefined}/>
                    <div className="mt-3">
                        <DataField label="Clock Speed"
                                   value={cpu ? `${cpu.current_clock_mhz} / ${cpu.max_clock_mhz} MHz` : undefined}/>
                    </div>
                </GlassCard>
            </div>
        </div>
    );
}
