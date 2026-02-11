import React, {useEffect} from "react";
import {BrowserRouter, Route, Routes, useNavigate} from "react-router-dom";
import ReactDOM from "react-dom/client";

import "./css/index.css";
import Home from "./pages/Home.tsx";
import Navigation from "./components/Navigation.tsx";
import {ThemeProvider} from "./providers/ThemeProvider.tsx";
import {HeroUIProvider, ToastProvider} from "@heroui/react";
import {useSettings} from "./hooks/useSettings.ts";
import {useSystemInfo} from "./hooks/useSystemInfo.ts";


ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <BrowserRouter>
            <ThemeProvider>
                <MainContentRenderer/>
            </ThemeProvider>
        </BrowserRouter>
    </React.StrictMode>
);

export function MainContentRenderer() {
    const navigate = useNavigate();
    const {refreshInterval, setRefreshInterval} = useSettings();
    const {data, loading, isRefreshing, progress, refreshCountdown, lastRefreshDurationMs} = useSystemInfo({intervalMs: refreshInterval});

    useEffect(() => {
        const handleContextMenu = (e: MouseEvent) => e.preventDefault();
        window.addEventListener("contextmenu", handleContextMenu);
        return () => window.removeEventListener("contextmenu", handleContextMenu);
    }, []);

    return (
        <HeroUIProvider navigate={navigate}>

            <ToastProvider
                placement={"bottom-right"}
                toastProps={{
                    shouldShowTimeoutProgress: true,
                    timeout: 3000,
                    variant: "flat"
                }}
            />

            <main className="flex flex-col p-0 m-0 h-screen overflow-hidden">
                <Navigation
                    refreshInterval={refreshInterval}
                    onRefreshIntervalChange={setRefreshInterval}
                    data={data}
                    isRefreshing={isRefreshing}
                    refreshCountdown={refreshCountdown}
                    lastRefreshDurationMs={lastRefreshDurationMs}
                />

                <div className="flex flex-row w-full flex-1 overflow-hidden p-0 m-0">
                    <Routes>
                        <Route>
                            <Route path="/" element={
                                <Home
                                    data={data}
                                    loading={loading}
                                    isRefreshing={isRefreshing}
                                    progress={progress}
                                />
                            }/>
                        </Route>
                    </Routes>
                </div>
            </main>
        </HeroUIProvider>
    );
}
