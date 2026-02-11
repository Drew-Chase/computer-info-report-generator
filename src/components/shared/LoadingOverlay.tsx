import {Progress} from "@heroui/react";
import {motion, AnimatePresence} from "framer-motion";

interface LoadingOverlayProps {
    visible: boolean;
    progress: number;
}

export default function LoadingOverlay({visible, progress}: LoadingOverlayProps) {
    return (
        <AnimatePresence>
            {visible && (
                <motion.div
                    className="fixed inset-0 z-[100] flex flex-col items-center justify-center bg-background"
                    initial={{opacity: 1}}
                    exit={{opacity: 0}}
                    transition={{duration: 0.6}}
                >
                    <motion.h1
                        className="text-4xl font-semibold tracking-[0.3em] text-primary mb-4"
                        animate={{opacity: [0.5, 1, 0.5]}}
                        transition={{duration: 2, repeat: Infinity, ease: "easeInOut"}}
                    >
                        CIRG
                    </motion.h1>
                    <p className="text-foreground/40 text-sm tracking-wide mb-8">
                        Scanning System...
                    </p>
                    <Progress
                        value={progress * 100}
                        className="max-w-xs"
                        color="primary"
                        size="sm"
                        aria-label="Loading system information"
                    />
                    <p className="text-foreground/30 text-xs mt-3">
                        {Math.round(progress * 100)}%
                    </p>
                </motion.div>
            )}
        </AnimatePresence>
    );
}
