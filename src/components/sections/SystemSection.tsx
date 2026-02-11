import {useState, useMemo} from "react";
import SectionHeader from "../shared/SectionHeader";
import GlassCard from "../shared/GlassCard";
import DataField from "../shared/DataField";
import {
    Table, TableHeader, TableColumn, TableBody, TableRow, TableCell,
    Tabs, Tab, Input, Pagination, Chip,
} from "@heroui/react";
import {Icon} from "@iconify-icon/react";
import type {AllSystemInfo} from "../../types/system-info";

interface Props {
    data: AllSystemInfo;
}

const PAGE_SIZE = 25;

export default function SystemSection({data}: Props) {
    const {service, process, users_groups, environment, event_log} = data;

    const [serviceSearch, setServiceSearch] = useState("");
    const [servicePage, setServicePage] = useState(1);
    const [processSearch, setProcessSearch] = useState("");
    const [processPage, setProcessPage] = useState(1);

    const filteredServices = useMemo(() => {
        if (!service) return [];
        if (!serviceSearch) return service.services;
        const q = serviceSearch.toLowerCase();
        return service.services.filter(
            (s) => s.display_name.toLowerCase().includes(q) || s.name.toLowerCase().includes(q)
        );
    }, [service, serviceSearch]);

    const serviceTotalPages = Math.max(1, Math.ceil(filteredServices.length / PAGE_SIZE));
    const pagedServices = filteredServices.slice((servicePage - 1) * PAGE_SIZE, servicePage * PAGE_SIZE);

    const filteredProcesses = useMemo(() => {
        if (!process) return [];
        if (!processSearch) return process.processes;
        const q = processSearch.toLowerCase();
        return process.processes.filter((p) => p.name.toLowerCase().includes(q));
    }, [process, processSearch]);

    const processTotalPages = Math.max(1, Math.ceil(filteredProcesses.length / PAGE_SIZE));
    const pagedProcesses = filteredProcesses.slice((processPage - 1) * PAGE_SIZE, processPage * PAGE_SIZE);

    return (
        <div>
            <SectionHeader icon="material-symbols:settings-rounded" title="System"/>
            <GlassCard>
                <Tabs aria-label="System tabs" color="primary" variant="underlined" classNames={{
                    tabList: "gap-4",
                    tab: "text-xs font-medium uppercase tracking-wide",
                }}>
                    <Tab key="services" title={`Services (${service?.services.length ?? 0})`}>
                        <div className="mb-3">
                            <Input
                                placeholder="Search services..."
                                size="sm"
                                variant="bordered"
                                value={serviceSearch}
                                onValueChange={(v) => {
                                    setServiceSearch(v);
                                    setServicePage(1);
                                }}
                                startContent={<Icon icon="material-symbols:search" className="text-foreground/40"/>}
                                className="max-w-xs"
                            />
                        </div>
                        <Table aria-label="Services" removeWrapper classNames={{
                            th: "bg-[#1f1f1f] text-foreground/60 text-xs font-medium uppercase tracking-wide",
                            td: "text-sm text-foreground/70",
                        }}>
                            <TableHeader>
                                <TableColumn>Name</TableColumn>
                                <TableColumn>Display Name</TableColumn>
                                <TableColumn>State</TableColumn>
                                <TableColumn>Start Mode</TableColumn>
                            </TableHeader>
                            <TableBody>
                                {pagedServices.map((s, i) => (
                                    <TableRow key={i}>
                                        <TableCell>{s.name}</TableCell>
                                        <TableCell>{s.display_name}</TableCell>
                                        <TableCell>
                                            <Chip size="sm" variant="flat"
                                                  color={s.state === "Running" ? "success" : "default"}>
                                                {s.state}
                                            </Chip>
                                        </TableCell>
                                        <TableCell>{s.start_mode}</TableCell>
                                    </TableRow>
                                ))}
                            </TableBody>
                        </Table>
                        {serviceTotalPages > 1 && (
                            <div className="flex justify-center mt-3">
                                <Pagination total={serviceTotalPages} page={servicePage} onChange={setServicePage}
                                            color="primary" size="sm"/>
                            </div>
                        )}
                    </Tab>

                    <Tab key="processes" title={`Processes (${process?.processes.length ?? 0})`}>
                        <div className="mb-3">
                            <Input
                                placeholder="Search processes..."
                                size="sm"
                                variant="bordered"
                                value={processSearch}
                                onValueChange={(v) => {
                                    setProcessSearch(v);
                                    setProcessPage(1);
                                }}
                                startContent={<Icon icon="material-symbols:search" className="text-foreground/40"/>}
                                className="max-w-xs"
                            />
                        </div>
                        <Table aria-label="Processes" removeWrapper classNames={{
                            th: "bg-[#1f1f1f] text-foreground/60 text-xs font-medium uppercase tracking-wide",
                            td: "text-sm text-foreground/70",
                        }}>
                            <TableHeader>
                                <TableColumn>Name</TableColumn>
                                <TableColumn>PID</TableColumn>
                                <TableColumn>CPU (s)</TableColumn>
                                <TableColumn>Memory (MB)</TableColumn>
                            </TableHeader>
                            <TableBody>
                                {pagedProcesses.map((p, i) => (
                                    <TableRow key={i}>
                                        <TableCell>{p.name}</TableCell>
                                        <TableCell>{p.pid}</TableCell>
                                        <TableCell>{p.cpu_seconds}</TableCell>
                                        <TableCell>{p.memory_mb.toFixed(1)}</TableCell>
                                    </TableRow>
                                ))}
                            </TableBody>
                        </Table>
                        {processTotalPages > 1 && (
                            <div className="flex justify-center mt-3">
                                <Pagination total={processTotalPages} page={processPage} onChange={setProcessPage}
                                            color="primary" size="sm"/>
                            </div>
                        )}
                    </Tab>

                    <Tab key="users" title="Users & Groups">
                        {users_groups && (
                            <div className="space-y-4">
                                <div>
                                    <h4 className="text-xs font-semibold text-foreground/60 mb-2">Local
                                        Users</h4>
                                    <Table aria-label="Users" removeWrapper classNames={{
                                        th: "bg-[#1f1f1f] text-foreground/60 text-xs font-medium uppercase tracking-wide",
                                        td: "text-sm text-foreground/70",
                                    }}>
                                        <TableHeader>
                                            <TableColumn>Name</TableColumn>
                                            <TableColumn>Disabled</TableColumn>
                                            <TableColumn>Description</TableColumn>
                                        </TableHeader>
                                        <TableBody>
                                            {users_groups.users.map((u, i) => (
                                                <TableRow key={i}>
                                                    <TableCell>{u.name}</TableCell>
                                                    <TableCell>
                                                        <Chip size="sm" variant="flat"
                                                              color={u.disabled ? "danger" : "success"}>
                                                            {u.disabled ? "Yes" : "No"}
                                                        </Chip>
                                                    </TableCell>
                                                    <TableCell>{u.description}</TableCell>
                                                </TableRow>
                                            ))}
                                        </TableBody>
                                    </Table>
                                </div>
                                <div>
                                    <h4 className="text-xs font-semibold text-foreground/60 mb-2">Local
                                        Groups</h4>
                                    <Table aria-label="Groups" removeWrapper classNames={{
                                        th: "bg-[#1f1f1f] text-foreground/60 text-xs font-medium uppercase tracking-wide",
                                        td: "text-sm text-foreground/70",
                                    }}>
                                        <TableHeader>
                                            <TableColumn>Name</TableColumn>
                                            <TableColumn>Description</TableColumn>
                                            <TableColumn>Members</TableColumn>
                                        </TableHeader>
                                        <TableBody>
                                            {users_groups.groups.map((g, i) => (
                                                <TableRow key={i}>
                                                    <TableCell>{g.name}</TableCell>
                                                    <TableCell>{g.description}</TableCell>
                                                    <TableCell>
                                                        <div className="flex flex-wrap gap-1">
                                                            {g.members.map((m, j) => (
                                                                <Chip key={j} size="sm"
                                                                      variant="flat">{m}</Chip>
                                                            ))}
                                                        </div>
                                                    </TableCell>
                                                </TableRow>
                                            ))}
                                        </TableBody>
                                    </Table>
                                </div>
                            </div>
                        )}
                    </Tab>

                    <Tab key="env" title="Environment">
                        {environment && (
                            <div className="grid grid-cols-1 gap-2 max-h-[400px] overflow-y-auto">
                                {Object.entries(environment.variables).map(([key, value]) => (
                                    <div key={key} className="flex gap-2 py-1 border-b border-foreground/5">
                                        <DataField label={key} value={value}/>
                                    </div>
                                ))}
                            </div>
                        )}
                    </Tab>

                    <Tab key="events" title="Event Logs">
                        {event_log && (
                            <Tabs aria-label="Event log tabs" size="sm" variant="bordered" classNames={{
                                tab: "text-xs",
                            }}>
                                <Tab key="system"
                                     title={`System (${event_log.system_events.length})`}>
                                    <Table aria-label="System events" removeWrapper classNames={{
                                        th: "bg-[#1f1f1f] text-foreground/60 text-xs font-medium uppercase tracking-wide",
                                        td: "text-sm text-foreground/70",
                                    }}>
                                        <TableHeader>
                                            <TableColumn>Level</TableColumn>
                                            <TableColumn>Source</TableColumn>
                                            <TableColumn>Event ID</TableColumn>
                                            <TableColumn>Time</TableColumn>
                                            <TableColumn>Message</TableColumn>
                                        </TableHeader>
                                        <TableBody>
                                            {event_log.system_events.map((e, i) => (
                                                <TableRow key={i}>
                                                    <TableCell>
                                                        <Chip size="sm" variant="flat"
                                                              color={e.level === "Error" ? "danger" : e.level === "Warning" ? "warning" : "default"}>
                                                            {e.level}
                                                        </Chip>
                                                    </TableCell>
                                                    <TableCell>{e.source}</TableCell>
                                                    <TableCell>{e.event_id}</TableCell>
                                                    <TableCell className="whitespace-nowrap">{e.time_created}</TableCell>
                                                    <TableCell
                                                        className="max-w-xs truncate">{e.message}</TableCell>
                                                </TableRow>
                                            ))}
                                        </TableBody>
                                    </Table>
                                </Tab>
                                <Tab key="application"
                                     title={`Application (${event_log.application_events.length})`}>
                                    <Table aria-label="Application events" removeWrapper classNames={{
                                        th: "bg-[#1f1f1f] text-foreground/60 text-xs font-medium uppercase tracking-wide",
                                        td: "text-sm text-foreground/70",
                                    }}>
                                        <TableHeader>
                                            <TableColumn>Level</TableColumn>
                                            <TableColumn>Source</TableColumn>
                                            <TableColumn>Event ID</TableColumn>
                                            <TableColumn>Time</TableColumn>
                                            <TableColumn>Message</TableColumn>
                                        </TableHeader>
                                        <TableBody>
                                            {event_log.application_events.map((e, i) => (
                                                <TableRow key={i}>
                                                    <TableCell>
                                                        <Chip size="sm" variant="flat"
                                                              color={e.level === "Error" ? "danger" : e.level === "Warning" ? "warning" : "default"}>
                                                            {e.level}
                                                        </Chip>
                                                    </TableCell>
                                                    <TableCell>{e.source}</TableCell>
                                                    <TableCell>{e.event_id}</TableCell>
                                                    <TableCell className="whitespace-nowrap">{e.time_created}</TableCell>
                                                    <TableCell
                                                        className="max-w-xs truncate">{e.message}</TableCell>
                                                </TableRow>
                                            ))}
                                        </TableBody>
                                    </Table>
                                </Tab>
                            </Tabs>
                        )}
                    </Tab>
                </Tabs>
            </GlassCard>
        </div>
    );
}
