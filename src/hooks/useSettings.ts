import {useState, useCallback, useEffect} from "react";

const REFRESH_KEY = "cirg-refresh-interval";

export function useSettings() {
    const [refreshInterval, setRefreshIntervalState] = useState<number>(() => {
        const saved = localStorage.getItem(REFRESH_KEY);
        return saved ? parseInt(saved, 10) : 0;
    });

    const setRefreshInterval = useCallback((ms: number) => {
        setRefreshIntervalState(ms);
        localStorage.setItem(REFRESH_KEY, String(ms));
    }, []);

    useEffect(() => {
        const handler = (e: StorageEvent) => {
            if (e.key === REFRESH_KEY) {
                setRefreshIntervalState(e.newValue ? parseInt(e.newValue, 10) : 0);
            }
        };
        window.addEventListener("storage", handler);
        return () => window.removeEventListener("storage", handler);
    }, []);

    return {refreshInterval, setRefreshInterval};
}
