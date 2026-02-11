import SectionHeader from "../shared/SectionHeader";
import GlassCard from "../shared/GlassCard";
import DataField from "../shared/DataField";
import StatusChip from "../shared/StatusChip";
import {Chip, Table, TableHeader, TableColumn, TableBody, TableRow, TableCell} from "@heroui/react";
import type {AllSystemInfo} from "../../types/system-info";

interface Props {
    data: AllSystemInfo;
}

export default function SecuritySection({data}: Props) {
    const {security, power} = data;

    return (
        <div>
            <SectionHeader icon="material-symbols:shield-rounded" title="Security & Power"/>

            {security && (
                <>
                    <GlassCard className="mb-6">
                        <h3 className="text-sm font-semibold text-foreground/60 mb-3">Security Status</h3>
                        <div className="flex flex-wrap gap-2 mb-4">
                            <StatusChip status={security.secure_boot ? "ok" : "warning"}
                                        label={`Secure Boot: ${security.secure_boot ? "ON" : "OFF"}`}/>
                            <StatusChip status={security.uac ? "ok" : "warning"}
                                        label={`UAC: ${security.uac ? "ON" : "OFF"}`}/>
                            <StatusChip status={security.bit_locker ? "ok" : "info"}
                                        label={`BitLocker: ${security.bit_locker ? "ON" : "OFF"}`}/>
                            <StatusChip status={security.rdp_enabled ? "warning" : "ok"}
                                        label={`RDP: ${security.rdp_enabled ? "Enabled" : "Disabled"}`}/>
                        </div>

                        <div className="grid grid-cols-2 sm:grid-cols-3 gap-3">
                            <DataField label="Antivirus" value={security.antivirus ?? "Not detected"}/>
                        </div>
                    </GlassCard>

                    <GlassCard className="mb-6">
                        <h3 className="text-sm font-semibold text-foreground/60 mb-3">Pending Updates</h3>
                        {security.pending_updates === null ? (
                            <p className="text-sm text-foreground/50">Unable to query Windows Update</p>
                        ) : security.pending_updates.length === 0 ? (
                            <Chip color="success" variant="flat" size="sm">No pending updates</Chip>
                        ) : (
                            <Table aria-label="Pending Windows updates" removeWrapper classNames={{
                                th: "bg-[#1f1f1f] text-foreground/60 text-xs font-medium uppercase tracking-wide",
                                td: "text-sm text-foreground/70",
                            }}>
                                <TableHeader>
                                    <TableColumn>Title</TableColumn>
                                    <TableColumn>KB</TableColumn>
                                    <TableColumn>Severity</TableColumn>
                                    <TableColumn>Downloaded</TableColumn>
                                    <TableColumn>Category</TableColumn>
                                </TableHeader>
                                <TableBody>
                                    {security.pending_updates.map((u, i) => (
                                        <TableRow key={i}>
                                            <TableCell>{u.title}</TableCell>
                                            <TableCell>{u.kb_article_ids.join(", ") || "—"}</TableCell>
                                            <TableCell>
                                                {u.severity ? (
                                                    <Chip
                                                        color={u.severity === "Critical" ? "danger" : u.severity === "Important" ? "warning" : "default"}
                                                        variant="flat" size="sm"
                                                    >
                                                        {u.severity}
                                                    </Chip>
                                                ) : (
                                                    <span className="text-foreground/40">—</span>
                                                )}
                                            </TableCell>
                                            <TableCell>
                                                <Chip color={u.is_downloaded ? "success" : "default"} variant="flat" size="sm">
                                                    {u.is_downloaded ? "Yes" : "No"}
                                                </Chip>
                                            </TableCell>
                                            <TableCell>
                                                <div className="flex flex-wrap gap-1">
                                                    {u.categories.length > 0
                                                        ? u.categories.map((cat, ci) => (
                                                            <Chip key={ci} variant="flat" size="sm">{cat}</Chip>
                                                        ))
                                                        : <span className="text-foreground/40">—</span>
                                                    }
                                                </div>
                                            </TableCell>
                                        </TableRow>
                                    ))}
                                </TableBody>
                            </Table>
                        )}
                    </GlassCard>

                    {security.tpm && (
                        <GlassCard className="mb-6">
                            <h3 className="text-sm font-semibold text-foreground/60 mb-3">TPM</h3>
                            <div className="grid grid-cols-2 sm:grid-cols-3 gap-3">
                                <DataField label="Present" value={security.tpm.present}/>
                                <DataField label="Ready" value={security.tpm.ready}/>
                                <DataField label="Enabled" value={security.tpm.enabled}/>
                                <DataField label="Activated" value={security.tpm.activated}/>
                                <DataField label="Version" value={security.tpm.version}/>
                                <DataField label="Manufacturer" value={security.tpm.manufacturer}/>
                            </div>
                        </GlassCard>
                    )}

                    {security.firewall && (
                        <GlassCard className="mb-6">
                            <h3 className="text-sm font-semibold text-foreground/60 mb-3">Firewall</h3>
                            <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
                                <div>
                                    <p className="text-xs font-semibold text-foreground/60 mb-2">Domain</p>
                                    <DataField label="Enabled" value={security.firewall.domain_enabled}/>
                                    <DataField label="Inbound" value={security.firewall.domain_inbound}/>
                                    <DataField label="Outbound" value={security.firewall.domain_outbound}/>
                                </div>
                                <div>
                                    <p className="text-xs font-semibold text-foreground/60 mb-2">Private</p>
                                    <DataField label="Enabled" value={security.firewall.private_enabled}/>
                                    <DataField label="Inbound" value={security.firewall.private_inbound}/>
                                    <DataField label="Outbound" value={security.firewall.private_outbound}/>
                                </div>
                                <div>
                                    <p className="text-xs font-semibold text-foreground/60 mb-2">Public</p>
                                    <DataField label="Enabled" value={security.firewall.public_enabled}/>
                                    <DataField label="Inbound" value={security.firewall.public_inbound}/>
                                    <DataField label="Outbound" value={security.firewall.public_outbound}/>
                                </div>
                            </div>
                        </GlassCard>
                    )}
                </>
            )}

            {power && (
                <GlassCard>
                    <h3 className="text-sm font-semibold text-foreground/60 mb-3">Power</h3>
                    <DataField label="Power Plan" value={power.plan}/>
                    {power.battery && (
                        <div className="grid grid-cols-2 sm:grid-cols-3 gap-3 mt-3">
                            <DataField label="Battery" value={power.battery.name}/>
                            <DataField label="Status" value={power.battery.status}/>
                            <DataField label="Charge" value={power.battery.charge_pct}/>
                            <DataField label="Runtime (min)" value={power.battery.run_time_mins}/>
                            <DataField label="Chemistry" value={power.battery.chemistry}/>
                        </div>
                    )}
                </GlassCard>
            )}
        </div>
    );
}
