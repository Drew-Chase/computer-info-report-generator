import {Popover, PopoverTrigger, PopoverContent, Button, Slider} from "@heroui/react";
import {Icon} from "@iconify-icon/react";

interface SettingsPopoverProps {
    refreshInterval: number;
    onRefreshIntervalChange: (ms: number) => void;
}

// Slider goes from 0 to 30. 0 = off, 1-30 = seconds.
function msToSlider(ms: number): number {
    if (ms <= 0) return 0;
    return Math.round(ms / 1000);
}

function sliderToMs(val: number): number {
    if (val <= 0) return 0;
    return val * 1000;
}

function formatLabel(val: number): string {
    if (val <= 0) return "Off";
    return `${val}s`;
}

export default function SettingsPopover({refreshInterval, onRefreshIntervalChange}: SettingsPopoverProps) {
    const sliderValue = msToSlider(refreshInterval);

    return (
        <Popover placement="bottom-end">
            <PopoverTrigger>
                <Button variant="light" className="min-w-0 h-[2rem] text-[1rem]" radius="sm">
                    <Icon icon="material-symbols:settings-rounded"/>
                </Button>
            </PopoverTrigger>
            <PopoverContent className="bg-[#1a1a1a] border border-[#262626] py-4 px-5 w-72">
                <div className="flex flex-col gap-3 w-full">
                    <h3 className="text-sm font-semibold text-foreground/80">Settings</h3>
                    <Slider
                        label="Refresh Interval"
                        size="sm"
                        step={1}
                        minValue={0}
                        maxValue={60}
                        value={sliderValue}
                        getValue={(val) => formatLabel(val as number)}
                        onChange={(val) => onRefreshIntervalChange(sliderToMs(val as number))}
                        color="primary"
                        classNames={{
                            label: "text-xs text-foreground/60",
                            value: "text-xs text-foreground/60",
                        }}
                    />
                </div>
            </PopoverContent>
        </Popover>
    );
}
