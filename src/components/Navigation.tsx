import {useEffect, useRef, useState} from "react";
import {Button, ButtonGroup} from "@heroui/react";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {Icon} from "@iconify-icon/react";
import SettingsPopover from "./shared/SettingsPopover";
import ExportMenu from "./shared/ExportMenu";
import type {AllSystemInfo} from "../types/system-info";

interface NavigationProps {
    refreshInterval: number;
    onRefreshIntervalChange: (ms: number) => void;
    data: AllSystemInfo;
    isRefreshing?: boolean;
    refreshCountdown: number;
    lastRefreshDurationMs: number | null;
}

export default function Navigation({refreshInterval, onRefreshIntervalChange, data, isRefreshing, refreshCountdown, lastRefreshDurationMs}: NavigationProps) {
    const appWindow = getCurrentWindow();
    const showCountdown = refreshInterval > 0;

    const [visibleDuration, setVisibleDuration] = useState<number | null>(null);
    const hideTimer = useRef<ReturnType<typeof setTimeout>>();

    useEffect(() => {
        if (lastRefreshDurationMs === null) return;
        setVisibleDuration(lastRefreshDurationMs);
        clearTimeout(hideTimer.current);
        hideTimer.current = setTimeout(() => setVisibleDuration(null), 3000);
        return () => clearTimeout(hideTimer.current);
    }, [lastRefreshDurationMs]);

    return (
        <div className="flex flex-col mx-3 mt-2 z-[60] select-none">
            <div
                className="flex flex-row h-[3rem] rounded-xl bg-[#1a1a1a] border border-[#262626] sticky top-0 w-full items-center px-4"
                data-tauri-drag-region=""
            >
                <div className="flex flex-row items-center gap-2" data-tauri-drag-region="">
                    <p className="text-sm font-semibold tracking-wide text-foreground/80 select-none"
                       data-tauri-drag-region="">CIRG</p>
                    {isRefreshing && (
                        <Icon icon="material-symbols:sync-rounded" className="text-primary text-sm animate-spin"/>
                    )}
                    {!isRefreshing && visibleDuration !== null && (
                        <span className="text-xs text-foreground/40 tabular-nums">
                            {visibleDuration >= 1000
                                ? `${(visibleDuration / 1000).toFixed(1)}s`
                                : `${visibleDuration}ms`}
                        </span>
                    )}
                </div>
                <div className="flex flex-row ml-auto items-center">
                    <ButtonGroup className="h-[2rem]">
                        <ExportMenu data={data}/>
                        <SettingsPopover
                            refreshInterval={refreshInterval}
                            onRefreshIntervalChange={onRefreshIntervalChange}
                        />
                        <Button variant="light" className="min-w-0 h-[2rem] text-[1rem]" radius="sm"
                                onPress={() => appWindow.minimize()}>
                            <Icon icon="material-symbols:minimize-rounded"/>
                        </Button>
                        <Button variant="light" className="min-w-0 h-[2rem] text-[.7rem]" radius="sm"
                                onPress={() => appWindow.toggleMaximize()}>
                            <Icon icon="material-symbols:square-outline-rounded"/>
                        </Button>
                        <Button variant="light" color="danger" className="min-w-0 h-[2rem] text-[1rem]" radius="sm"
                                onPress={() => appWindow.close()}>
                            <Icon icon="material-symbols:close-rounded"/>
                        </Button>
                    </ButtonGroup>
                </div>
            </div>
            {showCountdown && (
                <div className="w-full h-[2px] bg-[#1a1a1a] rounded-b-xl overflow-hidden">
                    <div
                        className="h-full bg-primary/40 transition-none"
                        style={{width: `${refreshCountdown * 100}%`}}
                    />
                </div>
            )}
        </div>
    );
}
