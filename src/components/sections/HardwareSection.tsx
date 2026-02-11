import SectionHeader from "../shared/SectionHeader";
import GlassCard from "../shared/GlassCard";
import DataField from "../shared/DataField";
import {Table, TableHeader, TableColumn, TableBody, TableRow, TableCell} from "@heroui/react";
import type {AllSystemInfo} from "../../types/system-info";

interface Props {
    data: AllSystemInfo;
}

export default function HardwareSection({data}: Props) {
    const {cpu, gpu, memory} = data;

    return (
        <div>
            <SectionHeader icon="material-symbols:memory-rounded" title="Hardware"/>

            {cpu && (
                <GlassCard className="mb-6">
                    <h3 className="text-sm font-semibold text-foreground/60 mb-3">CPU</h3>
                    <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4">
                        <DataField label="Name" value={cpu.name}/>
                        <DataField label="Cores" value={cpu.cores}/>
                        <DataField label="Logical Processors" value={cpu.logical_processors}/>
                        <DataField label="Max Clock (MHz)" value={cpu.max_clock_mhz}/>
                        <DataField label="Current Clock (MHz)" value={cpu.current_clock_mhz}/>
                        <DataField label="Socket" value={cpu.socket}/>
                        <DataField label="L2 Cache (KB)" value={cpu.l2_cache_kb}/>
                        <DataField label="L3 Cache (KB)" value={cpu.l3_cache_kb}/>
                        <DataField label="Architecture" value={cpu.architecture}/>
                        <DataField label="Virtualization" value={cpu.virtualization}/>
                        <DataField label="Status" value={cpu.status}/>
                        <DataField label="Load %" value={cpu.load_pct}/>
                    </div>
                </GlassCard>
            )}

            {gpu && gpu.adapters.length > 0 && (
                <div className="mb-6">
                    <h3 className="text-sm font-semibold text-foreground/60 mb-3">GPU Adapters</h3>
                    <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
                        {gpu.adapters.map((adapter, i) => (
                            <GlassCard key={i}>
                                <h4 className="text-sm font-semibold text-foreground/80 mb-3">{adapter.name}</h4>
                                <div className="grid grid-cols-2 gap-3">
                                    <DataField label="Driver Version" value={adapter.driver_version}/>
                                    <DataField label="Driver Date" value={adapter.driver_date}/>
                                    <DataField label="VRAM (MB)" value={adapter.adapter_ram_mb}/>
                                    <DataField label="Resolution" value={adapter.resolution}/>
                                    <DataField label="Refresh Rate" value={`${adapter.refresh_rate} Hz`}/>
                                    <DataField label="Status" value={adapter.status}/>
                                </div>
                            </GlassCard>
                        ))}
                    </div>
                </div>
            )}

            {memory && memory.slots.length > 0 && (
                <GlassCard>
                    <h3 className="text-sm font-semibold text-foreground/60 mb-3">
                        Memory ({memory.slots.length}/{memory.total_slots} slots, max {memory.max_capacity_gb} GB)
                    </h3>
                    <Table aria-label="Memory slots" removeWrapper classNames={{
                        th: "bg-[#1f1f1f] text-foreground/60 text-xs font-medium uppercase tracking-wide",
                        td: "text-sm text-foreground/70",
                    }}>
                        <TableHeader>
                            <TableColumn>Bank</TableColumn>
                            <TableColumn>Capacity</TableColumn>
                            <TableColumn>Speed</TableColumn>
                            <TableColumn>Type</TableColumn>
                            <TableColumn>Manufacturer</TableColumn>
                            <TableColumn>Part Number</TableColumn>
                        </TableHeader>
                        <TableBody>
                            {memory.slots.map((slot, i) => (
                                <TableRow key={i}>
                                    <TableCell>{slot.bank_label}</TableCell>
                                    <TableCell>{slot.capacity_gb} GB</TableCell>
                                    <TableCell>{slot.speed_mhz} MHz</TableCell>
                                    <TableCell>{slot.memory_type}</TableCell>
                                    <TableCell>{slot.manufacturer}</TableCell>
                                    <TableCell>{slot.part_number}</TableCell>
                                </TableRow>
                            ))}
                        </TableBody>
                    </Table>
                </GlassCard>
            )}
        </div>
    );
}
