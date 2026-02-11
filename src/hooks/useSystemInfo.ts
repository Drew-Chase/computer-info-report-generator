import {useCallback, useEffect, useRef, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import type {AllSystemInfo} from "../types/system-info";

interface UseSystemInfoOptions {
    intervalMs?: number;
}

export function useSystemInfo({intervalMs = 0}: UseSystemInfoOptions = {}) {
    const [data, setData] = useState<AllSystemInfo>({});
    const [loading, setLoading] = useState(true);
    const [isRefreshing, setIsRefreshing] = useState(false);
    const [progress, setProgress] = useState(0);
    const [refreshCountdown, setRefreshCountdown] = useState(0);
    const [lastRefreshDurationMs, setLastRefreshDurationMs] = useState<number | null>(null);
    const isInitialLoad = useRef(true);
    const lastRefreshTime = useRef(0);

    const fetchAll = useCallback(async () => {
        if (isInitialLoad.current) {
            setLoading(true);
            setProgress(0);
        } else {
            setIsRefreshing(true);
        }

        const start = performance.now();

        // Fake progress for initial load (we can't track sub-progress with a single call)
        let progressTimer: ReturnType<typeof setInterval> | undefined;
        if (isInitialLoad.current) {
            let fakeProgress = 0;
            progressTimer = setInterval(() => {
                fakeProgress = Math.min(fakeProgress + 0.04, 0.95);
                setProgress(fakeProgress);
            }, 100);
        }

        try {
            const result = await invoke<AllSystemInfo>("get_all_system_info");
            setData(result);
        } catch {
            // keep stale data on error
        }

        const elapsed = performance.now() - start;
        setLastRefreshDurationMs(Math.round(elapsed));
        lastRefreshTime.current = Date.now();
        setRefreshCountdown(0);

        if (progressTimer) clearInterval(progressTimer);

        if (isInitialLoad.current) {
            setProgress(1);
            setLoading(false);
            isInitialLoad.current = false;
        } else {
            setIsRefreshing(false);
        }
    }, []);

    useEffect(() => {
        fetchAll();
    }, [fetchAll]);

    useEffect(() => {
        if (intervalMs <= 0) return;
        const id = setInterval(fetchAll, intervalMs);
        return () => clearInterval(id);
    }, [intervalMs, fetchAll]);

    // Countdown progress tracker - updates ~15fps for smooth bar
    useEffect(() => {
        if (intervalMs <= 0) {
            setRefreshCountdown(0);
            return;
        }
        const tick = () => {
            const elapsed = Date.now() - lastRefreshTime.current;
            setRefreshCountdown(Math.min(elapsed / intervalMs, 1));
        };
        const id = setInterval(tick, 64);
        return () => clearInterval(id);
    }, [intervalMs]);

    return {data, loading, isRefreshing, progress, refreshCountdown, lastRefreshDurationMs, refresh: fetchAll};
}
