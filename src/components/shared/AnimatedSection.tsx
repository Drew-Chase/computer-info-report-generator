import {motion, useInView} from "framer-motion";
import {useRef, type ReactNode} from "react";

interface AnimatedSectionProps {
    id: string;
    children: ReactNode;
    direction?: "up" | "left" | "right";
    delay?: number;
    className?: string;
}

export default function AnimatedSection({id, children, direction = "up", delay = 0, className = ""}: AnimatedSectionProps) {
    const ref = useRef<HTMLDivElement>(null);
    const isInView = useInView(ref, {once: true, margin: "-100px"});

    const variants = {
        hidden: {
            opacity: 0,
            x: direction === "left" ? -20 : direction === "right" ? 20 : 0,
            y: direction === "up" ? 20 : 0,
        },
        visible: {
            opacity: 1,
            x: 0,
            y: 0,
        },
    };

    return (
        <motion.section
            id={id}
            ref={ref}
            variants={variants}
            initial="hidden"
            animate={isInView ? "visible" : "hidden"}
            transition={{duration: 0.4, delay, ease: "easeOut"}}
            className={className}
        >
            {children}
        </motion.section>
    );
}
