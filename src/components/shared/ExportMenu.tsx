import {Dropdown, DropdownTrigger, DropdownMenu, DropdownItem, Button} from "@heroui/react";
import {Icon} from "@iconify-icon/react";
import {addToast} from "@heroui/react";
import type {AllSystemInfo} from "../../types/system-info";
import {exportAsHtml} from "../../utils/exportHtml";
import {exportAsMarkdown} from "../../utils/exportMarkdown";
import {saveFile} from "../../utils/saveFile";

interface ExportMenuProps {
    data: AllSystemInfo;
}

export default function ExportMenu({data}: ExportMenuProps) {
    const handleExport = async (format: "html" | "markdown") => {
        try {
            const computerName = data.computer?.name ?? "system-report";
            if (format === "html") {
                const content = exportAsHtml(data);
                const saved = await saveFile({
                    content,
                    defaultName: `${computerName}-report.html`,
                    filterName: "HTML",
                    extensions: ["html"],
                });
                if (saved) addToast({title: "Report exported as HTML", color: "success"});
            } else {
                const content = exportAsMarkdown(data);
                const saved = await saveFile({
                    content,
                    defaultName: `${computerName}-report.md`,
                    filterName: "Markdown",
                    extensions: ["md"],
                });
                if (saved) addToast({title: "Report exported as Markdown", color: "success"});
            }
        } catch (err) {
            addToast({title: "Export failed", description: String(err), color: "danger"});
        }
    };

    return (
        <Dropdown>
            <DropdownTrigger>
                <Button variant="light" className="min-w-0 h-[2rem] text-[1rem]" radius="sm">
                    <Icon icon="material-symbols:download-rounded"/>
                </Button>
            </DropdownTrigger>
            <DropdownMenu aria-label="Export options" onAction={(key) => handleExport(key as "html" | "markdown")}>
                <DropdownItem key="html" startContent={<Icon icon="material-symbols:code-rounded"/>}>
                    Export as HTML
                </DropdownItem>
                <DropdownItem key="markdown" startContent={<Icon icon="material-symbols:markdown"/>}>
                    Export as Markdown
                </DropdownItem>
            </DropdownMenu>
        </Dropdown>
    );
}
