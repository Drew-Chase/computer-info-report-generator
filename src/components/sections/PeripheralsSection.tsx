import SectionHeader from "../shared/SectionHeader";
import GlassCard from "../shared/GlassCard";
import DataField from "../shared/DataField";
import {Table, TableHeader, TableColumn, TableBody, TableRow, TableCell} from "@heroui/react";
import type {AllSystemInfo} from "../../types/system-info";

interface Props {
    data: AllSystemInfo;
}

export default function PeripheralsSection({data}: Props) {
    const {monitor, audio, usb} = data;

    return (
        <div>
            <SectionHeader icon="material-symbols:devices-rounded" title="Peripherals"/>

            {monitor && monitor.monitors.length > 0 && (
                <div className="mb-6">
                    <h3 className="text-sm font-semibold text-foreground/60 mb-3">Monitors</h3>
                    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                        {monitor.monitors.map((m, i) => (
                            <GlassCard key={i}>
                                <h4 className="text-sm font-semibold text-foreground/80 mb-3">{m.name || "Monitor"}</h4>
                                <div className="grid grid-cols-2 gap-3">
                                    <DataField label="Manufacturer" value={m.manufacturer}/>
                                    <DataField label="Year" value={m.year_of_manufacture}/>
                                    <DataField label="Serial" value={m.serial_number}/>
                                </div>
                            </GlassCard>
                        ))}
                    </div>
                </div>
            )}

            {audio && audio.devices.length > 0 && (
                <div className="mb-6">
                    <h3 className="text-sm font-semibold text-foreground/60 mb-3">Audio Devices</h3>
                    <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                        {audio.devices.map((d, i) => (
                            <GlassCard key={i}>
                                <h4 className="text-sm font-semibold text-foreground/80 mb-3">{d.name}</h4>
                                <div className="grid grid-cols-2 gap-3">
                                    <DataField label="Manufacturer" value={d.manufacturer}/>
                                    <DataField label="Status" value={d.status}/>
                                </div>
                            </GlassCard>
                        ))}
                    </div>
                </div>
            )}

            {usb && usb.devices.length > 0 && (
                <GlassCard>
                    <h3 className="text-sm font-semibold text-foreground/60 mb-3">USB Devices</h3>
                    <Table aria-label="USB devices" removeWrapper classNames={{
                        th: "bg-[#1f1f1f] text-foreground/60 text-xs font-medium uppercase tracking-wide",
                        td: "text-sm text-foreground/70",
                    }}>
                        <TableHeader>
                            <TableColumn>Name</TableColumn>
                            <TableColumn>Manufacturer</TableColumn>
                            <TableColumn>Status</TableColumn>
                        </TableHeader>
                        <TableBody>
                            {usb.devices.map((d, i) => (
                                <TableRow key={i}>
                                    <TableCell>{d.name}</TableCell>
                                    <TableCell>{d.manufacturer}</TableCell>
                                    <TableCell>{d.status}</TableCell>
                                </TableRow>
                            ))}
                        </TableBody>
                    </Table>
                </GlassCard>
            )}
        </div>
    );
}
