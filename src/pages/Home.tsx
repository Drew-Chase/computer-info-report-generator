import {useCallback, useEffect, useRef, useState} from "react";
import LoadingOverlay from "../components/shared/LoadingOverlay";
import AnimatedSection from "../components/shared/AnimatedSection";
import Sidebar from "../components/sidebar/Sidebar";
import OverviewSection from "../components/sections/OverviewSection";
import HardwareSection from "../components/sections/HardwareSection";
import StorageSection from "../components/sections/StorageSection";
import NetworkSection from "../components/sections/NetworkSection";
import PeripheralsSection from "../components/sections/PeripheralsSection";
import SecuritySection from "../components/sections/SecuritySection";
import SoftwareSection from "../components/sections/SoftwareSection";
import SystemSection from "../components/sections/SystemSection";
import type {AllSystemInfo} from "../types/system-info";

const SECTION_IDS = ["overview", "hardware", "storage", "network", "peripherals", "security", "software", "system"];

interface HomeProps {
    data: AllSystemInfo;
    loading: boolean;
    isRefreshing: boolean;
    progress: number;
}

export default function Home({data, loading, progress}: HomeProps) {
    const [activeSection, setActiveSection] = useState("overview");
    const contentRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const container = contentRef.current;
        if (!container) return;

        const observer = new IntersectionObserver(
            (entries) => {
                for (const entry of entries) {
                    if (entry.isIntersecting) {
                        setActiveSection(entry.target.id);
                    }
                }
            },
            {root: container, rootMargin: "-20% 0px -60% 0px", threshold: 0}
        );

        SECTION_IDS.forEach((id) => {
            const el = document.getElementById(id);
            if (el) observer.observe(el);
        });

        return () => observer.disconnect();
    }, [loading]);

    const handleNavigate = useCallback((id: string) => {
        const el = document.getElementById(id);
        el?.scrollIntoView({behavior: "smooth", block: "start"});
    }, []);

    return (
        <>
            <LoadingOverlay visible={loading} progress={progress}/>

            <div className="flex flex-row w-full h-full">
                <Sidebar activeSection={activeSection} onNavigate={handleNavigate}/>

                <div
                    ref={contentRef}
                    className="ml-[4.75rem] lg:ml-[14.5rem] flex-1 overflow-y-auto p-6 space-y-12"
                    style={{maxHeight: "calc(100vh - 3.5rem)"}}
                >
                    <AnimatedSection id="overview">
                        <OverviewSection data={data}/>
                    </AnimatedSection>

                    <AnimatedSection id="hardware" direction="left" delay={0.1}>
                        <HardwareSection data={data}/>
                    </AnimatedSection>

                    <AnimatedSection id="storage" direction="right" delay={0.1}>
                        <StorageSection data={data}/>
                    </AnimatedSection>

                    <AnimatedSection id="network" delay={0.1}>
                        <NetworkSection data={data}/>
                    </AnimatedSection>

                    <AnimatedSection id="peripherals" direction="left" delay={0.1}>
                        <PeripheralsSection data={data}/>
                    </AnimatedSection>

                    <AnimatedSection id="security" direction="right" delay={0.1}>
                        <SecuritySection data={data}/>
                    </AnimatedSection>

                    <AnimatedSection id="software" delay={0.1}>
                        <SoftwareSection data={data}/>
                    </AnimatedSection>

                    <AnimatedSection id="system" direction="left" delay={0.1}>
                        <SystemSection data={data}/>
                    </AnimatedSection>

                    <div className="h-8"/>
                </div>
            </div>
        </>
    );
}
