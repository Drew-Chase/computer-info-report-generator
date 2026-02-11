import {useState, useMemo} from "react";
import SectionHeader from "../shared/SectionHeader";
import GlassCard from "../shared/GlassCard";
import {
    Table, TableHeader, TableColumn, TableBody, TableRow, TableCell,
    Tabs, Tab, Input, Pagination,
} from "@heroui/react";
import {Icon} from "@iconify-icon/react";
import type {AllSystemInfo} from "../../types/system-info";

interface Props {
    data: AllSystemInfo;
}

const PAGE_SIZE = 25;

export default function SoftwareSection({data}: Props) {
    const {software, hotfix, startup, scheduled_task} = data;
    const [search, setSearch] = useState("");
    const [page, setPage] = useState(1);

    const filteredPrograms = useMemo(() => {
        if (!software) return [];
        if (!search) return software.programs;
        const q = search.toLowerCase();
        return software.programs.filter(
            (p) => p.name.toLowerCase().includes(q) || p.publisher.toLowerCase().includes(q)
        );
    }, [software, search]);

    const totalPages = Math.max(1, Math.ceil(filteredPrograms.length / PAGE_SIZE));
    const pagedPrograms = filteredPrograms.slice((page - 1) * PAGE_SIZE, page * PAGE_SIZE);

    return (
        <div>
            <SectionHeader icon="material-symbols:deployed-code" title="Software"/>
            <GlassCard>
                <Tabs aria-label="Software tabs" color="primary" variant="underlined" classNames={{
                    tabList: "gap-4",
                    tab: "text-xs font-medium uppercase tracking-wide",
                }}>
                    <Tab key="installed" title={`Installed (${software?.programs.length ?? 0})`}>
                        <div className="mb-3">
                            <Input
                                placeholder="Search software..."
                                size="sm"
                                variant="bordered"
                                value={search}
                                onValueChange={(v) => {
                                    setSearch(v);
                                    setPage(1);
                                }}
                                startContent={<Icon icon="material-symbols:search" className="text-foreground/40"/>}
                                className="max-w-xs"
                            />
                        </div>
                        <Table aria-label="Installed software" removeWrapper classNames={{
                            th: "bg-[#1f1f1f] text-foreground/60 text-xs font-medium uppercase tracking-wide",
                            td: "text-sm text-foreground/70",
                        }}>
                            <TableHeader>
                                <TableColumn>Name</TableColumn>
                                <TableColumn>Version</TableColumn>
                                <TableColumn>Publisher</TableColumn>
                                <TableColumn>Install Date</TableColumn>
                            </TableHeader>
                            <TableBody>
                                {pagedPrograms.map((p, i) => (
                                    <TableRow key={i}>
                                        <TableCell>{p.name}</TableCell>
                                        <TableCell>{p.version}</TableCell>
                                        <TableCell>{p.publisher}</TableCell>
                                        <TableCell>{p.install_date}</TableCell>
                                    </TableRow>
                                ))}
                            </TableBody>
                        </Table>
                        {totalPages > 1 && (
                            <div className="flex justify-center mt-3">
                                <Pagination total={totalPages} page={page} onChange={setPage} color="primary"
                                            size="sm"/>
                            </div>
                        )}
                    </Tab>

                    <Tab key="hotfixes" title={`Hotfixes (${hotfix?.hotfixes.length ?? 0})`}>
                        <Table aria-label="Hotfixes" removeWrapper classNames={{
                            th: "bg-[#1f1f1f] text-foreground/60 text-xs font-medium uppercase tracking-wide",
                            td: "text-sm text-foreground/70",
                        }}>
                            <TableHeader>
                                <TableColumn>ID</TableColumn>
                                <TableColumn>Description</TableColumn>
                                <TableColumn>Installed By</TableColumn>
                                <TableColumn>Installed On</TableColumn>
                            </TableHeader>
                            <TableBody>
                                {(hotfix?.hotfixes ?? []).map((h, i) => (
                                    <TableRow key={i}>
                                        <TableCell>{h.hotfix_id}</TableCell>
                                        <TableCell>{h.description}</TableCell>
                                        <TableCell>{h.installed_by}</TableCell>
                                        <TableCell>{h.installed_on}</TableCell>
                                    </TableRow>
                                ))}
                            </TableBody>
                        </Table>
                    </Tab>

                    <Tab key="startup" title={`Startup (${startup?.items.length ?? 0})`}>
                        <Table aria-label="Startup items" removeWrapper classNames={{
                            th: "bg-[#1f1f1f] text-foreground/60 text-xs font-medium uppercase tracking-wide",
                            td: "text-sm text-foreground/70",
                        }}>
                            <TableHeader>
                                <TableColumn>Name</TableColumn>
                                <TableColumn>Command</TableColumn>
                                <TableColumn>Location</TableColumn>
                                <TableColumn>User</TableColumn>
                            </TableHeader>
                            <TableBody>
                                {(startup?.items ?? []).map((s, i) => (
                                    <TableRow key={i}>
                                        <TableCell>{s.name}</TableCell>
                                        <TableCell className="max-w-xs truncate">{s.command}</TableCell>
                                        <TableCell>{s.location}</TableCell>
                                        <TableCell>{s.user}</TableCell>
                                    </TableRow>
                                ))}
                            </TableBody>
                        </Table>
                    </Tab>

                    <Tab key="tasks" title={`Tasks (${scheduled_task?.tasks.length ?? 0})`}>
                        <Table aria-label="Scheduled tasks" removeWrapper classNames={{
                            th: "bg-[#1f1f1f] text-foreground/60 text-xs font-medium uppercase tracking-wide",
                            td: "text-sm text-foreground/70",
                        }}>
                            <TableHeader>
                                <TableColumn>Name</TableColumn>
                                <TableColumn>State</TableColumn>
                                <TableColumn>Last Run</TableColumn>
                                <TableColumn>Next Run</TableColumn>
                                <TableColumn>Author</TableColumn>
                            </TableHeader>
                            <TableBody>
                                {(scheduled_task?.tasks ?? []).map((t, i) => (
                                    <TableRow key={i}>
                                        <TableCell>{t.name}</TableCell>
                                        <TableCell>{t.state}</TableCell>
                                        <TableCell>{t.last_run}</TableCell>
                                        <TableCell>{t.next_run}</TableCell>
                                        <TableCell>{t.author}</TableCell>
                                    </TableRow>
                                ))}
                            </TableBody>
                        </Table>
                    </Tab>
                </Tabs>
            </GlassCard>
        </div>
    );
}
