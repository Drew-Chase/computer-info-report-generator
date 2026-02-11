import SectionHeader from "../shared/SectionHeader";
import GlassCard from "../shared/GlassCard";
import DataField from "../shared/DataField";
import {Chip} from "@heroui/react";
import type {AllSystemInfo} from "../../types/system-info";

interface Props {
    data: AllSystemInfo;
}

export default function NetworkSection({data}: Props) {
    const {network} = data;

    if (!network) return null;

    return (
        <div>
            <SectionHeader icon="material-symbols:lan-rounded" title="Network"/>
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
                {network.adapters.map((adapter, i) => (
                    <GlassCard key={i}>
                        <h4 className="text-sm font-semibold text-foreground/80 mb-1">{adapter.name}</h4>
                        <p className="text-xs text-foreground/40 mb-3">{adapter.description}</p>

                        <div className="grid grid-cols-2 gap-3 mb-3">
                            <DataField label="MAC Address" value={adapter.mac_address}/>
                            <DataField label="Speed" value={adapter.speed}/>
                            <DataField label="Gateway" value={adapter.gateway}/>
                            <DataField label="DHCP" value={adapter.dhcp_enabled}/>
                        </div>

                        {adapter.ipv4_addresses.length > 0 && (
                            <div className="mb-2">
                                <span className="text-xs font-medium uppercase tracking-wide text-foreground/40">IPv4</span>
                                <div className="flex flex-wrap gap-1 mt-1">
                                    {adapter.ipv4_addresses.map((ip, j) => (
                                        <Chip key={j} size="sm" variant="flat" className="bg-primary/15 text-primary">{ip}</Chip>
                                    ))}
                                </div>
                            </div>
                        )}

                        {adapter.ipv6_addresses.length > 0 && (
                            <div className="mb-2">
                                <span className="text-xs font-medium uppercase tracking-wide text-foreground/40">IPv6</span>
                                <div className="flex flex-wrap gap-1 mt-1">
                                    {adapter.ipv6_addresses.map((ip, j) => (
                                        <Chip key={j} size="sm" variant="flat"
                                              className="text-[0.6rem]">{ip}</Chip>
                                    ))}
                                </div>
                            </div>
                        )}

                        {adapter.dns_servers.length > 0 && (
                            <div>
                                <span className="text-xs font-medium uppercase tracking-wide text-foreground/40">DNS Servers</span>
                                <div className="flex flex-wrap gap-1 mt-1">
                                    {adapter.dns_servers.map((dns, j) => (
                                        <Chip key={j} size="sm" variant="flat" className="bg-foreground/10 text-foreground/70">{dns}</Chip>
                                    ))}
                                </div>
                            </div>
                        )}
                    </GlassCard>
                ))}
            </div>
        </div>
    );
}
