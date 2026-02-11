import {Button, Tooltip} from "@heroui/react";
import {Icon} from "@iconify-icon/react";

interface SidebarSection {
    id: string;
    label: string;
    icon: string;
}

const sections: SidebarSection[] = [
    {id: "overview", label: "Overview", icon: "material-symbols:dashboard-rounded"},
    {id: "hardware", label: "Hardware", icon: "material-symbols:memory-rounded"},
    {id: "storage", label: "Storage", icon: "material-symbols:storage"},
    {id: "network", label: "Network", icon: "material-symbols:lan-rounded"},
    {id: "peripherals", label: "Peripherals", icon: "material-symbols:devices-rounded"},
    {id: "security", label: "Security", icon: "material-symbols:shield-rounded"},
    {id: "software", label: "Software", icon: "material-symbols:deployed-code"},
    {id: "system", label: "System", icon: "material-symbols:settings-rounded"},
];

interface SidebarProps {
    activeSection: string;
    onNavigate: (id: string) => void;
}

export default function Sidebar({activeSection, onNavigate}: SidebarProps) {
    return (
        <aside className="fixed left-3 top-[4.5rem] bottom-3 w-14 lg:w-52 bg-[#1a1a1a] border border-[#262626] rounded-xl z-[50] flex flex-col py-3 px-2 gap-1 overflow-y-auto">
            {sections.map((section) => {
                const isActive = activeSection === section.id;
                return (
                    <Tooltip key={section.id} content={section.label} placement="right" className="lg:hidden">
                        <Button
                            variant="light"
                            className={`justify-start min-w-0 w-full h-10 px-3 rounded-lg ${
                                isActive
                                    ? "text-foreground bg-[#262626]"
                                    : "text-foreground/50 hover:text-foreground/80"
                            }`}
                            onPress={() => onNavigate(section.id)}
                        >
                            <Icon icon={section.icon} className="text-lg shrink-0"/>
                            <span className="hidden lg:inline ml-3 text-sm">{section.label}</span>
                        </Button>
                    </Tooltip>
                );
            })}
        </aside>
    );
}
