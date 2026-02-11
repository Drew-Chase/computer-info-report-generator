import {Card, CardBody, CardHeader} from "@heroui/react";
import type {ReactNode} from "react";

interface GlassCardProps {
    children: ReactNode;
    header?: ReactNode;
    className?: string;
    hoverEffect?: boolean;
}

export default function GlassCard({
                                      children,
                                      header,
                                      className = "",
                                      hoverEffect = true,
                                  }: GlassCardProps) {
    return (
        <Card
            className={`card-surface ${hoverEffect ? "transition-colors duration-200 hover:border-[#333]" : ""} ${className}`}
        >
            {header && <CardHeader className="pb-0">{header}</CardHeader>}
            <CardBody>{children}</CardBody>
        </Card>
    );
}
