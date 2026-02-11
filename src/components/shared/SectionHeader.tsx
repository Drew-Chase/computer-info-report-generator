import {Icon} from "@iconify-icon/react";

interface SectionHeaderProps {
    icon: string;
    title: string;
}

export default function SectionHeader({icon, title}: SectionHeaderProps) {
    return (
        <div className="mb-4">
            <div className="flex items-center gap-3 mb-2">
                <Icon icon={icon} className="text-primary text-xl"/>
                <h2 className="text-lg font-semibold text-foreground/90">{title}</h2>
            </div>
            <div className="divider-subtle"/>
        </div>
    );
}
