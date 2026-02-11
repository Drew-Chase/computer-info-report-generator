import {Icon} from "@iconify-icon/react";

interface DataFieldProps {
    label: string;
    value: string | number | boolean | null | undefined;
}

export default function DataField({label, value}: DataFieldProps) {
    const renderValue = () => {
        if (value === null || value === undefined) return <span className="text-foreground/30">N/A</span>;
        if (typeof value === "boolean") {
            return value
                ? <Icon icon="material-symbols:check-circle" className="text-success text-lg"/>
                : <Icon icon="material-symbols:cancel" className="text-danger text-lg"/>;
        }
        return <span className="text-foreground/80">{String(value)}</span>;
    };

    return (
        <div className="flex flex-col gap-0.5">
            <span className="text-xs font-medium uppercase tracking-wide text-foreground/40">{label}</span>
            <div className="text-sm">{renderValue()}</div>
        </div>
    );
}
