import SectionHeader from "../shared/SectionHeader";
import GlassCard from "../shared/GlassCard";
import DataField from "../shared/DataField";
import {Progress} from "@heroui/react";
import type {AllSystemInfo} from "../../types/system-info";

interface Props {
    data: AllSystemInfo;
}

export default function StorageSection({data}: Props) {
    const {disk} = data;

    if (!disk) return null;

    return (
        <div>
            <SectionHeader icon="material-symbols:storage" title="Storage"/>

            {disk.physical_disks.length > 0 && (
                <div className="mb-6">
                    <h3 className="text-sm font-semibold text-foreground/60 mb-3">Physical Disks</h3>
                    <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
                        {disk.physical_disks.map((d, i) => (
                            <GlassCard key={i}>
                                <h4 className="text-sm font-semibold text-foreground/80 mb-3">{d.model}</h4>
                                <div className="grid grid-cols-2 gap-3">
                                    <DataField label="Interface" value={d.interface_type}/>
                                    <DataField label="Media Type" value={d.media_type}/>
                                    <DataField label="Size" value={`${d.size_gb.toFixed(1)} GB`}/>
                                    <DataField label="Status" value={d.status}/>
                                </div>
                            </GlassCard>
                        ))}
                    </div>
                </div>
            )}

            {disk.logical_disks.length > 0 && (
                <div>
                    <h3 className="text-sm font-semibold text-foreground/60 mb-3">Logical Drives</h3>
                    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                        {disk.logical_disks.map((d, i) => (
                            <GlassCard key={i}>
                                <div className="flex items-baseline gap-2 mb-3">
                                    <span className="text-lg font-semibold text-foreground/90">{d.device_id}</span>
                                    {d.volume_name && (
                                        <span className="text-foreground/50 text-sm">{d.volume_name}</span>
                                    )}
                                </div>
                                <div className="grid grid-cols-2 gap-2 mb-3">
                                    <DataField label="File System" value={d.file_system}/>
                                    <DataField label="Total" value={`${d.total_gb.toFixed(1)} GB`}/>
                                    <DataField label="Free" value={`${d.free_gb.toFixed(1)} GB`}/>
                                    <DataField label="Used" value={`${d.used_gb.toFixed(1)} GB`}/>
                                </div>
                                <div className="flex justify-between text-xs text-foreground/40 mb-1">
                                    <span>Usage</span>
                                    <span>{d.usage_pct.toFixed(1)}%</span>
                                </div>
                                <Progress
                                    value={d.usage_pct}
                                    color={d.usage_pct > 90 ? "danger" : d.usage_pct > 75 ? "warning" : "primary"}
                                    size="sm"
                                />
                            </GlassCard>
                        ))}
                    </div>
                </div>
            )}
        </div>
    );
}
