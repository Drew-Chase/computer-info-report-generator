import {Chip} from "@heroui/react";

type StatusType = "ok" | "warning" | "error" | "info" | "disabled";

const colorMap: Record<StatusType, "success" | "warning" | "danger" | "primary" | "default"> = {
    ok: "success",
    warning: "warning",
    error: "danger",
    info: "primary",
    disabled: "default",
};

interface StatusChipProps {
    status: StatusType;
    label: string;
}

export default function StatusChip({status, label}: StatusChipProps) {
    return (
        <Chip color={colorMap[status]} variant="flat" size="sm">
            {label}
        </Chip>
    );
}
